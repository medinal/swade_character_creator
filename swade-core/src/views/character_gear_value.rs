use serde::{Deserialize, Serialize};

use crate::views::GearView;

/// Represents a gear item owned by a character
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct CharacterGearValue {
    pub id: i64,
    pub gear: GearView,
    pub quantity: i64,
    pub is_equipped: bool,
    pub custom_notes: Option<String>,
    /// Total weight of this gear stack (weight * quantity)
    pub total_weight: f64,
}

impl CharacterGearValue {
    pub fn new(
        id: i64,
        gear: GearView,
        quantity: i64,
        is_equipped: bool,
        custom_notes: Option<String>,
    ) -> Self {
        let total_weight = gear.weight * quantity as f64;
        Self {
            id,
            gear,
            quantity,
            is_equipped,
            custom_notes,
            total_weight,
        }
    }

    /// Calculate the total weight of this gear (weight * quantity)
    pub fn total_weight(&self) -> f64 {
        self.gear.weight * self.quantity as f64
    }

    /// Calculate the total cost of this gear (cost * quantity)
    pub fn total_cost(&self) -> i64 {
        self.gear.cost * self.quantity
    }
}
