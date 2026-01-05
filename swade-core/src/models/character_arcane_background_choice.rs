use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterArcaneBackgroundChoice {
    pub id: i64,
    pub character_id: i64,
    pub choice_id: i64,
    pub selected_option_id: i64,
    pub created_at: String,
    pub updated_at: String,
}
