use chrono::{DateTime, Utc};

pub struct TransferRowInsertion {
    pub sender_account_id: i32,
    pub receiver_account_id: i32,
    pub amount: f64,
    pub status: String,
    pub created_at: DateTime<Utc>,
}
