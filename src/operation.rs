use std::collections::HashMap;

use crate::{
    bank::Bank, merchant::Merchant, payment::Payment, transaction::Transaction, GatewayError,
    Result,
};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum RequestType {
    Auth,
    Refund,
    AccountCheck,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Operation {
    pub request_type: Option<RequestType>,
    pub bank: Option<Bank>,
    pub payment: Option<Payment>,
    pub transaction: Option<Transaction>,
    pub merchant: Option<Merchant>,
}

impl Operation {
    pub fn encode(&self) -> Result<String> {
        match self.bank {
            Some(bank) => bank.encode_request(&self),
            None => Err(GatewayError::EncodingError(format!(
                "This operation has no bank! Are you sure it needs to be encoded?"
            ))),
        }
    }

    // pub fn decode(&mut self, encoded_string: &str) {
    //     let _decoded: HashMap<String, String> = self.bank.decode_response_string(encoded_string);
    // }
}

impl TryFrom<HashMap<&str, String>> for Operation {
    fn try_from(v: HashMap<&str, String>) -> Result<Self> {
        let transaction = Transaction {
            amount: v
                .get("baseamount")
                .unwrap()
                .parse()
                .map_err(|err| GatewayError::FieldError(format!("{err}")))?,
            currency: v
                .get("currencyiso3a")
                .ok_or(GatewayError::FieldError(format!("Missing currencyiso3a")))?
                .parse()?,
            billingname: v.get("billingname").unwrap_or(&"".into()).into(),
        };

        Ok(Operation {
            request_type: Some(RequestType::Auth),
            bank: Some(Bank::Ems),
            payment: Some(Payment::card(
                "4000000000000000",
                "2024/12",
                "123",
                "Ben Jones",
            )),
            transaction: Some(transaction),
            merchant: Some(Merchant::new(
                "Test Merchant",
                "000104912345678",
                "test@merchant.com",
            )),
        })
    }

    type Error = GatewayError;
}

#[cfg(test)]
mod tests {
    use core::assert_eq;

    use crate::{
        Result,
        bank::Bank, currency::Currency, map, merchant::test_merchant, payment::Payment,
        transaction::Transaction, GatewayError,
    };

    use super::{example_operation, Operation, RequestType};

    #[test]
    fn test_card_auth_encoding() {
        let tests: Vec<(Payment, Result<Transaction>, Bank, RequestType, Result<String>)> = vec![
            (
                Payment::card("5100000000000000", "2024/12", "123", "Ben Jones"),
                Transaction::new(12345, Currency::GBP, "Ben Jones".into()),
                Bank::Ems,
                RequestType::Auth,
                Ok("0103abc0204AUTH0342011651000000000000000201M030620241204031230434011000000123450203GBP0309Ben Jones052001160000104912345678".to_string()),
            ),
            (
                Payment::card("5100000000000000", "2024/12", "123", "Ben Jones"),
                Transaction::new(12345, Currency::GBP, "Ben Jones".into()),
                Bank::Stfs,
                RequestType::Auth,
                Ok("01031230204AUTH0342011651000000000000000201M030620241204031230434011000000123450203GBP0309Ben Jones052001160000104912345678".to_string()),
            ),
            (
                Payment::card("5100000000000000", "2024/12", "123123", "Ben Jones"),
                Transaction::new(12345, Currency::GBP, "Ben Jones".into()),
                Bank::Stfs,
                RequestType::Auth,
                Err(GatewayError::EncodingError("value '123123' too long (6) for bitfield '3.4' (4)".into())),
            ),
        ];
        for (i, (payment, transaction, bank, request_type, expected)) in
            tests.into_iter().enumerate()
        {
            let op = Operation {
                payment: Some(payment),
                transaction: Some(transaction.unwrap()),
                bank: Some(bank),
                request_type: Some(request_type),
                merchant: Some(test_merchant()),
            };
            let request_string = op.encode();
            assert_eq!(expected, request_string, "Case number {}", i + 1);
        }
    }

    #[test]
    fn test_operation_from_hashmap() {
        let tests = [
            (
                map! {
                    "billingname"   => "Ben Jones".to_string(),
                    "currencyiso3a" => "GBP".to_string(),
                    "baseamount"    => "12345".to_string(),
                },
                Ok(example_operation()),
            ),
            (
                map! {
                    "billingname"   => "Ben Jones".to_string(),
                    "baseamount"    => "12345".to_string(),
                },
                Err(GatewayError::FieldError("Missing currencyiso3a".into())),
            ),
        ];
        for (hm, expected) in tests.into_iter() {
            let res = Operation::try_from(hm);
            assert_eq!(expected, res);
        }
    }
}

#[cfg(test)]
pub fn example_operation() -> crate::operation::Operation {
    use crate::merchant::test_merchant;

    crate::operation::Operation {
        payment: Some(crate::payment::Payment::card(
            "4000000000000000",
            "2024/12",
            "123",
            "Ben Jones",
        )),
        transaction: Some(crate::transaction::Transaction {
            amount: 12345,
            currency: crate::currency::Currency::GBP,
            billingname: "Ben Jones".into(),
        }),
        merchant: Some(test_merchant()),
        bank: Some(crate::bank::Bank::Ems),
        request_type: Some(crate::operation::RequestType::Auth),
    }
}
