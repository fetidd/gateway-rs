#![allow(non_snake_case)]

use super::{
    bitmap, string_field, BitField, EncodingContext, OperationParseResult, OperationParser,
};
use crate::{
    map,
    operation::{Operation, RequestType},
    payment::Payment,
};

bitmap! {ISO8853_BITMAP_TEMPLATE,
    1 => (TransactionIdentifier as OperationParser, 3, 3, None),
    2 => (RequestType as OperationParser, 4, 4, None),
    3 => map!{
        1 => (AccountNumber as OperationParser, 8, 20, Some('0')),
        2 => (Network as OperationParser, 1, 1, None),
        3 => (ExpiryDate as OperationParser, 4, 6, Some('0')),
        4 => (CVV as OperationParser, 3, 4, Some('0')),
    },
    4 => map!{
        1 => (TransactionAmount as OperationParser, 10, 20, Some('0')),
        2 => (Currency as OperationParser, 3, 3, None),
        3 => (BillingName as OperationParser, 0, 20, Some(' ')),
    },
    5 => map!{
        1 => (MerchantID as OperationParser, 16, 16, Some('0')),
    },
}

type Iso8853BitField = (OperationParser, usize, usize, Option<char>);

impl From<Iso8853BitField> for BitField {
    fn from(value: Iso8853BitField) -> Self {
        Self::Single {
            parser: value.0,
            min_length: value.1,
            max_length: value.2,
            padding_char: value.3,
        }
    }
}

pub fn iso8853_string_field(data: &mut String, ctx: EncodingContext, _field: &BitField) {
    let pos = ctx.position.unwrap();
    string_field(data, ctx, _field);
    data.insert_str(0, &format!("{:0>2}", data.len()));
    data.insert_str(0, &format!("{:0>2}", pos));
}

pub fn TransactionIdentifier(_: &Operation) -> OperationParseResult {
    Ok(Some("abc".into()))
}

pub fn MerchantID(op: &Operation) -> OperationParseResult {
    Ok(Some(
        op.merchant.as_ref().expect("TODO handle").mid.to_string(),
    ))
}

pub fn RequestType(op: &Operation) -> OperationParseResult {
    let rt = match op.request_type.expect("TODO handle") {
        RequestType::Auth => "AUTH",
        RequestType::Refund => todo!(),
        RequestType::AccountCheck => todo!(),
    }
    .into();
    Ok(Some(rt))
}

pub fn AccountNumber(op: &Operation) -> OperationParseResult {
    match op.payment.as_ref().expect("TODO handle") {
        Payment::Card { pan, .. } => Ok(Some(pan.into())),
        Payment::Account { account_number, .. } => Ok(Some(account_number.into())),
    }
}

pub fn Network(op: &Operation) -> OperationParseResult {
    match op.payment.as_ref().expect("TODO handle") {
        Payment::Card { network, .. } => Ok(Some(network[..1].into())),
        _ => Ok(None),
    }
}

pub fn ExpiryDate(op: &Operation) -> OperationParseResult {
    match op.payment.as_ref().expect("TODO handle") {
        Payment::Card { expiry_date, .. } => Ok(Some(expiry_date.replace("/", ""))),
        _ => Ok(None),
    }
}

pub fn CVV(op: &Operation) -> OperationParseResult {
    match op.payment.as_ref().expect("TODO handle") {
        Payment::Card { security_code, .. } => Ok(Some(security_code.into())),
        _ => Ok(None),
    }
}

pub fn TransactionAmount(op: &Operation) -> OperationParseResult {
    Ok(Some(
        op.transaction
            .as_ref()
            .expect("TODO handle")
            .amount
            .to_string(),
    ))
}

pub fn BillingName(op: &Operation) -> OperationParseResult {
    Ok(Some(
        op.transaction
            .as_ref()
            .expect("TODO handle")
            .billingname
            .to_string(),
    ))
}

pub fn Currency(op: &Operation) -> OperationParseResult {
    Ok(Some(
        op.transaction
            .as_ref()
            .expect("TODO handle")
            .currency
            .to_string(),
    ))
}

#[cfg(test)]
mod tests {
    use crate::operation::example_operation;

    use super::*;

    #[test]
    fn test_Currency() {
        let tests = [
            (example_operation(), "GBP".to_string()),
        ];
        for (op, expected) in tests.into_iter() {
            let actual = Currency(&op).unwrap().unwrap();
            assert_eq!(expected, actual);
        }
    }

    #[test]
    fn test_TransactionIdentifier() {
        let tests = [
            (example_operation(), "abc".to_string()),
        ];
        for (op, expected) in tests.into_iter() {
            let actual = TransactionIdentifier(&op).unwrap().unwrap();
            assert_eq!(expected, actual);
        }
    }
}
