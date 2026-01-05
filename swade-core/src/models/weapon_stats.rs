use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeaponStats {
    pub id: i64,
    pub gear_id: i64,
    pub damage: String,
    pub ap: i64,
    pub range_short: Option<i64>,
    pub range_medium: Option<i64>,
    pub range_long: Option<i64>,
    pub rof: Option<i64>,
    pub shots: Option<i64>,
    pub min_strength: Option<i64>,
    pub is_two_handed: bool,
    pub reach: Option<i64>,
    pub blast_template: Option<String>,
    pub notes: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}
