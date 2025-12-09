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
    pub history: Vec<NoteHistory>,
    pub font: String,
    pub background: String,
    pub color: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_size: Option<u8>,
}

impl Note {
    pub fn new(title: String, content: String, user_id: String, font: String, color: String, background: String, font_size: Option<u8>) -> Self {
        let timestamp = js_sys::Date::now() as i64;
        Self {
            id: None,
            title: title.clone(),
            content: content.clone(),
            user_id,
            created_at: timestamp,
            updated_at: timestamp,
            history: vec![NoteHistory{title, content, updated_at: timestamp.clone()}],
            font: font.clone(),
            background: background.clone(),
            color: color.clone()
            ,
            font_size
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub uid: String,
    pub email: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NoteHistory{
    pub title: String,
    pub content: String,
    pub updated_at: i64,
}