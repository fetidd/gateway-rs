use std::collections::HashMap;

use crate::{operation::{Operation, RequestType}, payment::Payment};

use super::bitmap_templates::*;

pub enum Specification {
    Iso8853,
    Apacs,
}

impl Specification {
    pub fn encode_request(&self, op: &Operation) -> String {
        let required_information = self.parse_required_information(op);
        let template: BitmapTemplate = match self {
            Specification::Iso8853 => ISO8853_BITMAP_TEMPLATE.as_slice(),
            Specification::Apacs => APACS_BITMAP_TEMPLATE.as_slice(),
        };
        let separator = match self {
            Specification::Iso8853 => "_",
            Specification::Apacs => "",
        };
        format(&required_information, &template, &separator)
    }

    pub fn parse_required_information<'a>(&self, op: &Operation) -> HashMap<String, String> {
        let mut data = HashMap::new();
        data.insert("request_type".into(), self.parse_request_type(&op));
        for (field, value) in self.parse_transaction(&op).into_iter() { // TODO can we chain these?
            data.insert(field.into(), value);
        }
        for (field, value) in self.parse_payment(&op).into_iter() { // TODO can we chain these?
            data.insert(field.into(), value);
        }
        data
    }

    fn parse_request_type(&self, op: &Operation) -> String {
        match self {
            Specification::Iso8853 => match op.request_type {
                RequestType::Auth => "AUTH",
                RequestType::Refund => "REFUND",
                RequestType::AccountCheck => "ACCOUNTCHECK",
            },
            Specification::Apacs => "01",
        }.into()
    }

    fn parse_transaction(&self, op: &Operation) -> HashMap<String, String> {
        let mut parsed = HashMap::new();
        let t = &op.transaction;
        match self {
            Specification::Iso8853 => {
                parsed.insert("amount".into(), t.amount.to_string());
            },
            Specification::Apacs => {},
        }
        parsed
    }

    fn parse_payment(&self, op: &Operation) -> HashMap<String, String> {
        let mut parsed = HashMap::new();
        let p = &op.payment;
        match self {
            Specification::Iso8853 => {
                parsed.insert("account_number".into(), match p {
                    Payment::Card { pan, ..} => pan,
                    Payment::Account { account_number, ..} => account_number,
                }.to_string());
            },
            Specification::Apacs => {},
        }
        parsed
    }

}

fn format(data: &HashMap<String, String>, template: BitmapTemplate, separator: &str) -> String {
    let mut output = String::new();
    for (i, (field, length, pad)) in template.iter().enumerate() {
        let from_data = data.get(*field).expect("Missing {field}");
        if from_data.len() > *length {
            panic!("too long!");
        }
        let padding = pad.repeat(length - from_data.len());
        output.push_str(&padding);
        output.push_str(from_data);
        if i != template.len() {
            output.push_str(separator);
        }
    }
    output
}
