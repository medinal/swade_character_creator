use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterSkill {
    pub id: i64,
    pub character_id: i64,
    pub skill_id: i64,
    pub current_die_size: Option<i64>,
    pub current_die_modifier: i64,
    pub created_at: String,
    pub updated_at: String,
}
