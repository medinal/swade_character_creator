use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterAttribute {
    pub id: i64,
    pub character_id: i64,
    pub attribute_id: i64,
    pub steps_incremented: i64,
    pub created_at: String,
    pub updated_at: String,
}
