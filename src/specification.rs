use std::collections::HashMap;
mod bitmap_templates;
mod messaging_specification;

use crate::{operation::{Operation, RequestType}, payment::Payment, transaction::Transaction};

pub enum Specification {
    Iso8853,
    Apacs,
}

pub type FormatFunction = fn(&HashMap<String, String>) -> String;

impl Specification {
    pub fn encode_request(&self, op: &Operation) -> String {
        let required_information = self.parse_required_information(op);
        let format_fn: FormatFunction = match self {
            Specification::Iso8853 => iso8853_format,
            Specification::Apacs => apacs_format,
        };
        format_fn(&required_information)
    }

    pub fn parse_required_information<'a>(&self, op: &Operation) -> HashMap<String, String> {
        let mut data = HashMap::new();
        data.insert("request_type".into(), self.parse_request_type(&op.request_type));
        for (field, value) in self.parse_transaction(&op.transaction).into_iter() { // TODO can we chain these?
            data.insert(field.into(), value);
        }
        for (field, value) in self.parse_payment(&op.payment).into_iter() { // TODO can we chain these?
            data.insert(field.into(), value);
        }
        
        
        data
    }

    fn parse_request_type(&self, request_type: &RequestType) -> String {
        match self {
            Specification::Iso8853 => match request_type {
                RequestType::Auth => "AUTH",
                RequestType::Refund => "REFUND",
                RequestType::AccountCheck => "ACCOUNTCHECK",
            },
            Specification::Apacs => "01",
        }.into()
    }

    fn parse_transaction(&self, transaction: &Transaction) -> HashMap<String, String> {
        HashMap::new()
    }

    fn parse_payment(&self, payment: &Payment) -> HashMap<String, String> {
        HashMap::new()
    }

}


fn iso8853_format(data: &HashMap<String, String>) -> String {
    format!(
        "{}",
        data.get("request_type").expect("Missing request_type mapping")
    )
}

fn apacs_format(data: &HashMap<String, String>) -> String {
    format!(
        "{}",
        data.get("request_type").expect("Missing request_type mapping")
    )
}

fn get_spec_bitmap_template(spec: &Specification) -> Vec<(&str, u32)> {
    match spec {
        Specification::Iso8853 => bitmap_templates::ISO8853_BITMAP_TEMPLATE.into(),
        Specification::Apacs => bitmap_templates::APACS_BITMAP_TEMPLATE.into(),
    }
}