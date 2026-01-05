use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HindranceModifier {
    pub id: i64,
    pub hindrance_id: i64,
    pub modifier_id: i64,
    pub created_at: String,
    pub updated_at: String,
}
