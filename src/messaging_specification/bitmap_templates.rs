use std::collections::HashMap;

use iso8853_parsers::{
    AccountNumber, BillingName, CardholderVerificationNumber, Currency, ExpiryDate, MerchantID,
    Network, RequestType, TransactionAmount, TransactionIdentifier,
};

use super::{BitField, BitMap, OperationParser};

impl From<Iso8853BitFieldSingle> for BitField {
    fn from(value: Iso8853BitFieldSingle) -> Self {
        BitField::Single {
            parser: value.0,
            min_length: value.1,
            max_length: value.2,
            padding_char: value.3,
        }
    }
}

impl From<Iso8853BitFieldMap> for BitField {
    fn from(value: Iso8853BitFieldMap) -> Self {
        BitField::Map(value.into_iter().map(|(i, x)| (i, x.into())).collect())
    }
}

type Iso8853BitFieldSingle = (OperationParser, usize, usize, Option<char>);
type Iso8853BitFieldMap = HashMap<usize, Iso8853BitFieldSingle>;

pub fn iso8853_bitmap_template() -> BitMap {
    let mut hm = HashMap::new();
    hm.insert(
        1,
        (TransactionIdentifier as OperationParser, 3, 3, None).into(),
    );
    hm.insert(2, (RequestType as OperationParser, 4, 4, None).into());

    // Payment Data
    let mut payment_fields = HashMap::new();
    payment_fields.insert(
        1,
        (AccountNumber as OperationParser, 8, 20, Some('0')).into(),
    );
    payment_fields.insert(2, (Network as OperationParser, 1, 1, None).into());
    payment_fields.insert(3, (ExpiryDate as OperationParser, 4, 6, Some('0')).into());
    payment_fields.insert(
        4,
        (
            CardholderVerificationNumber as OperationParser,
            3,
            4,
            Some('0'),
        )
            .into(),
    );
    hm.insert(3, payment_fields.into());

    // Transaction Data
    let mut trxn_fields = HashMap::new();
    trxn_fields.insert(
        1,
        (TransactionAmount as OperationParser, 10, 20, Some('0')).into(),
    );
    trxn_fields.insert(2, (Currency as OperationParser, 3, 3, None).into());
    trxn_fields.insert(3, (BillingName as OperationParser, 0, 20, Some(' ')).into());
    hm.insert(4, trxn_fields.into());

    // Merchant Data
    let mut merchant_fields = HashMap::new();
    merchant_fields.insert(1, (MerchantID as OperationParser, 16, 16, Some('0')).into());
    hm.insert(5, merchant_fields.into());

    hm
}

mod iso8853_parsers {
    use crate::operation::Operation;

    use crate::messaging_specification::OperationParseResult;

    #[allow(non_snake_case)]
    pub fn TransactionIdentifier(_: &Operation) -> OperationParseResult {
        Ok(Some("abc".into()))
    }

    #[allow(non_snake_case)]
    pub fn MerchantID(op: &Operation) -> OperationParseResult {
        Ok(Some(op.merchant.mid.to_string()))
    }

    #[allow(non_snake_case)]
    pub fn RequestType(op: &Operation) -> OperationParseResult {
        let rt = match op.request_type {
            crate::operation::RequestType::Auth => "AUTH",
            crate::operation::RequestType::Refund => todo!(),
            crate::operation::RequestType::AccountCheck => todo!(),
        }
        .into();
        Ok(Some(rt))
    }

    #[allow(non_snake_case)]
    pub fn AccountNumber(op: &Operation) -> OperationParseResult {
        match &op.payment {
            crate::payment::Payment::Card { pan, .. } => Ok(Some(pan.into())),
            crate::payment::Payment::Account { account_number, .. } => {
                Ok(Some(account_number.into()))
            }
        }
    }

    #[allow(non_snake_case)]
    pub fn Network(op: &Operation) -> OperationParseResult {
        match &op.payment {
            crate::payment::Payment::Card { network, .. } => Ok(Some(network[..1].into())),
            _ => Ok(None),
        }
    }

    #[allow(non_snake_case)]
    pub fn ExpiryDate(op: &Operation) -> OperationParseResult {
        match &op.payment {
            crate::payment::Payment::Card { expiry_date, .. } => {
                Ok(Some(expiry_date.replace("/", "")))
            }
            _ => Ok(None),
        }
    }

    #[allow(non_snake_case)]
    pub fn CardholderVerificationNumber(op: &Operation) -> OperationParseResult {
        match &op.payment {
            crate::payment::Payment::Card { security_code, .. } => Ok(Some(security_code.into())),
            _ => Ok(None),
        }
    }

    #[allow(non_snake_case)]
    pub fn TransactionAmount(op: &Operation) -> OperationParseResult {
        Ok(Some(op.transaction.amount.to_string()))
    }

    #[allow(non_snake_case)]
    pub fn BillingName(op: &Operation) -> OperationParseResult {
        Ok(Some(op.transaction.billingname.to_string()))
    }

    #[allow(non_snake_case)]
    pub fn Currency(op: &Operation) -> OperationParseResult {
        Ok(Some(op.transaction.currency.to_string()))
    }
}
