pub type BitmapTemplate<'a> = &'a [(&'a str, usize, &'a str)];

pub const ISO8853_BITMAP_TEMPLATE: [(&str, usize, &str); 12] = [
    ("first_bit", 3, ""),
    ("separator", 1, ""),
    ("request_type", 4, " "),
    ("separator", 1, ""),
    ("account_number", 20, "0"),
    ("network", 1, ""),
    ("expiry_date", 6, ""),
    ("security_code", 4, "0"),
    ("separator", 1, ""),
    ("amount", 10, "0"),
    ("currency", 3, ""),
    ("name", 20, " "),
];

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
