use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub linked_attribute_id: i64,
    pub is_core_skill: bool,
    pub default_die_size: Option<i64>,
    pub max_die_size: i64,
    pub max_die_modifier: i64,
    pub source: String,
    pub created_at: String,
    pub updated_at: String,
}
