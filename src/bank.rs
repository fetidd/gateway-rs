use std::collections::HashMap;

use crate::{operation::Operation, specification::specification::Specification};

#[derive(Copy, Clone)]
pub enum Bank {
    Ems,
    Hsbc,
    Fdms,
    Cardnet,
    Stfs,
    Lloyds,
    Barclays,
}

impl Bank {
    pub fn encode_request(&self, op: &Operation) -> Result<String, String> {
        let spec = self.spec();
        match self {
            Bank::Stfs => {
                let mut data = spec.parse_required_information(op);
                if let Some(x) = data.get_mut("first_bit") {
                    *x = "123".into();
                }
                spec.format(&data)
            }
            _ => spec.encode_request(op),
        }
    }

    pub fn spec(&self) -> Specification {
        match self {
            Bank::Ems | Bank::Fdms | Bank::Cardnet | Bank::Stfs => Specification::Iso8853,
            Bank::Hsbc | Bank::Lloyds | Bank::Barclays => Specification::Apacs,
        }
    }

    pub fn decode_response_string(&self, _encoded_string: &str) -> HashMap<String, String> {
        todo!()
    }
}
