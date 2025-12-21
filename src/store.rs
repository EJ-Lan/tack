use std::collections::HashMap;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct TackDb {
    pub entries: HashMap<Uuid, BookmarkEntry>,
    pub name_index: HashMap<String, Uuid>, // normalized_name -> uuid
}

// Normalize the name by removing whitespace and converting to lowercase
fn normalize_name(name: &str) -> String {
    name.trim().to_lowercase()
}