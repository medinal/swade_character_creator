use serde::{Deserialize, Serialize};

use crate::views::PowerView;

/// Represents a power that a character has learned, along with when it was acquired.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct CharacterPowerValue {
    pub power: PowerView,
    pub advance_taken: Option<i64>,
    /// Whether this power is locked (e.g., required starting power from arcane background).
    /// Locked powers cannot be removed by the player.
    pub is_locked: bool,
}

impl CharacterPowerValue {
    pub fn new(power: PowerView, advance_taken: Option<i64>) -> Self {
        Self {
            power,
            advance_taken,
            is_locked: false,
        }
    }
}
