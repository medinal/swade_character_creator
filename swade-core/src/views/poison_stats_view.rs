use serde::{Deserialize, Serialize};

use crate::models::PoisonStats;

/// View model for poison statistics
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct PoisonStatsView {
    pub poison_type: String,
    pub delivery_method: String,
    pub affected_attribute: Option<String>,
    pub notes: Option<String>,
}

impl PoisonStatsView {
    pub fn new(stats: PoisonStats) -> Self {
        Self {
            poison_type: stats.poison_type,
            delivery_method: stats.delivery_method,
            affected_attribute: stats.affected_attribute,
            notes: stats.notes,
        }
    }
}
