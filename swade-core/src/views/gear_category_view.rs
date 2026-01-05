use serde::{Deserialize, Serialize};

use crate::models::GearCategory;

/// View model for a gear category
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct GearCategoryView {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
}

impl GearCategoryView {
    pub fn new(category: GearCategory) -> Self {
        Self {
            id: category.id,
            name: category.name,
            description: category.description,
        }
    }
}
