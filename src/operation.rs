use std::collections::HashMap;

use crate::{bank::Bank, payment::Payment, transaction::Transaction};

#[derive(Copy, Clone)]
pub enum RequestType {
    Auth,
    Refund,
    AccountCheck,
}

pub struct Operation {
    pub request_type: RequestType,
    pub bank: Bank,
    pub payment: Payment,
    pub transaction: Transaction,
}

impl Operation {
    pub fn encode(&self) -> Result<String, String> {
        self.bank.encode_request(&self)
    }

    pub fn decode(&mut self, encoded_string: &str) {
        let _decoded: HashMap<String, String> = self.bank.decode_response_string(encoded_string);
    }
}

#[cfg(test)]
mod test {
    use core::assert_eq;

    use crate::{bank::Bank, payment::Payment, transaction::Transaction};

    use super::{Operation, RequestType};

    #[test]
    fn test_card_auth_encoding() {
        let tests: Vec<(Payment, Bank, RequestType, &str)> = vec![
            (
                Payment::card("5100000000000000", "2024/12", "123", "Ben Jones"),
                Bank::Ems,
                RequestType::Auth,
                "0103abc0204AUTH0342011651000000000000000201M030620241204031230434011000000123450203GBP0309Ben Jones",
            ),
            (
                Payment::card("5100000000000000", "2024/12", "123", "Ben Jones"),
                Bank::Stfs,
                RequestType::Auth,
                "01031230204AUTH0342011651000000000000000201M030620241204031230434011000000123450203GBP0309Ben Jones",
            ),
        ];
        for (i, (payment, bank, request_type, expected)) in tests.into_iter().enumerate() {
            let op = Operation {
                payment,
                transaction: Transaction {
                    amount: 12345,
                    currency: "GBP".into(),
                    billingname: "Ben Jones".into(),
                    merchantname: "Amazon".into(),
                },
                bank,
                request_type,
            };
            let request_string = op.encode().expect("This should be fine!");
            assert_eq!(request_string, *expected, "Case number {}", i + 1);
        }
    }
}
