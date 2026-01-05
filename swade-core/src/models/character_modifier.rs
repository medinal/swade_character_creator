use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterModifier {
    pub id: i64,
    pub character_id: i64,
    pub modifier_id: i64,
    pub advance_taken: Option<i64>,
    pub created_at: String,
    pub updated_at: String,
}
