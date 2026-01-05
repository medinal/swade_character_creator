use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgeModifier {
    pub id: i64,
    pub edge_id: i64,
    pub modifier_id: i64,
    pub created_at: String,
    pub updated_at: String,
}
