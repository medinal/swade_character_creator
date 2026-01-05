use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GearModifier {
    pub id: i64,
    pub gear_id: i64,
    pub modifier_id: i64,
    pub created_at: String,
    pub updated_at: String,
}
