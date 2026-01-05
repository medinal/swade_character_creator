use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Character {
    pub id: i64,
    pub is_wild_card: bool,
    pub name: String,
    pub ancestry_id: Option<i64>,

    // Character Creation Tracking
    pub attribute_points_spent: i64,
    pub attribute_points_earned: i64,
    pub skill_points_spent: i64,
    pub skill_points_earned: i64,
    pub hindrance_points_spent: i64,
    pub hindrance_points_earned: i64,

    // Hindrance Points Conversion Tracking
    pub hindrance_points_to_edges: i64,
    pub hindrance_points_to_attributes: i64,
    pub hindrance_points_to_skills: i64,
    pub hindrance_points_to_wealth: i64,

    // Additional Character Info
    pub power_points: i64,
    pub power_points_used: i64,
    pub wounds: i64,
    pub fatigue: i64,
    pub wealth: i64,
    pub background: Option<String>,
    pub description: Option<String>,

    // Portrait (stored as resized image, max 1024x1024)
    pub portrait: Option<Vec<u8>>,
    pub portrait_mime_type: Option<String>,

    pub created_at: String,
    pub updated_at: String,
}
