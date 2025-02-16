pub enum CardType {
    Debit,
    Credit,
}

impl CardType {
    pub fn to_string(&self) -> String {
        match self {
            Self::Debit => String::from("debit"),
            Self::Credit => String::from("credit"),
        }
    }
}
