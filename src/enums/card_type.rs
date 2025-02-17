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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_card_type_to_string_checking() {
        assert_eq!(CardType::Debit.to_string(), "debit");
    }

    #[test]
    fn test_card_type_to_string_savings() {
        assert_eq!(CardType::Credit.to_string(), "credit");
    }
}
