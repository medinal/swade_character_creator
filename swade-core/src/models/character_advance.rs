use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterAdvance {
    pub id: i64,
    pub character_id: i64,
    pub advance_number: i64,
    pub advance_type: String, // 'edge', 'attribute', 'skill_expensive', 'skill_cheap', 'hindrance'
    pub edge_id: Option<i64>,
    pub attribute_id: Option<i64>,
    pub skill_id_1: Option<i64>,
    pub skill_id_2: Option<i64>,
    pub hindrance_id: Option<i64>,
    pub hindrance_action: Option<String>, // 'remove_minor', 'reduce_major', 'remove_major_half'
    pub notes: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}
