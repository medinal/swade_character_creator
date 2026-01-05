use serde::{Deserialize, Serialize};

use crate::views::{AncestryChoiceOptionView, AncestryChoiceView};

/// Represents a character's selection for an ancestry choice.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct CharacterAncestryChoiceValue {
    pub choice: AncestryChoiceView,
    pub selected_option: Option<AncestryChoiceOptionView>,
}

impl CharacterAncestryChoiceValue {
    pub fn new(
        choice: AncestryChoiceView,
        selected_option: Option<AncestryChoiceOptionView>,
    ) -> Self {
        Self {
            choice,
            selected_option,
        }
    }
}
