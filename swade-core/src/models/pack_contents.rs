use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackContents {
    pub id: i64,
    pub pack_gear_id: i64,
    pub item_gear_id: i64,
    pub quantity: i64,
    pub notes: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}
