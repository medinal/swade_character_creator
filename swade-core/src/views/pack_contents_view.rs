use serde::{Deserialize, Serialize};

use crate::views::GearView;

/// View model for a single item within a pack
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct PackContentsView {
    /// The gear item in this pack
    pub item: GearView,
    /// How many of this item the pack contains
    pub quantity: i64,
    /// Optional notes about this item in the pack context
    pub notes: Option<String>,
}

impl PackContentsView {
    pub fn new(item: GearView, quantity: i64, notes: Option<String>) -> Self {
        Self {
            item,
            quantity,
            notes,
        }
    }
}
