pub enum TransactionType {
    Deposit,
    Withdrawal,
}

impl TransactionType {
    pub fn to_string(&self) -> String {
        match self {
            Self::Deposit => String::from("deposit"),
            Self::Withdrawal => String::from("withdrawal"),
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
    fn test_transaction_type_to_string_withdrawal() {
        assert_eq!(TransactionType::Withdrawal.to_string(), "withdrawal");
    }
}
