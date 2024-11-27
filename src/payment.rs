pub enum Payment {
    Card {
        pan: String,
        expiry_date: String,
        security_code: String,
        name: String,
        network: String
    },
    Account {
        account_number: String,
        sort_code: String,
        name: String,
        bank_name: String,
    },
}

impl Payment {
    pub fn card(pan: &str, expiry_date: &str, security_code: &str, name: &str) -> Self {
        Self::Card {
            pan: pan.into(),
            expiry_date: expiry_date.into(),
            security_code: security_code.into(),
            name: name.into(),
            network: get_network_from_pan(&pan)
        }
    }
}

fn get_network_from_pan(pan: &str) -> String {
    match pan[0..4].parse::<i32>().expect("invalid pan") {
        4000..5000 => "VISA",
        5000..7000 => "MASTERCARD",
        _ => panic!("invalid pan")
    }.into()

}
