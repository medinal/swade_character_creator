use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterEdge {
    pub id: i64,
    pub character_id: i64,
    pub edge_id: i64,
    pub advance_taken: i64,
    pub notes: Option<String>,
    pub source: String,
    pub created_at: String,
    pub updated_at: String,
}
