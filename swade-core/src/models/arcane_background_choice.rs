use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArcaneBackgroundChoice {
    pub id: i64,
    pub arcane_background_id: i64,
    pub choice_type: String,
    pub choice_category: Option<String>,
    pub min_selections: i64,
    pub max_selections: i64,
    pub description: String,
    pub position: i64,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArcaneBackgroundChoiceOption {
    pub id: i64,
    pub choice_id: i64,
    pub option_type: String,
    pub option_id: Option<i64>,
    pub option_description: Option<String>,
    pub position: i64,
    pub created_at: String,
    pub updated_at: String,
}
