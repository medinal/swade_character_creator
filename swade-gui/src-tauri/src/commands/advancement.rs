//! Advancement commands for character progression.

use std::sync::Mutex;
use swade_core::services::AdvancementService;
use swade_core::views::{AdvancementOptions, CharacterAdvanceValue};
use tauri::State;

use crate::error::CommandResult;
use crate::state::{lock_state, AppState};

/// Get the available advancement options for a character.
#[tauri::command]
#[specta::specta]
pub fn get_advancement_options(
    character_id: i64,
    state: State<Mutex<AppState>>,
) -> CommandResult<AdvancementOptions> {
    let state = lock_state(&state)?;
    let conn = state.connection()?;
    Ok(AdvancementService::get_advancement_options(&conn, character_id)?)
}

/// Take an edge as an advancement.
#[tauri::command]
#[specta::specta]
pub fn take_edge_advance(
    character_id: i64,
    edge_id: i64,
    notes: Option<String>,
    state: State<Mutex<AppState>>,
) -> CommandResult<CharacterAdvanceValue> {
    let state = lock_state(&state)?;
    let conn = state.connection()?;
    Ok(AdvancementService::apply_edge_advance(
        &conn,
        character_id,
        edge_id,
        notes,
    )?)
}

/// Take an attribute increase as an advancement.
#[tauri::command]
#[specta::specta]
pub fn take_attribute_advance(
    character_id: i64,
    attribute_id: i64,
    state: State<Mutex<AppState>>,
) -> CommandResult<CharacterAdvanceValue> {
    let state = lock_state(&state)?;
    let conn = state.connection()?;
    Ok(AdvancementService::apply_attribute_advance(
        &conn,
        character_id,
        attribute_id,
    )?)
}

/// Take an expensive skill increase (one skill at or above linked attribute) as an advancement.
#[tauri::command]
#[specta::specta]
pub fn take_expensive_skill_advance(
    character_id: i64,
    skill_id: i64,
    state: State<Mutex<AppState>>,
) -> CommandResult<CharacterAdvanceValue> {
    let state = lock_state(&state)?;
    let conn = state.connection()?;
    Ok(AdvancementService::apply_expensive_skill_advance(
        &conn,
        character_id,
        skill_id,
    )?)
}

/// Take two cheap skill increases (skills below linked attribute) as an advancement.
#[tauri::command]
#[specta::specta]
pub fn take_cheap_skill_advance(
    character_id: i64,
    skill_id_1: i64,
    skill_id_2: i64,
    state: State<Mutex<AppState>>,
) -> CommandResult<CharacterAdvanceValue> {
    let state = lock_state(&state)?;
    let conn = state.connection()?;
    Ok(AdvancementService::apply_cheap_skill_advance(
        &conn,
        character_id,
        skill_id_1,
        skill_id_2,
    )?)
}

/// Take a hindrance modification as an advancement.
/// Actions: "remove_minor", "reduce_major", "remove_major_half"
#[tauri::command]
#[specta::specta]
pub fn take_hindrance_advance(
    character_id: i64,
    hindrance_id: i64,
    action: String,
    state: State<Mutex<AppState>>,
) -> CommandResult<CharacterAdvanceValue> {
    let state = lock_state(&state)?;
    let conn = state.connection()?;
    Ok(AdvancementService::apply_hindrance_advance(
        &conn,
        character_id,
        hindrance_id,
        &action,
    )?)
}

/// Undo the most recent advancement for a character.
/// Returns true if an advance was successfully undone.
#[tauri::command]
#[specta::specta]
pub fn undo_last_advance(
    character_id: i64,
    state: State<Mutex<AppState>>,
) -> CommandResult<bool> {
    let state = lock_state(&state)?;
    let conn = state.connection()?;
    Ok(AdvancementService::undo_advance(&conn, character_id)?)
}

/// Get the advancement history for a character.
#[tauri::command]
#[specta::specta]
pub fn get_advancement_history(
    character_id: i64,
    state: State<Mutex<AppState>>,
) -> CommandResult<Vec<CharacterAdvanceValue>> {
    let state = lock_state(&state)?;
    let conn = state.connection()?;
    Ok(AdvancementService::get_advancement_history(&conn, character_id)?)
}
