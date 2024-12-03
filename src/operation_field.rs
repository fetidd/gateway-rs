use std::any::Any;

use crate::Result;
use crate::{bank::Bank, GatewayError};

#[derive(Debug, Clone, PartialEq)]
pub struct Mid(String);

macro_rules! regex {
    ($name:ident, $pattern:expr) => {
        const $name: std::sync::LazyLock<regex::Regex> =
            std::sync::LazyLock::new(|| regex::Regex::new($pattern).unwrap());
    };
}
pub(crate) use regex;

macro_rules! get_ctx {
    ($ctx:ident, $type:ty) => {
        *$ctx.unwrap().downcast::<$type>().unwrap()
    };
}
pub(crate) use get_ctx;

macro_rules! ctx {
    () => {
        None
    };
    ($arg:expr) => {
        std::option::Option::Some(std::boxed::Box::new($arg))
    };
}
pub(crate) use ctx;

type Ctx = Option<Box<dyn Any>>;

regex!(MID_REGEX, "[0-9]+");
regex!(STFS_MID_REGEX, "^0001049[0-9]{8}$");

pub struct ValidationContext {}

pub trait Validator {
    type Output;
    fn validate(value: &str, ctx: Ctx) -> Result<Self::Output>;
}

impl Validator for Mid {
    type Output = Mid;

    fn validate(value: &str, ctx: Ctx) -> Result<Mid> {
        let bank = get_ctx!(ctx, Bank);
        let ptn = match bank {
            Bank::Stfs => STFS_MID_REGEX,
            _ => MID_REGEX,
        };
        if !ptn.is_match(&value) {
            Err(GatewayError::ValidationError(format!(
                "mid '{value}' does not match regex {}",
                ptn.as_str()
            )))
        } else {
            Ok(Mid(value.into()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let bank = Bank::Stfs;
        let mid = Mid::validate("00010491231231289", ctx!(bank));
        assert_eq!(
            mid,
            Err(GatewayError::ValidationError(format!(
                "mid '00010491231231289' does not match regex ^0001049[0-9]{{8}}$"
            )))
        );
        let mid = Mid::validate("000104912312312", ctx!(bank));
        assert_eq!(mid, Ok(Mid("000104912312312".into())));
    }
}
