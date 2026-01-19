//! Edge selection and hindrance point allocation commands.

use std::sync::Mutex;
use swade_core::constants::{EDGE_HINDRANCE_POINT_COST, SOURCE_HINDRANCE_POINTS};
use swade_core::services::EdgeService;
use swade_core::views::{CharacterEdgeValue, CharacterView, EdgeView, RequirementStatus};
use tauri::State;

use crate::error::{CommandError, CommandResult};
use crate::state::{available_hindrance_points, clear_arcane_backgrounds, lock_state, AppState};

use super::types::{DraftResult, ValidationWarning};

/// Edge with its availability status for the current character.
#[derive(serde::Serialize, specta::Type)]
pub struct EdgeWithAvailability {
    pub edge: EdgeView,
    pub is_available: bool,
    pub requirement_statuses: Vec<RequirementStatus>,
}

#[tauri::command]
#[specta::specta]
pub fn get_edges(state: State<Mutex<AppState>>) -> CommandResult<Vec<EdgeWithAvailability>> {
    let state = lock_state(&state)?;
    let conn = state.connection()?;
    let edges = EdgeService::get_all(&conn)?;

    // If there's a draft character, evaluate requirements against it
    let results = match &state.draft_character {
        Some(draft) => {
            let ctx = draft.to_requirement_context();
            edges
                .into_iter()
                .map(|edge| {
                    let is_available = edge.requirements.evaluate(&ctx);
                    let requirement_statuses = edge.requirements.evaluate_detailed(&ctx);
                    EdgeWithAvailability {
                        edge,
                        is_available,
                        requirement_statuses,
                    }
                })
                .collect()
        }
        None => {
            // No draft character - all edges are "available" (no character to check against)
            edges
                .into_iter()
                .map(|edge| EdgeWithAvailability {
                    is_available: true,
                    requirement_statuses: vec![],
                    edge,
                })
                .collect()
        }
    };

    Ok(results)
}

#[tauri::command]
#[specta::specta]
pub fn add_draft_edge(
    edge_id: i64,
    notes: Option<String>,
    bypass_validation: Option<bool>,
    state: State<Mutex<AppState>>,
) -> CommandResult<DraftResult> {
    let mut state = lock_state(&state)?;
    let conn = state.connection()?;
    let bypass = bypass_validation.unwrap_or(false);
    let mut warnings = Vec::new();

    let draft = state.draft_mut()?;

    // Load the edge
    let edge = EdgeService::get_by_id(&conn, edge_id)?
        .ok_or_else(|| CommandError::NotFound("Edge not found".to_string()))?;

    // Check if edge can be taken multiple times, or if already taken
    if !edge.can_take_multiple_times
        && draft.edges.iter().any(|e| e.edge.id == edge_id) {
            return Err(CommandError::Validation("Edge already taken and cannot be taken multiple times".to_string()));
        }

    // Check requirements
    let ctx = draft.to_requirement_context();
    if !edge.requirements.evaluate(&ctx) {
        if bypass {
            warnings.push(ValidationWarning::requirement_not_met(format!(
                "Edge '{}' requirements not met",
                edge.name
            )));
        } else {
            return Err(CommandError::Validation("Character does not meet edge requirements".to_string()));
        }
    }

    // If edge can be taken multiple times, notes are required
    if edge.can_take_multiple_times && notes.is_none() {
        return Err(CommandError::Validation("This edge requires notes (e.g., specify the skill or weapon)".to_string()));
    }

    // Check if we have enough allocated hindrance points for edges
    // Count current edges from hindrance_points source
    let edges_from_hindrance_points = draft
        .edges
        .iter()
        .filter(|e| e.source == SOURCE_HINDRANCE_POINTS)
        .count() as i64;
    let points_spent_on_edges = edges_from_hindrance_points * EDGE_HINDRANCE_POINT_COST;
    let points_available_for_edges = draft.hindrance_points_to_edges - points_spent_on_edges;

    if points_available_for_edges < 2 {
        if bypass {
            warnings.push(ValidationWarning::point_limit_exceeded(
                "Not enough hindrance points allocated to edges".to_string(),
            ));
        } else {
            return Err(CommandError::Validation(
                "Not enough hindrance points allocated to edges. Allocate more points first."
                    .to_string(),
            ));
        }
    }

    // Apply wealth modifiers from the edge (e.g., Rich, Filthy Rich)
    let wealth_bonus: i64 = edge
        .modifiers
        .iter()
        .filter(|m| m.target_type.as_deref() == Some("wealth"))
        .filter_map(|m| m.value)
        .sum();
    if wealth_bonus > 0 {
        draft.wealth += wealth_bonus;
    }

    // Add the edge
    // Source is "hindrance_points" for character creation
    // advance_taken is 0 for character creation (not an advancement)
    draft.edges.push(CharacterEdgeValue::new(
        edge,
        0, // advance_taken
        notes,
        SOURCE_HINDRANCE_POINTS.to_string(),
    ));

    // Note: hindrance_points_to_edges is not modified here - it tracks allocated points,
    // and edge count * 2 tracks spent points

    // Recompute effective values (edges can have die modifiers)
    draft.compute_effective_values();

    Ok(DraftResult::with_warnings(draft.clone(), warnings))
}

#[tauri::command]
#[specta::specta]
pub fn remove_draft_edge(
    edge_id: i64,
    state: State<Mutex<AppState>>,
) -> CommandResult<CharacterView> {
    let mut state = lock_state(&state)?;

    let draft = state.draft_mut()?;

    // Find and remove the edge (only edges from hindrance_points can be removed during creation)
    let position = draft
        .edges
        .iter()
        .position(|e| e.edge.id == edge_id && e.source == SOURCE_HINDRANCE_POINTS);

    match position {
        Some(idx) => {
            let removed_edge = draft.edges.remove(idx);

            // If the removed edge is "Arcane Background", clear arcane backgrounds and powers
            if removed_edge.edge.name == "Arcane Background" {
                clear_arcane_backgrounds(draft);
            }

            // Remove wealth modifiers from the edge (e.g., Rich, Filthy Rich)
            let wealth_bonus: i64 = removed_edge
                .edge
                .modifiers
                .iter()
                .filter(|m| m.target_type.as_deref() == Some("wealth"))
                .filter_map(|m| m.value)
                .sum();
            if wealth_bonus > 0 {
                draft.wealth -= wealth_bonus;
                if draft.wealth < 0 {
                    draft.wealth = 0;
                }
            }

            // Note: hindrance_points_to_edges is not modified here - it tracks allocated points,
            // removing an edge just frees up those allocated points for another edge

            // Recompute effective values
            draft.compute_effective_values();

            Ok(draft.clone())
        }
        None => Err(CommandError::NotFound("Edge not found or cannot be removed".to_string())),
    }
}

#[tauri::command]
#[specta::specta]
pub fn allocate_hindrance_points_to_edges(
    points: i64,
    state: State<Mutex<AppState>>,
) -> CommandResult<CharacterView> {
    let mut state = lock_state(&state)?;

    let draft = state.draft_mut()?;
    let available = available_hindrance_points(draft);

    // Count edges from hindrance_points to track what's been "spent"
    let edges_from_hindrance_points = draft
        .edges
        .iter()
        .filter(|e| e.source == SOURCE_HINDRANCE_POINTS)
        .count() as i64;
    let points_spent_on_edges = edges_from_hindrance_points * EDGE_HINDRANCE_POINT_COST;

    if points < 0 {
        // Deallocating
        let new_total = draft.hindrance_points_to_edges + points;
        if new_total < 0 {
            return Err(CommandError::Validation("Cannot deallocate more points than allocated".to_string()));
        }
        // Check if removing these points would leave us with not enough for current edges
        if new_total < points_spent_on_edges {
            return Err(CommandError::Validation(
                "Cannot deallocate: points already spent on edges. Remove edges first.".to_string(),
            ));
        }
        draft.hindrance_points_to_edges = new_total;
    } else {
        // Allocating - must allocate in increments of 2 (edge cost)
        if points % EDGE_HINDRANCE_POINT_COST != 0 {
            return Err(CommandError::Validation("Must allocate hindrance points to edges in multiples of 2".to_string()));
        }
        if points > available {
            return Err(CommandError::Validation(format!(
                "Not enough hindrance points. Need {}, have {}",
                points, available
            )));
        }
        draft.hindrance_points_to_edges += points;
    }

    Ok(draft.clone())
}
