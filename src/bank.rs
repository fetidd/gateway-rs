use std::collections::HashMap;

use stfs_parsers::TransactionIdentifier;

use crate::{
    messaging_specification::{BitField, MessagingSpecification, OperationParser},
    operation::Operation,
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
    pub fn encode_request(&self, op: &Operation) -> Result<String, String> {
        let spec = self.spec();
        match self {
            Bank::Stfs => {
                let mut template = spec.get_template()();
                template.entry(1).and_modify(|bf| {
                    if let BitField::Single { parser, .. } = bf {
                        *parser = TransactionIdentifier as OperationParser;
                    }
                });
                spec.format(&op, &template)
            }
            _ => spec.encode_request(op),
        }
    }

    pub fn spec(&self) -> MessagingSpecification {
        match self {
            Bank::Ems | Bank::Fdms | Bank::Cardnet | Bank::Stfs => MessagingSpecification::Iso8853,
            Bank::Hsbc | Bank::Lloyds | Bank::Barclays => MessagingSpecification::Apacs,
        }
    }

    pub fn decode_response_string(&self, _encoded_string: &str) -> HashMap<String, String> {
        todo!()
    }
}

mod stfs_parsers {
    use crate::{messaging_specification::OperationParseResult, operation::Operation};

    #[allow(non_snake_case)]
    pub fn TransactionIdentifier(_: &Operation) -> OperationParseResult {
        Ok(Some("123".into()))
    }
}
