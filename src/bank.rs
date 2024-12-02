use std::collections::HashMap;

use crate::{
    Result,
    messaging_specification::{BitField, MessagingSpecification, OperationParser},
    operation::Operation,
};

#[derive(Copy, Clone, Debug, PartialEq)]
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
    pub fn encode_request(&self, op: &Operation) -> Result<String> {
        let spec = self.spec();
        match self {
            Bank::Stfs => {
                // STFS is essentially the iso spec but with minor differences, so all we need to do is clone the template (any less expensive way?) and change one of the parser functions
                let mut template = spec.get_template();
                template.entry(1).and_modify(|bf| {
                    if let BitField::Single { parser, .. } = bf {
                        *parser = stfs_parsers::TransactionIdentifier as OperationParser;
                    }
                });
                spec.encode_using_template(&op, &template)
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
