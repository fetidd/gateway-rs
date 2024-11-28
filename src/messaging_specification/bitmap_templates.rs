use std::{sync::OnceLock, collections::HashMap};

struct BitMap {
    data: HashMap<usize, BitField>,
    separator: char
}

enum BitField {
    SingleValue {field_name: String, length: usize, padding_char: char},
    MapValue(BitMap)
}

impl<'a> From<Iso8853BitFieldSingleValue<'a>> for BitField {
    fn from(value: Iso8853BitFieldSingleValue) -> Self {
        BitField::SingleValue {
            field_name: value.0.to_string(),
            length: value.1,
            padding_char: value.2,
        }
    }
}

impl<'a> From<Iso8853BitFieldMapValue<'a>> for BitField {
    fn from(value: Iso8853BitFieldMapValue) -> Self {
        BitField::MapValue(
            BitMap {
                data: HashMap::from(
                    value
                        .into_iter()
                        .map(|(i, j)| (i, BitField::from(j)))
                        .collect::<Vec<(usize, BitField)>>()
                        .as_slice()
                ),
                separator: ' ',
            }
        )
    }
}

type Iso8853BitFieldSingleValue<'a> = (&'a str, usize, char);
type Iso8853BitFieldMapValue<'a> = HashMap<usize, Iso8853BitFieldSingleValue<'a>>;

pub fn iso8853_bitmap_template() -> OnceLock<HashMap<usize, BitField>> {
    OnceLock::from(HashMap::from([
        (0, ("first_bit", 3, ' ').into()),
        (1, ("request_type", 4, ' ').into()),

        (2, ("account_number", 20, '0').into()),
        (3, ("network", 1, ' ').into()),
        (4, ("expiry_date", 6, ' ').into()),
        (5, ("security_code", 4, '0').into()),

        (6, ("amount", 10, '0').into()),
        (7, ("currency", 3, ' ').into()),
        (8, ("name", 20, ' ').into()),
    ]))
}

pub const APACS_BITMAP_TEMPLATE: [(&str, usize, &str); 8] = [
    ("request_type", 2, " "),
    ("expiry_date", 6, ""),
    ("security_code", 4, "0"),
    ("account_number", 16, ""),
    ("network", 1, ""),
    ("currency", 3, ""),
    ("amount", 10, "0"),
    ("name", 20, " "),
];
