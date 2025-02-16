#[derive(Debug, PartialEq)]
pub enum AuditLogAction {
    UserCreated,
    UserUpdated,
    UserDeleted,
    TransferCreated,
    TransferUpdated,
    TransactionCreated,
    TransactionDeleted,
    PaymentCreated,
    PaymentUpdated,
    LoanCreated,
    LoanUpdated,
    CardCreated,
    CardUpdated,
    CardDeleted,
    AccountCreated,
    AccountUpdated,
    AccountDeleted,
}

impl AuditLogAction {
    pub fn to_string(&self) -> &'static str {
        match self {
            Self::UserCreated => "user created",
            Self::UserUpdated => "user updated",
            Self::UserDeleted => "user deleted",
            Self::TransferCreated => "transfer created",
            Self::TransferUpdated => "transfer updated",
            Self::TransactionCreated => "transaction created",
            Self::TransactionDeleted => "transaction deleted",
            Self::PaymentCreated => "payment created",
            Self::PaymentUpdated => "payment updated",
            Self::LoanCreated => "loan created",
            Self::LoanUpdated => "loan updated",
            Self::CardCreated => "card created",
            Self::CardUpdated => "card updated",
            Self::CardDeleted => "card deleted",
            Self::AccountCreated => "account created",
            Self::AccountUpdated => "account updated",
            Self::AccountDeleted => "account deleted",
        }
    }

    pub fn from_string(action: &str) -> Option<Self> {
        match action {
            "user created" => Some(Self::UserCreated),
            "user updated" => Some(Self::UserUpdated),
            "user deleted" => Some(Self::UserDeleted),
            "transfer created" => Some(Self::TransferCreated),
            "transfer updated" => Some(Self::TransferUpdated),
            "transaction created" => Some(Self::TransactionCreated),
            "transaction deleted" => Some(Self::TransactionDeleted),
            "payment created" => Some(Self::PaymentCreated),
            "payment updated" => Some(Self::PaymentUpdated),
            "loan created" => Some(Self::LoanCreated),
            "loan updated" => Some(Self::LoanUpdated),
            "card created" => Some(Self::CardCreated),
            "card updated" => Some(Self::CardUpdated),
            "card deleted" => Some(Self::CardDeleted),
            "account created" => Some(Self::AccountCreated),
            "account updated" => Some(Self::AccountUpdated),
            "account deleted" => Some(Self::AccountDeleted),
            _ => {
                println!(
                    "Error: AuditLogAction not found from action - <action = {}>",
                    action
                );
                None
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_audit_log_action_to_string_user_created() {
        assert_eq!(AuditLogAction::UserCreated.to_string(), "user created");
    }

    fn test_audit_log_action_to_string_user_updated() {
        assert_eq!(AuditLogAction::UserUpdated.to_string(), "user updated");
    }

    fn test_audit_log_action_to_string_user_deleted() {
        assert_eq!(AuditLogAction::UserDeleted.to_string(), "user deleted");
    }

    fn test_audit_log_action_to_string_transfer_created() {
        assert_eq!(
            AuditLogAction::TransferCreated.to_string(),
            "transfer created"
        );
    }

    fn test_audit_log_action_to_string_transfer_updated() {
        assert_eq!(
            AuditLogAction::TransferUpdated.to_string(),
            "transfer updated"
        );
    }

    fn test_audit_log_action_to_string_transaction_created() {
        assert_eq!(
            AuditLogAction::TransactionCreated.to_string(),
            "transaction created"
        );
    }

    fn test_audit_log_action_to_string_transaction_deleted() {
        assert_eq!(
            AuditLogAction::TransactionDeleted.to_string(),
            "transaction deleted"
        );
    }

    fn test_audit_log_action_to_string_payment_created() {
        assert_eq!(
            AuditLogAction::PaymentCreated.to_string(),
            "payment created"
        );
    }

    fn test_audit_log_action_to_string_payment_updated() {
        assert_eq!(
            AuditLogAction::PaymentUpdated.to_string(),
            "payment updated"
        );
    }

    fn test_audit_log_action_to_string_loan_created() {
        assert_eq!(AuditLogAction::LoanCreated.to_string(), "loan created");
    }

    fn test_audit_log_action_to_string_loan_updated() {
        assert_eq!(AuditLogAction::LoanUpdated.to_string(), "loan updated");
    }

    fn test_audit_log_action_to_string_card_created() {
        assert_eq!(AuditLogAction::CardCreated.to_string(), "card created");
    }

    fn test_audit_log_action_to_string_card_updated() {
        assert_eq!(AuditLogAction::CardUpdated.to_string(), "card updated");
    }

    fn test_audit_log_action_to_string_card_deleted() {
        assert_eq!(AuditLogAction::CardDeleted.to_string(), "card deleted");
    }

    fn test_audit_log_action_to_string_account_created() {
        assert_eq!(
            AuditLogAction::AccountCreated.to_string(),
            "account created"
        );
    }

    fn test_audit_log_action_to_string_account_updated() {
        assert_eq!(
            AuditLogAction::AccountUpdated.to_string(),
            "account updated"
        );
    }

    fn test_audit_log_action_to_string_account_deleted() {
        assert_eq!(
            AuditLogAction::AccountDeleted.to_string(),
            "account deleted"
        );
    }

    fn test_audit_log_action_from_string_user_updated() {
        assert_eq!(
            AuditLogAction::from_string("user updated"),
            Some(AuditLogAction::UserUpdated),
        );
    }

    fn test_audit_log_action_from_string_user_deleted() {
        assert_eq!(
            AuditLogAction::from_string("user deleted"),
            Some(AuditLogAction::UserDeleted),
        );
    }

    fn test_audit_log_action_from_string_transfer_created() {
        assert_eq!(
            AuditLogAction::from_string("transfer created"),
            Some(AuditLogAction::TransferCreated),
        );
    }

    fn test_audit_log_action_from_string_transfer_updated() {
        assert_eq!(
            AuditLogAction::from_string("transfer updated"),
            Some(AuditLogAction::TransferUpdated),
        );
    }

    fn test_audit_log_action_from_string_transaction_created() {
        assert_eq!(
            AuditLogAction::from_string("transaction created"),
            Some(AuditLogAction::TransactionCreated),
        );
    }

    fn test_audit_log_action_from_string_transaction_deleted() {
        assert_eq!(
            AuditLogAction::from_string("transaction deleted"),
            Some(AuditLogAction::TransactionDeleted),
        );
    }

    fn test_audit_log_action_from_string_payment_created() {
        assert_eq!(
            AuditLogAction::from_string("payment created"),
            Some(AuditLogAction::PaymentCreated),
        );
    }

    fn test_audit_log_action_from_string_payment_updated() {
        assert_eq!(
            AuditLogAction::from_string("payment updated"),
            Some(AuditLogAction::PaymentUpdated),
        );
    }

    fn test_audit_log_action_from_string_loan_created() {
        assert_eq!(
            AuditLogAction::from_string("loan created"),
            Some(AuditLogAction::LoanCreated),
        );
    }

    fn test_audit_log_action_from_string_loan_updated() {
        assert_eq!(
            AuditLogAction::from_string("loan updated"),
            Some(AuditLogAction::LoanUpdated),
        );
    }

    fn test_audit_log_action_from_string_card_created() {
        assert_eq!(
            AuditLogAction::from_string("card created"),
            Some(AuditLogAction::CardCreated),
        );
    }

    fn test_audit_log_action_from_string_card_updated() {
        assert_eq!(
            AuditLogAction::from_string("card updated"),
            Some(AuditLogAction::CardUpdated),
        );
    }

    fn test_audit_log_action_from_string_card_deleted() {
        assert_eq!(
            AuditLogAction::from_string("card deleted"),
            Some(AuditLogAction::CardDeleted),
        );
    }

    fn test_audit_log_action_from_string_account_created() {
        assert_eq!(
            AuditLogAction::from_string("account created"),
            Some(AuditLogAction::AccountCreated),
        );
    }

    fn test_audit_log_action_from_string_account_updated() {
        assert_eq!(
            AuditLogAction::from_string("account updated"),
            Some(AuditLogAction::AccountUpdated),
        );
    }

    fn test_audit_log_action_from_string_account_deleted() {
        assert_eq!(
            AuditLogAction::from_string("account deleted"),
            Some(AuditLogAction::AccountDeleted),
        );
    }

    fn test_audit_log_action_from_string_not_found() {
        assert_eq!(
            AuditLogAction::from_string("not found"),
            None,
        );
    }
}
