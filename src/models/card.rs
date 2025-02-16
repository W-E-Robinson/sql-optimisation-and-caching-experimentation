use chrono::{DateTime, Utc};

pub struct CardRowInsertion {
    pub account_id: i32,
    pub card_number: unknown,
    pub card_type: String,
    pub expiration_date: DateTime<Utc>,
    pub status: String,
}
