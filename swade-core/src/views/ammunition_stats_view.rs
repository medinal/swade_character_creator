use serde::{Deserialize, Serialize};

use crate::models::AmmunitionStats;

/// View model for ammunition statistics
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct AmmunitionStatsView {
    pub ammo_type: String,
    pub quantity_per_unit: i64,
    pub notes: Option<String>,
}

impl AmmunitionStatsView {
    pub fn new(stats: AmmunitionStats) -> Self {
        Self {
            ammo_type: stats.ammo_type,
            quantity_per_unit: stats.quantity_per_unit,
            notes: stats.notes,
        }
    }
}
