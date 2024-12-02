mod iso8853;

use std::collections::HashMap;

use crate::{operation::Operation, GatewayError, Result};

use iso8853::*;
pub type BitMap = HashMap<usize, BitField>;
pub type OperationParseResult = Result<Option<String>>;
pub type OperationParser = fn(&Operation) -> OperationParseResult;

pub enum MessagingSpecification {
    Iso8853,
    Apacs,
}

impl MessagingSpecification {
    pub fn encode_request(&self, op: &Operation) -> Result<String> {
        self.encode_using_template(&op, &self.get_template())
    }

    pub fn get_template(&self) -> BitMap {
        match self {
            MessagingSpecification::Iso8853 => ISO8853_BITMAP_TEMPLATE.clone(),
            MessagingSpecification::Apacs => todo!(),
        }
    }

    fn get_formatter(&self) -> StringFormatter {
        match self {
            MessagingSpecification::Iso8853 => iso8853_string_field,
            MessagingSpecification::Apacs => todo!(),
        }
    }

    pub fn encode_using_template(
        &self,
        op: &Operation,
        template: &BitMap,
    ) -> Result<String> {
        let formatter = Some(self.get_formatter());
        encode(&op, &template, formatter, formatter)
    }
}

#[derive(Debug, Clone)]
pub enum BitField {
    Single {
        parser: OperationParser,
        min_length: usize,
        max_length: usize,
        padding_char: Option<char>,
    },
    Map(BitMap),
}

impl<T: Into<BitField>> From<HashMap<usize, T>> for BitField {
    fn from(value: HashMap<usize, T>) -> Self {
        BitField::Map(value.into_iter().map(|(i, x)| (i, x.into())).collect())
    }
}

macro_rules! bitmap {
    ($name:ident, $($key:expr => $value:expr),+ $(,)?) => {
        pub const $name: std::sync::LazyLock<crate::messaging_specification::BitMap> = std::sync::LazyLock::new(|| ::std::collections::HashMap::from([ $(($key, crate::messaging_specification::BitField::from($value))),* ]));
    };
}
pub(crate) use bitmap;

fn encode(
    op: &Operation,
    template: &BitMap,
    single_field_transform: Option<StringFormatter>,
    map_field_transform: Option<StringFormatter>,
) -> Result<String> {
    let mut output = String::new();
    _format(
        &op,
        &template,
        &mut output,
        single_field_transform,
        map_field_transform,
    )?;
    Ok(output)
}

struct EncodingContext {
    position: Option<usize>,
    padding: Option<(usize, char)>,
}
type StringFormatter =
    fn(data: &mut String, encoding_ctx: EncodingContext, field_template: &BitField);

fn _format(
    op: &Operation,
    template: &BitMap,
    output: &mut String,
    single_field_transform: Option<StringFormatter>,
    map_field_transform: Option<StringFormatter>,
) -> Result<()> {
    let mut sorted: Vec<(&usize, &BitField)> = template.iter().collect();
    sorted.sort_by(|a, b| a.0.cmp(b.0));
    for (pos, field) in sorted.into_iter() {
        match field {
            BitField::Single {
                parser,
                padding_char,
                min_length,
                max_length,
            } => {
                if let Some(mut data) = parser(&op)? {
                    if data.len() > *max_length {
                        return Err(GatewayError::EncodingError(format!("value '{data}' too long ({}) for bitfield '{pos}' ({})", data.len(), *max_length)));
                    }
                    if let Some(transformer) = single_field_transform {
                        let ctx = EncodingContext {
                            position: Some(*pos),
                            padding: padding_char.map(|ch| (*min_length, ch)),
                        };
                        transformer(&mut data, ctx, field);
                    }
                    output.push_str(&data);
                }
            }
            BitField::Map(map) => {
                let mut sorted_nested: Vec<(&usize, &BitField)> = map.iter().collect();
                sorted_nested.sort_by(|a, b| a.0.cmp(b.0));
                let mut nested = String::new();
                for (nested_pos, nested_field) in sorted_nested.into_iter() {
                    match nested_field {
                        BitField::Single {
                            parser,
                            padding_char,
                            min_length,
                            max_length,
                        } => {
                            if let Some(mut nested_data) = parser(&op)? {
                                if nested_data.len() > *max_length {
                                    return Err(GatewayError::EncodingError(format!("value '{nested_data}' too long ({}) for bitfield '{pos}.{nested_pos}' ({})", nested_data.len(), *max_length)));
                                }
                                if let Some(transformer) = single_field_transform {
                                    let ctx = EncodingContext {
                                        position: Some(*nested_pos),
                                        padding: padding_char.map(|ch| (*min_length, ch)),
                                    };
                                    transformer(&mut nested_data, ctx, nested_field);
                                }
                                nested.push_str(&nested_data);
                            }
                        }
                        BitField::Map(_map) => panic!("cannot handle more than 1 nested map"), // TODO will this ever be needed?
                    }
                }
                if let Some(transformer) = map_field_transform {
                    let ctx = EncodingContext {
                        position: Some(*pos),
                        padding: None,
                    };
                    transformer(&mut nested, ctx, field);
                }
                output.push_str(&nested);
            }
        }
    }
    Ok(())
}

fn string_field(mut data: &mut String, ctx: EncodingContext, _field: &BitField) {
    if let Some((length, pad)) = ctx.padding {
        pad_string(&mut data, length, pad);
    }
}

fn pad_string(string: &mut String, length: usize, padding_char: char) {
    if length > string.len() {
        for _ in 0..length - string.len() {
            string.insert(0, padding_char);
        }
    }
}

// #[cfg(test)]
// mod test {
//     use crate::operation::example_operation;

//     use super::*;
//     use iso8853::ISO8853_BITMAP_TEMPLATE;

// }
