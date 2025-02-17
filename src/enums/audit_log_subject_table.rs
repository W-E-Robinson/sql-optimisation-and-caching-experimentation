pub enum AuditLogSubjectTable {
    Users,
    Accounts,
    Cards,
    Transfers,
    Transactions,
    Loans,
    Payments,
}

impl AuditLogSubjectTable {
    pub fn to_string(&self) -> &'static str {
        match self {
            Self::Users => "users",
            Self::Accounts => "accounts",
            Self::Cards => "cards",
            Self::Transfers => "transfers",
            Self::Transactions => "transactions",
            Self::Loans => "loans",
            Self::Payments => "payments",
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_audit_subject_type_to_string_users() {
        assert_eq!(AuditLogSubjectTable::Users.to_string(), "users");
    }

    #[test]
    fn test_audit_subject_type_to_string_accounts() {
        assert_eq!(AuditLogSubjectTable::Accounts.to_string(), "accounts");
    }

    #[test]
    fn test_audit_subject_type_to_string_cards() {
        assert_eq!(AuditLogSubjectTable::Cards.to_string(), "cards");
    }

    #[test]
    fn test_audit_subject_type_to_string_transfers() {
        assert_eq!(AuditLogSubjectTable::Transfers.to_string(), "transfers");
    }

    #[test]
    fn test_audit_subject_type_to_string_transactions() {
        assert_eq!(
            AuditLogSubjectTable::Transactions.to_string(),
            "transactions"
        );
    }

    #[test]
    fn test_audit_subject_type_to_string_loans() {
        assert_eq!(AuditLogSubjectTable::Loans.to_string(), "loans");
    }

    #[test]
    fn test_audit_subject_type_to_string_payments() {
        assert_eq!(AuditLogSubjectTable::Payments.to_string(), "payments");
    }
}
