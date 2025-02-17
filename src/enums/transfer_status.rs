pub enum TransferStatus {
    Pending,
    Completed,
    Failed,
}

impl TransferStatus {
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
    fn test_transfer_status_to_string_pending() {
        assert_eq!(TransferStatus::Pending.to_string(), "pending");
    }

    #[test]
    fn test_transfer_status_to_string_completed() {
        assert_eq!(TransferStatus::Completed.to_string(), "completed");
    }

    #[test]
    fn test_transfer_status_to_string_failed() {
        assert_eq!(TransferStatus::Failed.to_string(), "failed");
    }
}
