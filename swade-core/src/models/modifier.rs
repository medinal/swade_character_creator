use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct Modifier {
    pub id: i64,
    pub target_type: Option<String>,
    pub target_identifier: Option<String>,
    pub value_type: String,
    pub value: Option<i64>,
    pub description: String,
    pub created_at: String,
    pub updated_at: String,
}
