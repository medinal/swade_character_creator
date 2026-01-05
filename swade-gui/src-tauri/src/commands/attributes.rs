//! Attribute modification and hindrance point allocation commands.

use std::sync::Mutex;
use swade_core::constants::ATTRIBUTE_HINDRANCE_POINT_COST;
use swade_core::views::CharacterView;
use tauri::State;

use crate::error::{CommandError, CommandResult};
use crate::state::{available_hindrance_points, lock_state, remove_invalid_edges, AppState};

#[tauri::command]
#[specta::specta]
pub fn update_draft_attribute(
    attribute_id: i64,
    increment: bool,
    state: State<Mutex<AppState>>,
) -> CommandResult<CharacterView> {
    let mut state = lock_state(&state)?;

    let draft = state.draft_mut()?;

    // Find the attribute
    let attr_value = draft
        .attributes
        .iter_mut()
        .find(|a| a.attribute.id == attribute_id)
        .ok_or_else(|| CommandError::NotFound("Attribute not found".to_string()))?;

    if increment {
        // Check if we have points available
        let total_available =
            draft.attribute_points_earned + draft.hindrance_points_to_attributes;
        if draft.attribute_points_spent >= total_available {
            return Err(CommandError::Validation("No attribute points available".to_string()));
        }

        // Check if already at max (d12 is the purchasable max)
        if attr_value.die.size() == 12 {
            return Err(CommandError::Validation("Attribute already at maximum".to_string()));
        }

        // Increment the die
        attr_value.die = attr_value.die.increment();
        draft.attribute_points_spent += 1;
    } else {
        // Decrement - check if at base
        if attr_value.die == attr_value.attribute.base_die {
            return Err(CommandError::Validation("Attribute already at base value".to_string()));
        }

        // Decrement the die
        attr_value.die = attr_value
            .die
            .decrement()
            .ok_or_else(|| CommandError::Validation("Cannot decrement below d4".to_string()))?;
        draft.attribute_points_spent -= 1;

        // Recompute effective values and remove edges that no longer meet requirements
        draft.compute_effective_values();
        remove_invalid_edges(draft);
    }

    // Recompute effective values after the change
    draft.compute_effective_values();

    Ok(draft.clone())
}

#[tauri::command]
#[specta::specta]
pub fn allocate_hindrance_points_to_attributes(
    points: i64,
    state: State<Mutex<AppState>>,
) -> CommandResult<CharacterView> {
    let mut state = lock_state(&state)?;

    let draft = state.draft_mut()?;
    let available = available_hindrance_points(draft);

    // Validate: need ATTRIBUTE_HINDRANCE_POINT_COST hindrance points per attribute point
    if points < 0 {
        // Deallocating - make sure we're not removing points that are spent
        let new_total = draft.hindrance_points_to_attributes + points;
        if new_total < 0 {
            return Err(CommandError::Validation("Cannot deallocate more points than allocated".to_string()));
        }
        // Check if removing these points would leave us with negative available attribute points
        let new_attribute_total = draft.attribute_points_earned + new_total;
        if draft.attribute_points_spent > new_attribute_total {
            return Err(CommandError::Validation("Cannot deallocate: attribute points already spent".to_string()));
        }
        draft.hindrance_points_to_attributes = new_total;
    } else {
        // Allocating - need ATTRIBUTE_HINDRANCE_POINT_COST hindrance points per attribute point
        let hindrance_cost = points * ATTRIBUTE_HINDRANCE_POINT_COST;
        if hindrance_cost > available {
            return Err(CommandError::Validation(format!(
                "Not enough hindrance points. Need {}, have {}",
                hindrance_cost, available
            )));
        }
        draft.hindrance_points_to_attributes += points;
    }

    // Recompute effective values so can_increment/can_decrement are updated
    draft.compute_effective_values();

    Ok(draft.clone())
}

/// Check which edges would be invalidated if an attribute is decremented.
/// Returns the list of edge names that would lose their requirements.
#[tauri::command]
#[specta::specta]
pub fn check_attribute_decrement_impact(
    attribute_id: i64,
    state: State<Mutex<AppState>>,
) -> CommandResult<Vec<String>> {
    let state = lock_state(&state)?;

    let draft = state.draft()?;

    // Find the attribute and check if it can be decremented
    let attr_value = draft
        .attributes
        .iter()
        .find(|a| a.attribute.id == attribute_id)
        .ok_or_else(|| CommandError::NotFound("Attribute not found".to_string()))?;

    // Can't decrement if at base
    if attr_value.die == attr_value.attribute.base_die {
        return Ok(vec![]);
    }

    // Simulate the decrement
    let new_die_size = attr_value.die.decrement().map(|d| d.size()).unwrap_or(4);

    // Check each edge's requirements against the hypothetical new state
    let mut affected_edges = Vec::new();

    for edge_value in &draft.edges {
        // Build a modified context with the decremented attribute
        let mut ctx = draft.to_requirement_context();

        // Update the attribute die in the context
        ctx.attribute_dies.insert(attribute_id, new_die_size);

        // Check if this edge's requirements would still be met
        let currently_met = edge_value
            .edge
            .requirements
            .evaluate(&draft.to_requirement_context());
        let would_be_met = edge_value.edge.requirements.evaluate(&ctx);

        if currently_met && !would_be_met {
            affected_edges.push(edge_value.edge.name.clone());
        }
    }

    Ok(affected_edges)
}
