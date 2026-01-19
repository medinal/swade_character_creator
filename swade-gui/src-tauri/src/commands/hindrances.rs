//! Hindrance selection commands.

use std::sync::Mutex;
use swade_core::constants::SOURCE_CHOSEN;
use swade_core::services::HindranceService;
use swade_core::views::{CharacterHindranceValue, CharacterView, HindranceView};
use tauri::State;

use crate::error::{CommandError, CommandResult};
use crate::state::{lock_state, AppState};

use super::types::{DraftResult, ValidationWarning};

#[tauri::command]
#[specta::specta]
pub fn get_hindrances(state: State<Mutex<AppState>>) -> CommandResult<Vec<HindranceView>> {
    let state = lock_state(&state)?;
    let conn = state.connection()?;
    Ok(HindranceService::get_all(&conn)?)
}

#[tauri::command]
#[specta::specta]
pub fn add_draft_hindrance(
    hindrance_id: i64,
    bypass_validation: Option<bool>,
    state: State<Mutex<AppState>>,
) -> CommandResult<DraftResult> {
    let mut state = lock_state(&state)?;
    let conn = state.connection()?;
    let bypass = bypass_validation.unwrap_or(false);
    let mut warnings = Vec::new();

    let draft = state.draft_mut()?;

    // Check if hindrance is already added
    if draft
        .hindrances
        .iter()
        .any(|h| h.hindrance.id == hindrance_id)
    {
        return Err(CommandError::Validation("Hindrance already added".to_string()));
    }

    // Load the hindrance
    let hindrance = HindranceService::get_by_id(&conn, hindrance_id)?
        .ok_or_else(|| CommandError::NotFound("Hindrance not found".to_string()))?;

    // Check point limit (max 4 hindrance points typically)
    let max_points = 4i64; // Could come from GameConfig
    let new_total = draft.hindrance_points_earned + hindrance.point_value;
    if new_total > max_points {
        if bypass {
            warnings.push(ValidationWarning::point_limit_exceeded(format!(
                "Hindrance points exceed maximum ({}/{})",
                new_total, max_points
            )));
        }
        // Note: We allow this even without bypass since it was previously allowed
    }

    // Add to draft and update points
    let point_value = hindrance.point_value;
    draft
        .hindrances
        .push(CharacterHindranceValue::new(hindrance, SOURCE_CHOSEN.to_string()));
    draft.hindrance_points_earned += point_value;

    // Recompute effective values (hindrances can have die modifiers)
    draft.compute_effective_values();

    Ok(DraftResult::with_warnings(draft.clone(), warnings))
}

#[tauri::command]
#[specta::specta]
pub fn remove_draft_hindrance(
    hindrance_id: i64,
    state: State<Mutex<AppState>>,
) -> CommandResult<CharacterView> {
    let mut state = lock_state(&state)?;

    let draft = state.draft_mut()?;

    // Find and remove the hindrance
    let position = draft
        .hindrances
        .iter()
        .position(|h| h.hindrance.id == hindrance_id && h.source == SOURCE_CHOSEN);

    match position {
        Some(idx) => {
            let removed = draft.hindrances.remove(idx);
            draft.hindrance_points_earned -= removed.hindrance.point_value;

            // Recompute effective values (hindrances can have die modifiers)
            draft.compute_effective_values();

            Ok(draft.clone())
        }
        None => Err(CommandError::NotFound("Hindrance not found or cannot be removed".to_string())),
    }
}
