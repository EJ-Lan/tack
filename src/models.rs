use chrono::{DateTime, Utc};
use uuid::Uuid;

pub enum BookmarkKind {
    Command {
        command: String,
    },
    Path {
        path: String,
    }
}

pub struct BookmarkEntry {
    pub uuid: Uuid,
    pub name: String, // nickname (required)
    pub kind: BookmarkKind,
    pub tags: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub last_used: Option<DateTime<Utc>>,
}