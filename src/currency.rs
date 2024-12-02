use std::{fmt::Display, str::FromStr};

use crate::GatewayError;

#[derive(Debug, Clone, PartialEq)]
pub enum Currency {
    GBP,
    USD,
}

impl Display for Currency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Currency::GBP => "GBP",
            Currency::USD => "USD",
        })
    }
}

impl FromStr for Currency {
    type Err = GatewayError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GBP" => Ok(Self::GBP),
            "USD" => Ok(Self::USD),
            invalid => Err(GatewayError::FieldError(format!("Invalid currency: {invalid}")))
        }
    }
}