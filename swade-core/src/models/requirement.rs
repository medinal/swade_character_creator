use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct Requirement {
    pub id: i64,
    pub requirement_type: String,
    pub target_id: Option<i64>,
    pub value: Option<i64>,
    pub description: String,
    pub created_at: String,
    pub updated_at: String,
}
