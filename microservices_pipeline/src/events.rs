use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::Utc;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Event {
    pub id: Uuid,
    pub timestamp: i64,
    pub payload: String,
}

impl Event {
    pub fn new(payload: &str) -> Self {
        Self {
            id: Uuid::new_v4(),
            timestamp: Utc::now().timestamp(),
            payload: payload.to_string(),
        }
    }
}