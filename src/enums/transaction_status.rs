pub enum TransactionStatus {
    Pending,
    Completed,
    Failed,
}

impl TransactionStatus {
    pub fn to_string(&self) -> String {
        match self {
            Self::Pending => String::from("pending"),
            Self::Completed => String::from("completed"),
            Self::Failed => String::from("failed"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_transaction_status_to_string_pending() {
        assert_eq!(TransactionStatus::Pending.to_string(), "pending");
    }

    #[test]
    fn test_transaction_status_to_string_completed() {
        assert_eq!(TransactionStatus::Completed.to_string(), "completed");
    }

    #[test]
    fn test_transaction_status_to_string_failed() {
        assert_eq!(TransactionStatus::Failed.to_string(), "failed");
    }
}
