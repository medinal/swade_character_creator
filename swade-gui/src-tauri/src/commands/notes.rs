//! Character notes commands.
//!
//! Notes operate directly on saved characters, not drafts.

use std::sync::Mutex;
use swade_core::models::CharacterNote;
use swade_core::repositories::CharacterNoteRepository;
use swade_core::views::CharacterNoteValue;
use tauri::State;

use crate::error::{CommandError, CommandResult};
use crate::state::{lock_state, AppState};

#[tauri::command]
#[specta::specta]
pub fn get_character_notes(
    character_id: i64,
    state: State<Mutex<AppState>>,
) -> CommandResult<Vec<CharacterNoteValue>> {
    let state = lock_state(&state)?;
    let conn = state.connection()?;

    let notes = CharacterNoteRepository::get_by_character_id(&conn, character_id)?;

    Ok(notes
        .into_iter()
        .map(|n| CharacterNoteValue::new(n.id, n.title, n.body, n.created_at, n.updated_at))
        .collect())
}

#[tauri::command]
#[specta::specta]
pub fn create_character_note(
    character_id: i64,
    title: String,
    body: String,
    state: State<Mutex<AppState>>,
) -> CommandResult<CharacterNoteValue> {
    let state = lock_state(&state)?;
    let conn = state.connection()?;

    let note = CharacterNote {
        id: 0,
        character_id,
        title,
        body,
        created_at: String::new(),
        updated_at: String::new(),
    };

    let id = CharacterNoteRepository::insert(&conn, &note)?;

    // Return the newly created note
    let created = CharacterNoteRepository::get_by_id(&conn, id)?
        .ok_or_else(|| CommandError::NotFound("Note not found after creation".to_string()))?;

    Ok(CharacterNoteValue::new(
        created.id,
        created.title,
        created.body,
        created.created_at,
        created.updated_at,
    ))
}

#[tauri::command]
#[specta::specta]
pub fn update_character_note(
    note_id: i64,
    title: String,
    body: String,
    state: State<Mutex<AppState>>,
) -> CommandResult<CharacterNoteValue> {
    let state = lock_state(&state)?;
    let conn = state.connection()?;

    // Get existing note to update
    let mut note = CharacterNoteRepository::get_by_id(&conn, note_id)?
        .ok_or_else(|| CommandError::NotFound("Note not found".to_string()))?;

    note.title = title;
    note.body = body;

    CharacterNoteRepository::update(&conn, &note)?;

    // Return the updated note
    let updated = CharacterNoteRepository::get_by_id(&conn, note_id)?
        .ok_or_else(|| CommandError::NotFound("Note not found after update".to_string()))?;

    Ok(CharacterNoteValue::new(
        updated.id,
        updated.title,
        updated.body,
        updated.created_at,
        updated.updated_at,
    ))
}

#[tauri::command]
#[specta::specta]
pub fn delete_character_note(
    note_id: i64,
    state: State<Mutex<AppState>>,
) -> CommandResult<()> {
    let state = lock_state(&state)?;
    let conn = state.connection()?;

    CharacterNoteRepository::delete(&conn, note_id)?;

    Ok(())
}
