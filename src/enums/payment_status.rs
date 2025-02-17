pub enum PaymentStatus {
    Pending,
    Completed,
    Failed,
}

impl PaymentStatus {
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
    fn test_payment_status_to_string_pending() {
        assert_eq!(PaymentStatus::Pending.to_string(), "pending");
    }

    #[test]
    fn test_payment_status_to_string_completed() {
        assert_eq!(PaymentStatus::Completed.to_string(), "completed");
    }

    #[test]
    fn test_payment_status_to_string_failed() {
        assert_eq!(PaymentStatus::Failed.to_string(), "failed");
    }
}
