use serde::{Deserialize, Serialize};

use crate::models::Modifier;
use crate::views::RequirementTree;

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct PowerView {
    pub id: i64,
    pub name: String,
    pub power_points: i64,
    pub range: String,
    pub duration: String,
    pub source: String,
    pub description: String,
    pub modifiers: Vec<Modifier>,
    pub requirements: RequirementTree,
}
