pub enum CardStatus {
    Active,
    Blocked,
    Expired,
}

impl CardStatus {
    pub fn to_string(&self) -> String {
        match self {
            Self::Active => String::from("active"),
            Self::Blocked => String::from("blocked"),
            Self::Expired => String::from("expired"),
        }
    }
}
