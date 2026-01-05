use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArcaneBackground {
    pub id: i64,
    pub name: String,
    pub arcane_skill_id: i64,
    pub starting_powers: i64,
    pub starting_power_points: i64,
    pub has_power_list: bool,
    pub source: String,
    pub description: String,
    pub created_at: String,
    pub updated_at: String,
}
