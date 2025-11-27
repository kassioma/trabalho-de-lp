// src/models/note.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Note {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub title: String,
    pub content: String,
    pub user_id: String,
    pub created_at: i64,
    pub updated_at: i64,
}

impl Note {
    pub fn new(title: String, content: String, user_id: String) -> Self {
        let timestamp = js_sys::Date::now() as i64;
        Self {
            id: None,
            title,
            content,
            user_id,
            created_at: timestamp,
            updated_at: timestamp,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub uid: String,
    pub email: String,
}
