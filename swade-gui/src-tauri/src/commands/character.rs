//! Character CRUD and draft management commands.

use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use std::sync::Mutex;
use swade_core::repositories::CharacterRepository;
use swade_core::services::CharacterService;
use swade_core::views::CharacterView;
use tauri::State;

use crate::error::{CommandError, CommandResult};
use crate::state::{lock_state, AppState};

#[tauri::command]
#[specta::specta]
pub fn get_characters(state: State<Mutex<AppState>>) -> CommandResult<Vec<CharacterView>> {
    let state = lock_state(&state)?;
    let conn = state.connection()?;
    Ok(CharacterService::get_all(&conn)?)
}

#[tauri::command]
#[specta::specta]
pub fn get_character(id: i64, state: State<Mutex<AppState>>) -> CommandResult<Option<CharacterView>> {
    let state = lock_state(&state)?;
    let conn = state.connection()?;
    Ok(CharacterService::get_by_id(&conn, id)?)
}

#[tauri::command]
#[specta::specta]
pub fn create_character(name: String, state: State<Mutex<AppState>>) -> CommandResult<CharacterView> {
    let mut state = lock_state(&state)?;
    let conn = state.connection()?;
    let character = CharacterService::build_new(&conn, name)?;
    state.draft_character = Some(character.clone());
    Ok(character)
}

#[tauri::command]
#[specta::specta]
pub fn get_draft_character(state: State<Mutex<AppState>>) -> CommandResult<Option<CharacterView>> {
    let state = lock_state(&state)?;
    Ok(state.draft_character.clone())
}

#[tauri::command]
#[specta::specta]
pub fn save_character(state: State<Mutex<AppState>>) -> CommandResult<CharacterView> {
    let mut state = lock_state(&state)?;
    let draft = state.draft()?;
    let conn = state.connection()?;
    let saved = CharacterService::save(&conn, draft)?;
    // Update draft with saved version (now has an ID if it was new)
    state.draft_character = Some(saved.clone());
    Ok(saved)
}

#[tauri::command]
#[specta::specta]
pub fn discard_draft(state: State<Mutex<AppState>>) -> CommandResult<()> {
    let mut state = lock_state(&state)?;
    state.draft_character = None;
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub fn load_character_into_draft(
    id: i64,
    state: State<Mutex<AppState>>,
) -> CommandResult<CharacterView> {
    let mut state = lock_state(&state)?;
    let conn = state.connection()?;
    let character = CharacterService::get_by_id(&conn, id)?
        .ok_or_else(|| CommandError::NotFound("Character not found".to_string()))?;
    state.draft_character = Some(character.clone());
    Ok(character)
}

#[tauri::command]
#[specta::specta]
pub fn update_draft_basic_info(
    name: String,
    is_wild_card: bool,
    background: Option<String>,
    description: Option<String>,
    state: State<Mutex<AppState>>,
) -> CommandResult<CharacterView> {
    let mut state = lock_state(&state)?;
    let draft = state.draft_mut()?;

    draft.name = name;
    draft.is_wild_card = is_wild_card;
    draft.background = background;
    draft.description = description;

    Ok(draft.clone())
}

#[tauri::command]
#[specta::specta]
pub fn update_character_status(
    id: i64,
    wounds: i64,
    fatigue: i64,
    power_points_used: i64,
    state: State<Mutex<AppState>>,
) -> CommandResult<CharacterView> {
    let state = lock_state(&state)?;
    let conn = state.connection()?;

    // Get the character from the database
    let mut character = CharacterRepository::get_by_id(&conn, id)?
        .ok_or_else(|| CommandError::NotFound("Character not found".to_string()))?;

    // Update the status fields
    character.wounds = wounds;
    character.fatigue = fatigue;
    character.power_points_used = power_points_used;

    // Save back to database
    CharacterRepository::update(&conn, &character)?;

    // Return the full character view
    CharacterService::get_by_id(&conn, id)?
        .ok_or_else(|| CommandError::NotFound("Character not found after update".to_string()))
}

/// Update a character's portrait.
/// Accepts base64-encoded image data and mime type (e.g., "image/png" or "image/jpeg").
#[tauri::command]
#[specta::specta]
pub fn update_character_portrait(
    id: i64,
    image_base64: String,
    mime_type: String,
    state: State<Mutex<AppState>>,
) -> CommandResult<CharacterView> {
    let state = lock_state(&state)?;
    let conn = state.connection()?;

    // Validate mime type
    if mime_type != "image/png" && mime_type != "image/jpeg" {
        return Err(CommandError::Validation(
            "Portrait must be PNG or JPEG format".to_string(),
        ));
    }

    // Decode base64 to bytes
    let portrait_bytes = BASE64
        .decode(&image_base64)
        .map_err(|e| CommandError::Validation(format!("Invalid base64 image data: {}", e)))?;

    // Validate size (max ~2MB for a 1024x1024 image is reasonable)
    if portrait_bytes.len() > 2 * 1024 * 1024 {
        return Err(CommandError::Validation(
            "Portrait image is too large (max 2MB)".to_string(),
        ));
    }

    // Update the portrait
    CharacterRepository::update_portrait(&conn, id, Some(&portrait_bytes), Some(&mime_type))?;

    // Return the updated character
    CharacterService::get_by_id(&conn, id)?
        .ok_or_else(|| CommandError::NotFound("Character not found".to_string()))
}

/// Clear a character's portrait.
#[tauri::command]
#[specta::specta]
pub fn clear_character_portrait(
    id: i64,
    state: State<Mutex<AppState>>,
) -> CommandResult<CharacterView> {
    let state = lock_state(&state)?;
    let conn = state.connection()?;

    // Clear the portrait
    CharacterRepository::update_portrait(&conn, id, None, None)?;

    // Return the updated character
    CharacterService::get_by_id(&conn, id)?
        .ok_or_else(|| CommandError::NotFound("Character not found".to_string()))
}

/// Delete a character permanently.
#[tauri::command]
#[specta::specta]
pub fn delete_character(
    id: i64,
    state: State<Mutex<AppState>>,
) -> CommandResult<()> {
    let mut state = lock_state(&state)?;
    let conn = state.connection()?;

    // Check if character exists
    CharacterRepository::get_by_id(&conn, id)?
        .ok_or_else(|| CommandError::NotFound("Character not found".to_string()))?;

    // Clear draft if it's this character
    if let Some(ref draft) = state.draft_character {
        if draft.id == id {
            state.draft_character = None;
        }
    }

    // Delete the character (cascade deletes related records)
    CharacterRepository::delete(&conn, id)?;

    Ok(())
}
