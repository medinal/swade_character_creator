use serde::{Deserialize, Serialize};

/// Static game configuration constants for SWADE.
/// Provides a single source of truth for game rules that the frontend can query.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct GameConfig {
    /// Maximum hindrance points a character can earn (4)
    pub max_hindrance_points: i64,
    /// Base attribute points for character creation (5)
    pub base_attribute_points: i64,
    /// Base skill points for character creation (12)
    pub base_skill_points: i64,
    /// Hindrance points required per attribute point (2)
    pub hindrance_points_per_attribute: i64,
    /// Hindrance points required per skill point (1)
    pub hindrance_points_per_skill: i64,
    /// Hindrance points required per edge (2)
    pub hindrance_points_per_edge: i64,
    /// Skill cost when at or below linked attribute (1)
    pub skill_cost_at_or_below_attribute: i64,
    /// Skill cost when above linked attribute (2)
    pub skill_cost_above_attribute: i64,
}

impl GameConfig {
    /// Returns the default SWADE game configuration.
    pub fn swade_defaults() -> Self {
        Self {
            max_hindrance_points: 4,
            base_attribute_points: 5,
            base_skill_points: 12,
            hindrance_points_per_attribute: 2,
            hindrance_points_per_skill: 1,
            hindrance_points_per_edge: 2,
            skill_cost_at_or_below_attribute: 1,
            skill_cost_above_attribute: 2,
        }
    }
}

impl Default for GameConfig {
    fn default() -> Self {
        Self::swade_defaults()
    }
}
