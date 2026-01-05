use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterNote {
    pub id: i64,
    pub character_id: i64,
    pub title: String,
    pub body: String,
    pub created_at: String,
    pub updated_at: String,
}
