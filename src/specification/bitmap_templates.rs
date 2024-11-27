pub type BitmapTemplate<'a> = &'a [(&'a str, usize, &'a str)];

pub const ISO8853_BITMAP_TEMPLATE: [(&str, usize, &str); 2] = [
    ("request_type", 4, " "),
    ("account_number", 20, "0"),
];

pub const APACS_BITMAP_TEMPLATE: [(&str, usize, &str); 1] = [
    ("request_type", 2, " "),
];
