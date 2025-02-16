use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct UserRowInsertion {
    pub public_id: Uuid,
    pub given_name: String,
    pub family_name: String,
    pub username: String,
    pub email: String,
    pub phone: String,
    pub created_at: DateTime<Utc>,
}
