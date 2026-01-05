use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct Rank {
    pub id: i64,
    pub name: String,
    pub min_advances: i64,
    pub max_advances: Option<i64>,
    pub description: String,
    pub created_at: String,
    pub updated_at: String,
}
