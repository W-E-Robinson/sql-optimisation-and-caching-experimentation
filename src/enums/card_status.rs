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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_card_status_to_string_savings() {
        assert_eq!(CardStatus::Active.to_string(), "active");
    }

    #[test]
    fn test_card_status_to_string_credit() {
        assert_eq!(CardStatus::Blocked.to_string(), "blocked");
    }

    #[test]
    fn test_card_status_to_string_business() {
        assert_eq!(CardStatus::Expired.to_string(), "expired");
    }
}
