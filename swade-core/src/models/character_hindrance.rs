use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterHindrance {
    pub id: i64,
    pub character_id: i64,
    pub hindrance_id: i64,
    pub source: String,
    pub created_at: String,
    pub updated_at: String,
}
