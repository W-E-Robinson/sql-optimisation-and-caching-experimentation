use chrono::{DateTime, Utc};

pub struct PaymentRowInsertion {
    pub account_id: i32,
    pub loan_id: i32,
    pub amount: f64,
    pub status: String,
    pub created_at: DateTime<Utc>,
}
