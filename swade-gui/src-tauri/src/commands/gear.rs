//! Gear management commands for character inventory.

use std::sync::Mutex;
use swade_core::models::CharacterGear;
use swade_core::repositories::CharacterGearRepository;
use swade_core::services::{CharacterService, GearService};
use swade_core::views::{CharacterGearValue, CharacterView, GearCategoryView, GearView};
use tauri::State;

use crate::error::{CommandError, CommandResult};
use crate::state::{lock_state, AppState};
use rusqlite::Connection;

/// Helper function to add or update gear quantity for a character.
/// If the character already has this gear, increases quantity; otherwise inserts new entry.
fn add_gear_item(
    conn: &Connection,
    character_id: i64,
    gear_id: i64,
    quantity: i64,
) -> Result<(), CommandError> {
    let existing_gear = CharacterGearRepository::get_by_character_id(conn, character_id)?;
    let existing = existing_gear.iter().find(|g| g.gear_id == gear_id);

    let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();

    if let Some(existing_item) = existing {
        // Update quantity
        let updated = CharacterGear {
            id: existing_item.id,
            character_id,
            gear_id,
            quantity: existing_item.quantity + quantity,
            is_equipped: existing_item.is_equipped,
            custom_notes: existing_item.custom_notes.clone(),
            created_at: existing_item.created_at.clone(),
            updated_at: now,
        };
        CharacterGearRepository::update(conn, &updated)?;
    } else {
        // Insert new gear
        let new_gear = CharacterGear {
            id: 0,
            character_id,
            gear_id,
            quantity,
            is_equipped: false,
            custom_notes: None,
            created_at: now.clone(),
            updated_at: now,
        };
        CharacterGearRepository::insert(conn, &new_gear)?;
    }

    Ok(())
}

/// Get all available gear items.
#[tauri::command]
#[specta::specta]
pub fn get_all_gear(state: State<Mutex<AppState>>) -> CommandResult<Vec<GearView>> {
    let state = lock_state(&state)?;
    let conn = state.connection()?;
    let gear = GearService::get_all(&conn)?;
    Ok(gear)
}

/// Get all gear categories.
#[tauri::command]
#[specta::specta]
pub fn get_gear_categories(state: State<Mutex<AppState>>) -> CommandResult<Vec<GearCategoryView>> {
    let state = lock_state(&state)?;
    let conn = state.connection()?;
    let categories = GearService::get_all_categories(&conn)?;
    Ok(categories)
}

/// Get gear items by category.
#[tauri::command]
#[specta::specta]
pub fn get_gear_by_category(
    category_id: i64,
    state: State<Mutex<AppState>>,
) -> CommandResult<Vec<GearView>> {
    let state = lock_state(&state)?;
    let conn = state.connection()?;
    let gear = GearService::get_by_category_id(&conn, category_id)?;
    Ok(gear)
}

/// Get a character's gear inventory.
#[tauri::command]
#[specta::specta]
pub fn get_character_gear(
    character_id: i64,
    state: State<Mutex<AppState>>,
) -> CommandResult<Vec<CharacterGearValue>> {
    let state = lock_state(&state)?;
    let conn = state.connection()?;

    let character_gear = CharacterGearRepository::get_by_character_id(&conn, character_id)?;

    let mut values = Vec::new();
    for cg in character_gear {
        if let Some(gear_view) = GearService::get_by_id(&conn, cg.gear_id)? {
            values.push(CharacterGearValue::new(
                cg.id,
                gear_view,
                cg.quantity,
                cg.is_equipped,
                cg.custom_notes,
            ));
        }
    }

    Ok(values)
}

/// Add gear to a character without paying (GM grants, found items, starting gear).
/// If the gear is a pack, adds individual pack contents instead of the pack itself.
#[tauri::command]
#[specta::specta]
pub fn add_gear(
    character_id: i64,
    gear_id: i64,
    quantity: i64,
    state: State<Mutex<AppState>>,
) -> CommandResult<CharacterView> {
    let state = lock_state(&state)?;
    let conn = state.connection()?;

    // Verify the gear exists
    let gear = GearService::get_by_id(&conn, gear_id)?
        .ok_or_else(|| CommandError::NotFound("Gear not found".to_string()))?;

    // Verify the character exists
    let _character = CharacterService::get_by_id(&conn, character_id)?
        .ok_or_else(|| CommandError::NotFound("Character not found".to_string()))?;

    // If this is a pack, add individual contents instead of the pack itself
    if let Some(ref pack_contents) = gear.pack_contents {
        for pack_item in pack_contents {
            // Multiply pack item quantity by the number of packs being added
            add_gear_item(&conn, character_id, pack_item.item.id, pack_item.quantity * quantity)?;
        }
    } else {
        // Not a pack - add the gear directly
        add_gear_item(&conn, character_id, gear_id, quantity)?;
    }

    // Reload the character with updated gear
    CharacterService::get_by_id(&conn, character_id)?
        .ok_or_else(|| CommandError::NotFound("Character not found".to_string()))
}

/// Purchase gear for a character (deducts from wealth).
/// If the gear is a pack, adds individual pack contents instead of the pack itself.
#[tauri::command]
#[specta::specta]
pub fn purchase_gear(
    character_id: i64,
    gear_id: i64,
    quantity: i64,
    state: State<Mutex<AppState>>,
) -> CommandResult<CharacterView> {
    let state = lock_state(&state)?;
    let conn = state.connection()?;

    // Verify the gear exists and get its cost
    let gear = GearService::get_by_id(&conn, gear_id)?
        .ok_or_else(|| CommandError::NotFound("Gear not found".to_string()))?;

    // Verify the character exists
    let character = CharacterService::get_by_id(&conn, character_id)?
        .ok_or_else(|| CommandError::NotFound("Character not found".to_string()))?;

    // Calculate total cost (pack price, not sum of individual items)
    let total_cost = gear.cost * quantity;

    // Check if character has enough wealth
    if character.wealth < total_cost {
        return Err(CommandError::Validation(format!(
            "Insufficient funds. Need ${}, have ${}",
            total_cost, character.wealth
        )));
    }

    // Deduct wealth
    conn.execute(
        "UPDATE characters SET wealth = wealth - ?, updated_at = ? WHERE id = ?",
        rusqlite::params![total_cost, chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(), character_id],
    ).map_err(|e| CommandError::Database(e.to_string()))?;

    // If this is a pack, add individual contents instead of the pack itself
    if let Some(ref pack_contents) = gear.pack_contents {
        for pack_item in pack_contents {
            // Multiply pack item quantity by the number of packs being purchased
            add_gear_item(&conn, character_id, pack_item.item.id, pack_item.quantity * quantity)?;
        }
    } else {
        // Not a pack - add the gear directly
        add_gear_item(&conn, character_id, gear_id, quantity)?;
    }

    // Reload the character with updated gear and wealth
    CharacterService::get_by_id(&conn, character_id)?
        .ok_or_else(|| CommandError::NotFound("Character not found".to_string()))
}

/// Sell gear from a character (at 50% value).
#[tauri::command]
#[specta::specta]
pub fn sell_gear(
    character_gear_id: i64,
    quantity: i64,
    state: State<Mutex<AppState>>,
) -> CommandResult<CharacterView> {
    let state = lock_state(&state)?;
    let conn = state.connection()?;

    // Get the character gear record
    let character_gear = CharacterGearRepository::get_by_id(&conn, character_gear_id)?
        .ok_or_else(|| CommandError::NotFound("Character gear not found".to_string()))?;

    // Validate quantity
    if quantity > character_gear.quantity {
        return Err(CommandError::Validation(format!(
            "Cannot sell {} items, only have {}",
            quantity, character_gear.quantity
        )));
    }

    // Get the gear to calculate sale price
    let gear = GearService::get_by_id(&conn, character_gear.gear_id)?
        .ok_or_else(|| CommandError::NotFound("Gear not found".to_string()))?;

    // Calculate sale value (50% of cost)
    let sale_value = (gear.cost * quantity) / 2;

    let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();

    // Add wealth
    conn.execute(
        "UPDATE characters SET wealth = wealth + ?, updated_at = ? WHERE id = ?",
        rusqlite::params![sale_value, &now, character_gear.character_id],
    ).map_err(|e| CommandError::Database(e.to_string()))?;

    // Update or delete gear
    if quantity >= character_gear.quantity {
        // Remove all
        CharacterGearRepository::delete(&conn, character_gear_id)?;
    } else {
        // Reduce quantity
        let updated = CharacterGear {
            id: character_gear.id,
            character_id: character_gear.character_id,
            gear_id: character_gear.gear_id,
            quantity: character_gear.quantity - quantity,
            is_equipped: character_gear.is_equipped,
            custom_notes: character_gear.custom_notes,
            created_at: character_gear.created_at,
            updated_at: now,
        };
        CharacterGearRepository::update(&conn, &updated)?;
    }

    // Reload the character
    CharacterService::get_by_id(&conn, character_gear.character_id)?
        .ok_or_else(|| CommandError::NotFound("Character not found".to_string()))
}

/// Remove gear from a character without selling (lost, destroyed, given away).
#[tauri::command]
#[specta::specta]
pub fn remove_gear(
    character_gear_id: i64,
    quantity: i64,
    state: State<Mutex<AppState>>,
) -> CommandResult<CharacterView> {
    let state = lock_state(&state)?;
    let conn = state.connection()?;

    // Get the character gear record
    let character_gear = CharacterGearRepository::get_by_id(&conn, character_gear_id)?
        .ok_or_else(|| CommandError::NotFound("Character gear not found".to_string()))?;

    // Validate quantity
    if quantity > character_gear.quantity {
        return Err(CommandError::Validation(format!(
            "Cannot remove {} items, only have {}",
            quantity, character_gear.quantity
        )));
    }

    let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();

    // Update or delete gear
    if quantity >= character_gear.quantity {
        // Remove all
        CharacterGearRepository::delete(&conn, character_gear_id)?;
    } else {
        // Reduce quantity
        let updated = CharacterGear {
            id: character_gear.id,
            character_id: character_gear.character_id,
            gear_id: character_gear.gear_id,
            quantity: character_gear.quantity - quantity,
            is_equipped: character_gear.is_equipped,
            custom_notes: character_gear.custom_notes,
            created_at: character_gear.created_at,
            updated_at: now,
        };
        CharacterGearRepository::update(&conn, &updated)?;
    }

    // Reload the character
    CharacterService::get_by_id(&conn, character_gear.character_id)?
        .ok_or_else(|| CommandError::NotFound("Character not found".to_string()))
}

/// Toggle whether gear is equipped.
#[tauri::command]
#[specta::specta]
pub fn toggle_gear_equipped(
    character_gear_id: i64,
    state: State<Mutex<AppState>>,
) -> CommandResult<CharacterView> {
    let state = lock_state(&state)?;
    let conn = state.connection()?;

    // Get the character gear record
    let character_gear = CharacterGearRepository::get_by_id(&conn, character_gear_id)?
        .ok_or_else(|| CommandError::NotFound("Character gear not found".to_string()))?;

    let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();

    // Toggle equipped status
    let updated = CharacterGear {
        id: character_gear.id,
        character_id: character_gear.character_id,
        gear_id: character_gear.gear_id,
        quantity: character_gear.quantity,
        is_equipped: !character_gear.is_equipped,
        custom_notes: character_gear.custom_notes,
        created_at: character_gear.created_at,
        updated_at: now,
    };
    CharacterGearRepository::update(&conn, &updated)?;

    // Reload the character (equipped gear affects modifiers)
    CharacterService::get_by_id(&conn, character_gear.character_id)?
        .ok_or_else(|| CommandError::NotFound("Character not found".to_string()))
}

/// Update custom notes on a gear item.
#[tauri::command]
#[specta::specta]
pub fn update_gear_notes(
    character_gear_id: i64,
    notes: Option<String>,
    state: State<Mutex<AppState>>,
) -> CommandResult<CharacterView> {
    let state = lock_state(&state)?;
    let conn = state.connection()?;

    // Get the character gear record
    let character_gear = CharacterGearRepository::get_by_id(&conn, character_gear_id)?
        .ok_or_else(|| CommandError::NotFound("Character gear not found".to_string()))?;

    let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();

    // Update notes
    let updated = CharacterGear {
        id: character_gear.id,
        character_id: character_gear.character_id,
        gear_id: character_gear.gear_id,
        quantity: character_gear.quantity,
        is_equipped: character_gear.is_equipped,
        custom_notes: notes,
        created_at: character_gear.created_at,
        updated_at: now,
    };
    CharacterGearRepository::update(&conn, &updated)?;

    // Reload the character
    CharacterService::get_by_id(&conn, character_gear.character_id)?
        .ok_or_else(|| CommandError::NotFound("Character not found".to_string()))
}

/// Update a character's wealth directly.
#[tauri::command]
#[specta::specta]
pub fn update_character_wealth(
    character_id: i64,
    wealth: i64,
    state: State<Mutex<AppState>>,
) -> CommandResult<CharacterView> {
    let state = lock_state(&state)?;
    let conn = state.connection()?;

    // Validate wealth is non-negative
    if wealth < 0 {
        return Err(CommandError::Validation("Wealth cannot be negative".to_string()));
    }

    let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();

    conn.execute(
        "UPDATE characters SET wealth = ?, updated_at = ? WHERE id = ?",
        rusqlite::params![wealth, now, character_id],
    ).map_err(|e| CommandError::Database(e.to_string()))?;

    CharacterService::get_by_id(&conn, character_id)?
        .ok_or_else(|| CommandError::NotFound("Character not found".to_string()))
}
