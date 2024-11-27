use std::collections::HashMap;

use crate::{payment::Payment, transaction::Transaction, bank::Bank};

#[derive(Copy, Clone)]
pub enum RequestType {
    Auth, Refund, AccountCheck
}

pub struct Operation {
    pub request_type: RequestType,
    pub bank: Bank,
    pub payment: Payment,
    pub transaction: Transaction,
}

impl Operation {
    pub fn encode(&self) -> String {
        self.bank.encode_request(&self)
    }

    pub fn decode(&mut self, encoded_string: &str) {
        let _decoded: HashMap<String, String> = self.bank.decode_response_string(encoded_string);
    }
}

#[cfg(test)]
mod test {
    use core::assert_eq;

    use crate::{bank::Bank, payment::Payment,  transaction::Transaction};

    use super::{Operation, RequestType};
    
    #[test]
    fn test_card_auth_encoding() {
        let tests: Vec<(&str, Bank, RequestType, &str)> = vec![
            ("4000000000000000", Bank::Stfs, RequestType::Auth, "THIS IS STFS LOL"),
            ("5100000000000000", Bank::Ems, RequestType::Auth, "AUTH_00005100000000000000M202412123           Ben Jones_0000012345GBP           Ben Jones"),
            ("4000000000000000", Bank::Ems, RequestType::Auth, "AUTH_00004000000000000000V202412123           Ben Jones_0000012345GBP           Ben Jones"),
            ("4000000000000000", Bank::Hsbc, RequestType::Auth, "012024121234000000000000000V           Ben JonesGBP0000012345           Ben Jones"),
        ];
        for (i, (pan, bank, request_type, expected)) in tests.iter().enumerate() {
            let op = Operation {
                payment: Payment::card(pan, "2024/12", "123", "Ben Jones"),
                transaction: Transaction {
                    amount: 12345,
                    currency: "GBP".into(),
                    billingname: "Ben Jones".into(),
                    merchantname: "Amazon".into(),
                },
                bank: *bank,
                request_type: *request_type
            };
            let request_string = op.encode();
            assert_eq!(request_string, *expected, "Case number {}", i+1);
        }
    }

    // #[test]
    // fn test_auth_decoding() {
    //     let op = Operation {
    //         payment: Payment::Card {
    //             pan: "4000 0000 0000 0000".into(),
    //             expiry_date: "2024/12".into(),
    //             security_code: "123".into(),
    //             name: "Ben Jones".into(),
    //         },
    //         transaction: Transaction {
    //             amount: 12345,
    //             currency: "GBP".into(),
    //             billingname: "Ben Jones".into(),
    //             merchantname: "Amazon".into(),
    //         },
    //         bank: Bank::Hsbc,
    //         request_type: RequestType::Auth
    //     };
    //     let request_string = op.encode();
    //     assert_eq!(request_string, "")
    // }
}