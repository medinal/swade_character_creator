use serde::{Deserialize, Serialize};

/// Represents a note attached to a character.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct CharacterNoteValue {
    pub id: i64,
    pub title: String,
    pub body: String,
    pub created_at: String,
    pub updated_at: String,
}

impl CharacterNoteValue {
    pub fn new(id: i64, title: String, body: String, created_at: String, updated_at: String) -> Self {
        Self {
            id,
            title,
            body,
            created_at,
            updated_at,
        }
    }
}
