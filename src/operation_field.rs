use crate::bank::Bank;
use regex::Regex;
use std::sync::LazyLock;

struct Mid(String);

macro_rules! regex {
    ($name:ident, $pattern:expr) => {
        const $name: LazyLock<Regex> = LazyLock::new(|| Regex::new(stringify!($pattern)).unwrap());
    };
}

regex!(MID_REGEX, "[0-9]{8,}");
regex!(STFS_MID_REGEX, "0001049[0-9]{8}");

#[derive(Debug)]
enum GatewayError {
    ValidationError(String),
}
type Result<T> = std::result::Result<T, GatewayError>;

impl Mid {
    fn from(value: &str, bank: &Bank) -> Result<Mid> {
        let ptn = match bank {
            Bank::Stfs => STFS_MID_REGEX,
            _ => MID_REGEX,
        };
        if ptn.is_match(&value) {
            Err(GatewayError::ValidationError(format!(
                "mid '{value}' does not match regex"
            )))
        } else {
            Ok(Mid(value.into()))
        }
    }
}

macro_rules! field {
    ($field_type:tt, $val_type:ty, $validation:block) => {
        struct $field_type;
        impl $field_type {
            fn from(value: $val_type) -> Result<$field_type> {
                $validation(&value)?;
                Ok($field_type(value))
            }
        }
    };
}

field!(Derp(u32), &u32, {
    return Derp(value);
});

pub struct ValOp {
    mid: Mid,
    bank: Bank,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let bank = Bank::Stfs;
        let op = ValOp {
            mid: Mid::from("000104912312312", &bank).unwrap(),
            bank,
        };
    }
}
