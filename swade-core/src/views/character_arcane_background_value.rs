use serde::{Deserialize, Serialize};

use crate::views::ArcaneBackgroundView;

/// Represents an arcane background that a character has, along with when it was acquired.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct CharacterArcaneBackgroundValue {
    pub arcane_background: ArcaneBackgroundView,
    pub advance_taken: Option<i64>,
}

impl CharacterArcaneBackgroundValue {
    pub fn new(arcane_background: ArcaneBackgroundView, advance_taken: Option<i64>) -> Self {
        Self {
            arcane_background,
            advance_taken,
        }
    }
}
