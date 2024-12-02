use std::fmt::Display;

pub struct Transaction {
    pub amount: u32,
    pub currency: Currency,
    pub billingname: String,
}

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

impl Transaction {
    pub fn new(amount: u32, currency: Currency, billingname: Option<&str>) -> Result<Self, String> {
        Ok(Transaction {
            amount,
            currency,
            billingname: billingname.map_or("".into(), |s| s.into()),
        })
    }
}
