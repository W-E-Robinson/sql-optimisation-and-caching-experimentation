use core::panic;

#[derive(PartialEq)]
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

    pub fn from_string(action: &str) -> Self {
        match action {
            "user created" => Self::UserCreated,
            "user updated" => Self::UserUpdated,
            "user deleted" => Self::UserDeleted,
            "transfer created" => Self::TransferCreated,
            "transfer updated" => Self::TransferUpdated,
            "transaction created" => Self::TransactionCreated,
            "transaction deleted" => Self::TransactionDeleted,
            "payment created" => Self::PaymentCreated,
            "payment updated" => Self::PaymentUpdated,
            "loan created" => Self::LoanCreated,
            "loan updated" => Self::LoanUpdated,
            "card created" => Self::CardCreated,
            "card updated" => Self::CardUpdated,
            "card deleted" => Self::CardDeleted,
            "account created" => Self::AccountCreated,
            "account updated" => Self::AccountUpdated,
            "account deleted" => Self::AccountDeleted,
            _ => panic!(
                "Error: AuditLogAction not found from action - <action = {}>",
                action
            ), // NOTE: or None?
        }
    }
}
