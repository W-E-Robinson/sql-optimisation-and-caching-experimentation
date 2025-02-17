#[derive(PartialEq)]
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_account_type_to_string_checking() {
        assert_eq!(AccountType::Checking.to_string(), "checking");
    }

    #[test]
    fn test_account_type_to_string_savings() {
        assert_eq!(AccountType::Savings.to_string(), "savings");
    }

    #[test]
    fn test_account_type_to_string_credit() {
        assert_eq!(AccountType::Credit.to_string(), "credit");
    }

    #[test]
    fn test_account_type_to_string_business() {
        assert_eq!(AccountType::Business.to_string(), "business");
    }
}
