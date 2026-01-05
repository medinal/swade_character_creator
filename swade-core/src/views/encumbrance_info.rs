use serde::{Deserialize, Serialize};

/// Information about a character's encumbrance status
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct EncumbranceInfo {
    /// Current total weight carried
    pub current_weight: f64,

    /// Maximum weight before encumbrance penalty
    pub load_limit: f64,

    /// Whether the character is encumbered
    pub is_encumbered: bool,

    /// Penalty applied if encumbered (-2 to all physical tasks)
    pub encumbrance_penalty: i64,
}

impl EncumbranceInfo {
    /// Create encumbrance info from current weight and strength die size
    pub fn from_weight_and_strength(current_weight: f64, strength_die_size: u8) -> Self {
        // SWADE encumbrance table based on Strength die
        let load_limit = match strength_die_size {
            4 => 20.0,
            6 => 40.0,
            8 => 60.0,
            10 => 80.0,
            12 => 100.0,
            // d12+N (die size stored as 12, modifier adds 20 per step beyond)
            _ => 100.0 + ((strength_die_size.saturating_sub(12)) as f64 * 20.0),
        };

        let is_encumbered = current_weight > load_limit;
        let encumbrance_penalty = if is_encumbered { 2 } else { 0 };

        Self {
            current_weight,
            load_limit,
            is_encumbered,
            encumbrance_penalty,
        }
    }

    /// Create empty encumbrance info (no gear)
    pub fn empty(strength_die_size: u8) -> Self {
        Self::from_weight_and_strength(0.0, strength_die_size)
    }
}
