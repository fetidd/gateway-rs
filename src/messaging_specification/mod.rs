mod bitmap_templates;

use std::collections::HashMap;

use crate::operation::Operation;

use bitmap_templates::*;
pub type BitMap = HashMap<usize, BitField>;
pub type OperationParseResult = Result<Option<String>, String>;
pub type OperationParser = fn(&Operation) -> OperationParseResult;

#[derive(Debug)]
pub enum BitField {
    Single {
        parser: OperationParser,
        min_length: usize,
        max_length: usize,
        padding_char: Option<char>,
    },
    Map(BitMap),
}

pub enum MessagingSpecification {
    Iso8853,
    // Apacs,
}

impl MessagingSpecification {
    pub fn encode_request(&self, op: &Operation) -> Result<String, String> {
        self.format(&op, &self.get_template()())
    }

    pub fn get_template(&self) -> fn() -> BitMap {
        match self {
            MessagingSpecification::Iso8853 => iso8853_bitmap_template,
            // MessagingSpecification::Apacs => todo!(),
        }
    }

    pub fn format(&self, op: &Operation, template: &BitMap) -> Result<String, String> {
        format(&op, &template)
    }
}

fn format(op: &Operation, template: &BitMap) -> Result<String, String> {
    let mut output = String::new();
    _format(&op, &template, &mut output)?;
    Ok(output)
}

fn _format(op: &Operation, template: &BitMap, output: &mut String) -> Result<(), String> {
    let mut sorted: Vec<(&usize, &BitField)> = template.iter().collect();
    sorted.sort_by(|a, b| a.0.cmp(b.0));
    for (pos, field) in sorted.iter() {
        match field {
            BitField::Single {
                parser,
                padding_char,
                min_length,
                max_length,
            } => {
                if let Some(data) = parser(&op)? {
                    if data.len() > *max_length {
                        return Err("value too long for bitfield".into());
                    }
                    let pad = padding_char.map(|ch| (*min_length, ch));
                    let sub_string = string_field(**pos, &data, pad);
                    output.push_str(&sub_string);
                }
            }
            BitField::Map(map) => {
                let mut sorted: Vec<(&usize, &BitField)> = map.iter().collect();
                sorted.sort_by(|a, b| a.0.cmp(b.0));
                let mut nested = String::new();
                for (pos, field) in sorted.iter() {
                    match field {
                        BitField::Single {
                            parser,
                            padding_char,
                            min_length,
                            max_length,
                        } => {
                            if let Some(data) = parser(&op)? {
                                if data.len() > *max_length {
                                    return Err("value too long for bitfield".into());
                                }
                                let pad = padding_char.map(|ch| (*min_length, ch));
                                let sub_string = string_field(**pos, &data, pad);
                                nested.push_str(&sub_string);
                            }
                        }
                        BitField::Map(_map) => panic!("cannot handle more than 1 nested map"),
                    }
                }
                output.push_str(&string_field(**pos, &nested, None));
            }
        }
    }
    Ok(())
}

fn string_field(pos: usize, data: &str, pad: Option<(usize, char)>) -> String {
    let mut string = String::new();
    string.push_str(&format!("{:0>2}", pos));
    let mut data = data.to_owned();
    if let Some((length, pad)) = pad {
        pad_string(&mut data, length, pad);
    }
    string.push_str(&format!("{:0>2}", data.len()));
    string.push_str(&data);
    string
}

fn pad_string(string: &mut String, length: usize, padding_char: char) {
    if length > string.len() {
        for _ in 0..length - string.len() {
            string.insert(0, padding_char);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use bitmap_templates::iso8853_bitmap_template;

    #[test]
    fn derp() {
        let temp = iso8853_bitmap_template();
        let op = example_operation();
        let enc = format(&op, &temp).unwrap();
        assert_eq!("0103abc0204AUTH0342011640000000000000000201V030620241204031230434011000000123450203GBP0309Ben Jones", enc);
    }
}
