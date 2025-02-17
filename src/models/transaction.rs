use chrono::{DateTime, Utc};

pub struct TransactionRowInsertion {
    pub account_id: i32,
    pub transaction_type: String,
    pub amount: f64,
    pub status: String,
    pub created_at: DateTime<Utc>,
}
