use serde::{Deserialize, Serialize};

use crate::models::WeaponStats;

/// View model for weapon statistics
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct WeaponStatsView {
    pub damage: String,
    pub ap: i64,
    pub range_short: Option<i64>,
    pub range_medium: Option<i64>,
    pub range_long: Option<i64>,
    pub rof: Option<i64>,
    pub shots: Option<i64>,
    pub min_strength: Option<i64>,
    pub is_two_handed: bool,
    pub reach: Option<i64>,
    pub blast_template: Option<String>,
    pub notes: Option<String>,
}

impl WeaponStatsView {
    pub fn new(stats: WeaponStats) -> Self {
        Self {
            damage: stats.damage,
            ap: stats.ap,
            range_short: stats.range_short,
            range_medium: stats.range_medium,
            range_long: stats.range_long,
            rof: stats.rof,
            shots: stats.shots,
            min_strength: stats.min_strength,
            is_two_handed: stats.is_two_handed,
            reach: stats.reach,
            blast_template: stats.blast_template,
            notes: stats.notes,
        }
    }

    /// Check if this is a ranged weapon (has range values)
    pub fn is_ranged(&self) -> bool {
        self.range_short.is_some()
    }

    /// Check if this is a melee weapon (no range values)
    pub fn is_melee(&self) -> bool {
        self.range_short.is_none()
    }

    /// Format range as "short/medium/long" string
    pub fn range_string(&self) -> Option<String> {
        match (self.range_short, self.range_medium, self.range_long) {
            (Some(s), Some(m), Some(l)) => Some(format!("{}/{}/{}", s, m, l)),
            _ => None,
        }
    }
}
