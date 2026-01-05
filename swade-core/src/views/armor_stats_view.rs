use serde::{Deserialize, Serialize};

use crate::models::ArmorStats;

/// View model for armor statistics
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct ArmorStatsView {
    pub armor_value: i64,
    pub coverage: String,
    pub min_strength: Option<i64>,
    pub is_heavy: bool,
}

impl ArmorStatsView {
    pub fn new(stats: ArmorStats) -> Self {
        Self {
            armor_value: stats.armor_value,
            coverage: stats.coverage,
            min_strength: stats.min_strength,
            is_heavy: stats.is_heavy,
        }
    }
}
