use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterGear {
    pub id: i64,
    pub character_id: i64,
    pub gear_id: i64,
    pub quantity: i64,
    pub is_equipped: bool,
    pub custom_notes: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}
