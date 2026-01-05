use serde::{Deserialize, Serialize};

use crate::views::HindranceView;

/// Represents a hindrance that a character has taken, along with metadata about how it was acquired.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct CharacterHindranceValue {
    pub hindrance: HindranceView,
    pub source: String,
}

impl CharacterHindranceValue {
    pub fn new(hindrance: HindranceView, source: String) -> Self {
        Self { hindrance, source }
    }
}
