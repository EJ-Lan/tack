use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct CommandEntry {
    pub id: Uuid,
    pub command: String,
    pub tags: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub last_run: Option<DateTime<Utc>>
        ,
}
