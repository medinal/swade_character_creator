use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Power {
    pub id: i64,
    pub name: String,
    pub power_points: i64,
    pub range: String,
    pub duration: String,
    pub source: String,
    pub description: String,
    pub created_at: String,
    pub updated_at: String,
}
