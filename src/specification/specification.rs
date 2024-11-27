use std::collections::HashMap;

use crate::{
    operation::{Operation, RequestType},
    payment::Payment,
};

use super::bitmap_templates::*;

pub enum Specification {
    Iso8853,
    Apacs,
}

impl Specification {
    pub fn encode_request(&self, op: &Operation) -> Result<String, String> {
        let required_information = self.parse_required_information(op);
        self.format(&required_information)
    }

    fn separator(&self) -> String {
        match self {
            Specification::Iso8853 => "_",
            Specification::Apacs => "",
        }
        .into()
    }

    pub fn parse_required_information<'a>(&self, op: &Operation) -> HashMap<String, String> {
        let mut data = HashMap::new();
        data.insert("request_type".into(), self.parse_request_type(&op));
        for (field, value) in self.parse_transaction(&op).into_iter() {
            data.insert(field.into(), value);
        }
        for (field, value) in self.parse_payment(&op).into_iter() {
            data.insert(field.into(), value);
        }
        match self {
            Specification::Iso8853 => insert_iso8853_extras(&mut data),
            _ => (),
        };
        data
    }

    pub fn format(&self, data: &HashMap<String, String>) -> Result<String, String> {
        match self {
            Specification::Iso8853 => {
                format(data, ISO8853_BITMAP_TEMPLATE.as_slice(), &self.separator())
            }
            Specification::Apacs => {
                format(data, APACS_BITMAP_TEMPLATE.as_slice(), &self.separator())
            }
        }
    }

    fn parse_request_type(&self, op: &Operation) -> String {
        match self {
            Specification::Iso8853 => match op.request_type {
                RequestType::Auth => "AUTH",
                RequestType::Refund => "REFUND",
                RequestType::AccountCheck => "ACCOUNTCHECK",
            },
            Specification::Apacs => "01",
        }
        .into()
    }

    fn parse_transaction(&self, op: &Operation) -> HashMap<String, String> {
        let mut parsed = HashMap::new();
        let t = &op.transaction;
        match self {
            Specification::Iso8853 => {
                parsed.insert("amount".into(), t.amount.to_string());
                parsed.insert("currency".into(), t.currency.to_string());
                parsed.insert("name".into(), t.billingname.to_string());
            }
            Specification::Apacs => {
                parsed.insert("amount".into(), t.amount.to_string());
                parsed.insert("currency".into(), t.currency.to_string());
                parsed.insert("name".into(), t.billingname.to_string());
            }
        }
        parsed
    }

    fn parse_payment(&self, op: &Operation) -> HashMap<String, String> {
        match &op.payment {
            Payment::Card {
                pan,
                expiry_date,
                security_code,
                name,
                network,
            } => HashMap::from(
                [
                    ("account_number", pan),
                    ("expiry_date", &expiry_date.replace("/", "")),
                    ("security_code", security_code),
                    ("name", name),
                    ("network", &network[..1].to_string()),
                ]
                .map(|(k, v)| (k.to_string(), v.to_string())),
            ),
            _ => unreachable!(),
        }
    }
}

fn format(
    data: &HashMap<String, String>,
    template: BitmapTemplate,
    separator: &str,
) -> Result<String, String> {
    let mut output = String::new();
    for (field, length, pad) in template.iter() {
        if *field == "separator" {
            output.push_str(separator);
            continue;
        }
        let from_data = data.get(*field).expect(&format!("Missing {field}"));
        if from_data.len() > *length {
            return Err(format!("TODO {field} too long error"));
        }
        let padding = pad.repeat(length - from_data.len());
        output.push_str(&padding);
        output.push_str(from_data);
    }
    Ok(output)
}

fn insert_iso8853_extras(data: &mut HashMap<String, String>) {
    data.insert("first_bit".into(), "abc".into());
}
