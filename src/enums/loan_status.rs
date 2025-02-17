pub enum LoanStatus {
    Approved,
    Rejected,
    Active,
    Closed,
}

impl LoanStatus {
    pub fn to_string(&self) -> String {
        match self {
            Self::Approved => String::from("approved"),
            Self::Rejected => String::from("rejected"),
            Self::Active => String::from("active"),
            Self::Closed => String::from("closed"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_loan_status_to_string_approved() {
        assert_eq!(LoanStatus::Approved.to_string(), "approved");
    }

    #[test]
    fn test_loan_status_to_string_rejected() {
        assert_eq!(LoanStatus::Rejected.to_string(), "rejected");
    }

    #[test]
    fn test_loan_status_to_string_active() {
        assert_eq!(LoanStatus::Active.to_string(), "active");
    }

    #[test]
    fn test_loan_status_to_string_closed() {
        assert_eq!(LoanStatus::Closed.to_string(), "closed");
    }
}
