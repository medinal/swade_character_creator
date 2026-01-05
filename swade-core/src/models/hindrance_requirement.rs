use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HindranceRequirement {
    pub id: i64,
    pub hindrance_id: i64,
    pub requirement_expression_id: i64,
    pub created_at: String,
    pub updated_at: String,
}
