use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Edge {
    pub id: i64,
    pub name: String,
    pub background: String,
    pub source: String,
    pub description: String,
    pub can_take_multiple_times: bool,
    pub created_at: String,
    pub updated_at: String,
}
