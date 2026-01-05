use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Gear {
    pub id: i64,
    pub name: String,
    pub category_id: i64,
    pub era: String,
    pub cost: i64,
    pub weight: f64,
    pub source: String,
    pub notes: Option<String>,
    pub description: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}
