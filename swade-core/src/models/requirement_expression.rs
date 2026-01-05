use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequirementExpression {
    pub id: i64,
    pub parent_id: Option<i64>,
    pub node_type: String,
    pub requirement_id: Option<i64>,
    pub position: i64,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}
