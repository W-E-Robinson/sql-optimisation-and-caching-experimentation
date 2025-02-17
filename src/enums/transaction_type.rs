pub enum TransactionType {
    Deposit,
    Withdrawel,
}

impl TransactionType {
    pub fn to_string(&self) -> String {
        match self {
            Self::Deposit => String::from("deposit"),
            Self::Withdrawel => String::from("withdrawel"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_transaction_type_to_string_deposit() {
        assert_eq!(TransactionType::Deposit.to_string(), "deposit");
    }

    #[test]
    fn test_transaction_type_to_string_withdrawel() {
        assert_eq!(TransactionType::Withdrawel.to_string(), "withdrawel");
    }
}
