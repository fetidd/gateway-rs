use std::collections::HashMap;

use crate::{
    operation::Operation,
    specification::specification::Specification,
};

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
    pub fn encode_request(&self, op: &Operation) -> String {
        let spec = match self {
            Bank::Ems | Bank::Fdms | Bank::Cardnet | Bank::Stfs => Specification::Iso8853,
            Bank::Hsbc | Bank::Lloyds | Bank::Barclays => Specification::Apacs,
        };
        match self {
            Bank::Stfs => String::from("THIS IS STFS LOL"),
            _ => spec.encode_request(op),
        }
    }

    pub fn decode_response_string(&self, _encoded_string: &str) -> HashMap<String, String> {
        todo!()
    }
}
