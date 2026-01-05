use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hindrance {
    pub id: i64,
    pub name: String,
    pub severity: String,
    pub point_value: i64,
    pub companion_hindrance_id: Option<i64>,
    pub source: String,
    pub description: String,
    pub created_at: String,
    pub updated_at: String,
}
