use std::{collections::HashMap, sync::OnceLock};

pub struct BitMapTemplate {
    pub data: BitMap,
    pub separator: char,
    pub length: usize,
}

pub type BitMap = HashMap<usize, BitField>;

pub enum BitField {
    SingleValue {
        field_name: String,
        min_length: usize,
        max_length: usize,
        padding_char: Option<char>,
    },
    MapValue(BitMap),
}

impl<'a> From<Iso8853BitFieldSingleValue<'a>> for BitField {
    fn from(value: Iso8853BitFieldSingleValue) -> Self {
        BitField::SingleValue {
            field_name: value.0.to_string(),
            min_length: value.1,
            max_length: value.2,
            padding_char: value.3,
        }
    }
}

impl<'a> From<Iso8853BitFieldMapValue<'a>> for BitField {
    fn from(value: Iso8853BitFieldMapValue) -> Self {
        BitField::MapValue(value.iter().map(|(i, x)| (*i, (*x).into())).collect())
    }
}

type Iso8853BitFieldSingleValue<'a> = (&'a str, usize, usize, Option<char>);
type Iso8853BitFieldMapValue<'a> = HashMap<usize, Iso8853BitFieldSingleValue<'a>>;

pub fn iso8853_bitmap_template() -> OnceLock<BitMapTemplate> {
    let data = HashMap::from([
        (1, ("first_bit", 3, 3, None).into()),
        (2, ("request_type", 4, 4, None).into()),
        (
            3,
            HashMap::from([
                (1, ("account_number", 8, 20, Some('0')).into()),
                (2, ("network", 1, 1, None).into()),
                (3, ("expiry_date", 4, 6, Some('0')).into()),
                (4, ("security_code", 3, 4, Some('0')).into()),
            ])
            .into(),
        ),
        (
            4,
            HashMap::from([
                (1, ("amount", 10, 20, Some('0')).into()),
                (2, ("currency", 3, 3, None).into()),
                (3, ("name", 0, 20, Some(' ')).into()),
            ])
            .into(),
        ),
    ]);
    let bm = BitMapTemplate {
        data,
        separator: '_',
        length: 8,
    };
    OnceLock::from(bm)
}

// pub const APACS_BITMAP_TEMPLATE: [(&str, usize, &str); 8] = [
//     (0, ("request_type", 2, " ")),
//     (0, ("expiry_date", 6, "")),
//     (0, ("security_code", 4, "0")),
//     (0, ("account_number", 16, "")),
//     (0, ("network", 1, "")),
//     (0, ("currency", 3, "")),
//     (0, ("amount", 10, "0")),
//     (0, ("name", 20, " ")),
// ];
