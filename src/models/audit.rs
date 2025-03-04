use crate::enums::audit_log_action::AuditLogAction;
use chrono::{DateTime, Utc};

pub struct AuditLogs {
    pub action: AuditLogAction,
    pub text: String,
    pub timestamp: DateTime<Utc>,
}
