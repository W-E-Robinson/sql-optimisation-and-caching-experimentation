use chrono::{DateTime, Utc};

pub struct AccountRowInsertion {
    pub user_id: i32,
    pub account_type: String,
    pub balance: f64,
    pub created_at: DateTime<Utc>,
    pub num_active_cards: i32,
}
