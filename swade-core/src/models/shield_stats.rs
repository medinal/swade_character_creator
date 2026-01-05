use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShieldStats {
    pub id: i64,
    pub gear_id: i64,
    pub parry_bonus: i64,
    pub cover_penalty: i64,
    pub min_strength: Option<i64>,
    pub notes: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}
