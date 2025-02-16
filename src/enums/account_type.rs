pub enum AccountType {
    Checking,
    Savings,
    Credit,
    Business,
}

impl AccountType {
    pub fn to_string(&self) -> String {
        match self {
            Self::Checking => String::from("checking"),
            Self::Savings => String::from("savings"),
            Self::Credit => String::from("credit"),
            Self::Business => String::from("business"),
        }
    }
}
