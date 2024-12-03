use crate::operation_field::{ctx, Mid, Validator};

#[derive(Debug, Clone, PartialEq)]
pub struct Merchant {
    pub name: String,
    pub mid: Mid,
    pub email: String,
}

impl Merchant {
    pub fn new(name: &str, mid: &str, email: &str) -> Self {
        Self {
            name: name.into(),
            mid: Mid::validate(mid, ctx!()).unwrap(),
            email: email.into(),
        }
    }
}

#[cfg(test)]
pub fn test_merchant() -> Merchant {
    Merchant::new("Test Merchant", "000104912345678", "test@merchant.com")
}
