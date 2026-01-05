use serde::{Deserialize, Serialize};

use crate::models::ShieldStats;

/// View model for shield statistics
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct ShieldStatsView {
    pub parry_bonus: i64,
    pub cover_penalty: i64,
    pub min_strength: Option<i64>,
    pub notes: Option<String>,
}

impl ShieldStatsView {
    pub fn new(stats: ShieldStats) -> Self {
        Self {
            parry_bonus: stats.parry_bonus,
            cover_penalty: stats.cover_penalty,
            min_strength: stats.min_strength,
            notes: stats.notes,
        }
    }
}
