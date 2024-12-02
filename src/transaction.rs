use crate::currency::Currency;

#[derive(Debug, Clone, PartialEq)]
pub struct Transaction {
    pub amount: u32,
    pub currency: Currency,
    pub billingname: String,
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
