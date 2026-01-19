//! Shared types for commands.

use swade_core::views::CharacterView;

/// A warning generated when validation is bypassed.
#[derive(serde::Serialize, specta::Type)]
pub struct ValidationWarning {
    pub warning_type: String,
    pub message: String,
}

impl ValidationWarning {
    pub fn new(warning_type: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            warning_type: warning_type.into(),
            message: message.into(),
        }
    }

    pub fn requirement_not_met(message: impl Into<String>) -> Self {
        Self::new("requirement_not_met", message)
    }

    pub fn point_limit_exceeded(message: impl Into<String>) -> Self {
        Self::new("point_limit_exceeded", message)
    }

    pub fn slot_limit_exceeded(message: impl Into<String>) -> Self {
        Self::new("slot_limit_exceeded", message)
    }
}

/// Result returned by draft commands that support bypass_validation.
#[derive(serde::Serialize, specta::Type)]
pub struct DraftResult {
    pub character: CharacterView,
    pub warnings: Vec<ValidationWarning>,
}

impl DraftResult {
    pub fn new(character: CharacterView) -> Self {
        Self {
            character,
            warnings: vec![],
        }
    }

    pub fn with_warnings(character: CharacterView, warnings: Vec<ValidationWarning>) -> Self {
        Self { character, warnings }
    }
}
