use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoisonStats {
    pub id: i64,
    pub gear_id: i64,
    pub poison_type: String,
    pub delivery_method: String,
    pub affected_attribute: Option<String>,
    pub notes: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}
