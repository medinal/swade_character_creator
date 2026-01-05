use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArmorStats {
    pub id: i64,
    pub gear_id: i64,
    pub armor_value: i64,
    pub coverage: String,
    pub min_strength: Option<i64>,
    pub is_heavy: bool,
    pub created_at: String,
    pub updated_at: String,
}
