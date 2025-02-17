use chrono::{DateTime, Utc};

pub struct LoanRowInsertion {
    pub user_id: i32,
    pub amount: f64,
    pub interest_rate: f64,
    pub term_months: i32,
    pub status: String,
    pub created_at: DateTime<Utc>,
}
