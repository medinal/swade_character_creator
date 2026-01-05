use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attribute {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub base_value: i64,
    pub created_at: String,
    pub updated_at: String,
}
