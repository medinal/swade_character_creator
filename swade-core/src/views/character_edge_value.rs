use serde::{Deserialize, Serialize};

use crate::views::EdgeView;

/// Represents an edge that a character has taken, along with metadata about when/how it was acquired.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct CharacterEdgeValue {
    pub edge: EdgeView,
    pub advance_taken: i64,
    pub notes: Option<String>,
    pub source: String,
}

impl CharacterEdgeValue {
    pub fn new(edge: EdgeView, advance_taken: i64, notes: Option<String>, source: String) -> Self {
        Self {
            edge,
            advance_taken,
            notes,
            source,
        }
    }
}
