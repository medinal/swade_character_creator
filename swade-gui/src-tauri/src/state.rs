//! Application state management for Tauri commands.

use std::sync::Mutex;
use swade_core::constants::{
    ATTRIBUTE_HINDRANCE_POINT_COST, EDGE_HINDRANCE_POINT_COST, SOURCE_HINDRANCE_POINTS,
};
use swade_core::db::Database;
use swade_core::views::CharacterView;
use tauri::State;

use crate::error::CommandError;

pub struct AppState {
    pub db: Database,
    pub draft_character: Option<CharacterView>,
}

impl AppState {
    pub fn draft(&self) -> Result<&CharacterView, CommandError> {
        self.draft_character
            .as_ref()
            .ok_or_else(|| CommandError::NoDraft("No draft character".to_string()))
    }

    pub fn draft_mut(&mut self) -> Result<&mut CharacterView, CommandError> {
        self.draft_character
            .as_mut()
            .ok_or_else(|| CommandError::NoDraft("No draft character to update".to_string()))
    }

    pub fn connection(&self) -> Result<rusqlite::Connection, CommandError> {
        self.db.connection().map_err(CommandError::from)
    }
}

/// Helper to lock the app state mutex with consistent error handling.
pub fn lock_state<'a>(
    state: &'a State<'a, Mutex<AppState>>,
) -> Result<std::sync::MutexGuard<'a, AppState>, CommandError> {
    state
        .lock()
        .map_err(|e| CommandError::State(format!("Lock poisoned: {}", e)))
}

/// Clear all arcane backgrounds and powers, refunding power points.
pub fn clear_arcane_backgrounds(draft: &mut CharacterView) {
    for ab in &draft.arcane_backgrounds {
        draft.power_points -= ab.arcane_background.starting_power_points;
    }
    if draft.power_points < 0 {
        draft.power_points = 0;
    }
    draft.arcane_backgrounds.clear();
    draft.powers.clear();
}

/// Calculate currently available (unallocated) hindrance points.
pub fn available_hindrance_points(draft: &CharacterView) -> i64 {
    let already_allocated = draft.hindrance_points_to_edges
        + (draft.hindrance_points_to_attributes * ATTRIBUTE_HINDRANCE_POINT_COST)
        + draft.hindrance_points_to_skills
        + draft.hindrance_points_to_wealth;
    draft.hindrance_points_earned - already_allocated
}

/// Remove edges from hindrance_points that no longer meet requirements.
/// Refunds the hindrance points for each removed edge.
pub fn remove_invalid_edges(draft: &mut CharacterView) {
    let ctx = draft.to_requirement_context();
    let mut edges_to_remove = Vec::new();
    for (idx, edge_value) in draft.edges.iter().enumerate() {
        if edge_value.source == SOURCE_HINDRANCE_POINTS
            && !edge_value.edge.requirements.evaluate(&ctx)
        {
            edges_to_remove.push(idx);
        }
    }
    // Remove in reverse order to preserve indices
    for idx in edges_to_remove.into_iter().rev() {
        draft.edges.remove(idx);
        draft.hindrance_points_to_edges -= EDGE_HINDRANCE_POINT_COST;
    }
}
