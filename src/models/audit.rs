use chrono::{DateTime, Utc};
use crate::enums::audit_log_action::AuditLogAction;

pub struct AuditLogs {
    pub action: AuditLogAction,
    pub text: String,
    pub timestamp: DateTime<Utc>,
}
