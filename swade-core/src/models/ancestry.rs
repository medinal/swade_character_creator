use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ancestry {
    pub id: i64,
    pub name: String,
    pub source: String,
    pub description: String,
    pub created_at: String,
    pub updated_at: String,
}
