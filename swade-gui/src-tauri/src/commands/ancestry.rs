//! Ancestry selection and choice commands.

use std::sync::Mutex;
use swade_core::constants::SOURCE_ANCESTRY;
use swade_core::services::{AncestryService, EdgeService, HindranceService};
use swade_core::views::{
    AncestryOptionType, AncestryView, CharacterAncestryChoiceValue, CharacterEdgeValue,
    CharacterHindranceValue, CharacterView,
};
use tauri::State;

use crate::error::{CommandError, CommandResult};
use crate::state::{clear_arcane_backgrounds, lock_state, AppState};

#[tauri::command]
#[specta::specta]
pub fn get_ancestries(state: State<Mutex<AppState>>) -> CommandResult<Vec<AncestryView>> {
    let state = lock_state(&state)?;
    let conn = state.connection()?;
    Ok(AncestryService::get_all(&conn)?)
}

#[tauri::command]
#[specta::specta]
pub fn update_draft_ancestry(
    ancestry_id: Option<i64>,
    state: State<Mutex<AppState>>,
) -> CommandResult<CharacterView> {
    let mut state = lock_state(&state)?;
    let conn = state.connection()?;

    let draft = state.draft_mut()?;

    // Load the ancestry if an ID was provided
    let ancestry = match ancestry_id {
        Some(id) => AncestryService::get_by_id(&conn, id)?,
        None => None,
    };

    // Check if any ancestry edge being removed is Arcane Background
    let had_arcane_background = draft
        .edges
        .iter()
        .any(|e| e.source == SOURCE_ANCESTRY && e.edge.name == "Arcane Background");

    // Calculate points from ancestry hindrances being removed
    let ancestry_hindrance_points: i64 = draft
        .hindrances
        .iter()
        .filter(|h| h.source == SOURCE_ANCESTRY)
        .map(|h| h.hindrance.point_value)
        .sum();

    // Remove any edges and hindrances from the previous ancestry
    draft.edges.retain(|e| e.source != SOURCE_ANCESTRY);
    draft.hindrances.retain(|h| h.source != SOURCE_ANCESTRY);

    // Update hindrance points earned
    draft.hindrance_points_earned -= ancestry_hindrance_points;

    // If an Arcane Background edge was removed, clear arcane backgrounds and powers
    if had_arcane_background {
        clear_arcane_backgrounds(draft);
    }

    // Initialize ancestry choices from the new ancestry
    draft.ancestry_choices = match &ancestry {
        Some(a) => a
            .choices
            .iter()
            .map(|choice| CharacterAncestryChoiceValue::new(choice.clone(), None))
            .collect(),
        None => vec![],
    };
    draft.ancestry = ancestry;

    // Recompute effective values (ancestry can have die modifiers)
    draft.compute_effective_values();

    Ok(draft.clone())
}

#[tauri::command]
#[specta::specta]
pub fn update_draft_ancestry_choice(
    choice_id: i64,
    selected_option_id: Option<i64>,
    state: State<Mutex<AppState>>,
) -> CommandResult<CharacterView> {
    let mut state = lock_state(&state)?;
    let conn = state.connection()?;

    let draft = state.draft_mut()?;

    // Find the choice and get the previous selected option (if any)
    let choice_value = draft
        .ancestry_choices
        .iter_mut()
        .find(|c| c.choice.id == choice_id)
        .ok_or_else(|| CommandError::NotFound("Choice not found".to_string()))?;

    let previous_option = choice_value.selected_option.clone();

    // Find the new selected option in the choice's options
    let new_option = match selected_option_id {
        Some(opt_id) => {
            let option = choice_value
                .choice
                .options
                .iter()
                .find(|o| o.id == opt_id)
                .ok_or_else(|| CommandError::NotFound("Option not found in choice".to_string()))?;
            Some(option.clone())
        }
        None => None,
    };

    // Update the selected option
    choice_value.selected_option = new_option.clone();

    // Handle edge/hindrance removal if previous option was an edge or hindrance
    if let Some(prev_opt) = &previous_option {
        match prev_opt.option_type {
            AncestryOptionType::Edge => {
                if let Some(edge_id) = prev_opt.option_id {
                    // Check if this is an Arcane Background edge before removing
                    let is_arcane_background = draft.edges.iter().any(|e| {
                        e.edge.id == edge_id
                            && e.source == SOURCE_ANCESTRY
                            && e.edge.name == "Arcane Background"
                    });

                    // Remove the edge from draft.edges (only ancestry-sourced edges)
                    draft
                        .edges
                        .retain(|e| !(e.edge.id == edge_id && e.source == SOURCE_ANCESTRY));

                    // If the removed edge was Arcane Background, clear arcane backgrounds and powers
                    if is_arcane_background {
                        clear_arcane_backgrounds(draft);
                    }
                }
            }
            AncestryOptionType::Hindrance => {
                if let Some(hindrance_id) = prev_opt.option_id {
                    // Find and remove the hindrance, updating points
                    if let Some(pos) = draft.hindrances.iter().position(
                        |h| h.hindrance.id == hindrance_id && h.source == SOURCE_ANCESTRY
                    ) {
                        let removed = draft.hindrances.remove(pos);
                        draft.hindrance_points_earned -= removed.hindrance.point_value;
                    }
                }
            }
            AncestryOptionType::Ancestry => {}
        }
    }

    // Handle edge/hindrance addition if new option is an edge or hindrance
    if let Some(new_opt) = &new_option {
        match new_opt.option_type {
            AncestryOptionType::Edge => {
                if let Some(edge_id) = new_opt.option_id {
                    // Load the edge from the database
                    let edge = EdgeService::get_by_id(&conn, edge_id)?
                        .ok_or_else(|| CommandError::NotFound(format!("Edge with id {} not found", edge_id)))?;

                    // Add the edge with source "ancestry"
                    draft.edges.push(CharacterEdgeValue::new(
                        edge,
                        0,    // advance_taken
                        None, // notes
                        SOURCE_ANCESTRY.to_string(),
                    ));
                }
            }
            AncestryOptionType::Hindrance => {
                if let Some(hindrance_id) = new_opt.option_id {
                    // Load the hindrance from the database
                    let hindrance = HindranceService::get_by_id(&conn, hindrance_id)?
                        .ok_or_else(|| CommandError::NotFound(format!("Hindrance with id {} not found", hindrance_id)))?;

                    // Update hindrance points earned
                    draft.hindrance_points_earned += hindrance.point_value;

                    // Add the hindrance with source "ancestry"
                    draft.hindrances.push(CharacterHindranceValue::new(
                        hindrance,
                        SOURCE_ANCESTRY.to_string(),
                    ));
                }
            }
            AncestryOptionType::Ancestry => {}
        }
    }

    // Recompute effective values (choice options can have die modifiers, edges can too)
    draft.compute_effective_values();

    Ok(draft.clone())
}
