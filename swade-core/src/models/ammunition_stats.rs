use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AmmunitionStats {
    pub id: i64,
    pub gear_id: i64,
    pub ammo_type: String,
    pub quantity_per_unit: i64,
    pub notes: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}
