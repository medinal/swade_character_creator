use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PowerRequirement {
    pub id: i64,
    pub power_id: i64,
    pub requirement_expression_id: i64,
    pub created_at: String,
    pub updated_at: String,
}
