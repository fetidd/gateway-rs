use std::collections::HashMap;

use crate::{bank::Bank, merchant::Merchant, payment::Payment, transaction::Transaction};

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
    pub merchant: Merchant,
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

    use crate::{
        bank::Bank,
        merchant::test_merchant,
        payment::Payment,
        transaction::{Currency, Transaction},
    };

    use super::{Operation, RequestType};

    #[test]
    fn test_card_auth_encoding() {
        let tests: Vec<(Payment, Result<Transaction, String>, Bank, RequestType, &str)> = vec![
            (
                Payment::card("5100000000000000", "2024/12", "123", "Ben Jones"),
                Transaction::new(12345, Currency::GBP, "Ben Jones".into()),
                Bank::Ems,
                RequestType::Auth,
                "0103abc0204AUTH0342011651000000000000000201M030620241204031230434011000000123450203GBP0309Ben Jones",
            ),
            (
                Payment::card("5100000000000000", "2024/12", "123", "Ben Jones"),
                Transaction::new(12345, Currency::GBP, "Ben Jones".into()),
                Bank::Stfs,
                RequestType::Auth,
                "01031230204AUTH0342011651000000000000000201M030620241204031230434011000000123450203GBP0309Ben Jones",
            ),
        ];
        for (i, (payment, transaction, bank, request_type, expected)) in
            tests.into_iter().enumerate()
        {
            let op = Operation {
                payment,
                transaction: transaction.unwrap(),
                bank,
                request_type,
                merchant: test_merchant(),
            };
            let request_string = op.encode().expect("This should be fine!");
            assert_eq!(request_string, *expected, "Case number {}", i + 1);
        }
    }
}

#[cfg(test)]
pub fn example_operation() -> crate::operation::Operation {
    use crate::merchant::test_merchant;

    crate::operation::Operation {
        payment: crate::payment::Payment::card("4000000000000000", "2024/12", "123", "Ben Jones"),
        transaction: crate::transaction::Transaction {
            amount: 12345,
            currency: crate::transaction::Currency::GBP,
            billingname: "Ben Jones".into(),
        },
        merchant: test_merchant(),
        bank: crate::bank::Bank::Ems,
        request_type: crate::operation::RequestType::Auth,
    }
}
