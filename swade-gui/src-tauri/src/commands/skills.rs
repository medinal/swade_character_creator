//! Skill selection and hindrance point allocation commands.

use std::sync::Mutex;
use swade_core::services::SkillService;
use swade_core::views::{CharacterView, GameConfig, SkillView};
use tauri::State;

use crate::error::{CommandError, CommandResult};
use crate::state::{available_hindrance_points, lock_state, remove_invalid_edges, AppState};

use super::types::{DraftResult, ValidationWarning};

#[tauri::command]
#[specta::specta]
pub fn get_skills(state: State<Mutex<AppState>>) -> CommandResult<Vec<SkillView>> {
    let state = lock_state(&state)?;
    let conn = state.connection()?;
    Ok(SkillService::get_all(&conn)?)
}

#[tauri::command]
#[specta::specta]
pub fn get_game_config() -> GameConfig {
    GameConfig::default()
}

#[tauri::command]
#[specta::specta]
pub fn update_draft_skill(
    skill_id: i64,
    increment: bool,
    bypass_validation: Option<bool>,
    state: State<Mutex<AppState>>,
) -> CommandResult<DraftResult> {
    let mut state = lock_state(&state)?;
    let bypass = bypass_validation.unwrap_or(false);
    let mut warnings = Vec::new();

    let draft = state.draft_mut()?;

    // Find the skill
    let skill_value = draft
        .skills
        .iter_mut()
        .find(|s| s.skill.id == skill_id)
        .ok_or_else(|| CommandError::NotFound("Skill not found".to_string()))?;

    // Get the linked attribute's effective die for cost calculation
    let linked_attr_die = draft
        .attributes
        .iter()
        .find(|a| a.attribute.id == skill_value.skill.linked_attribute_id)
        .map(|a| a.effective_die)
        .ok_or_else(|| CommandError::NotFound("Linked attribute not found".to_string()))?;

    if increment {
        // Calculate cost based on SWADE rules:
        // - Training an untrained skill to d4: 1 point
        // - Raising skill at or below linked attribute: 1 point per step
        // - Raising skill above linked attribute: 2 points per step
        let (cost, next_die) = match skill_value.die {
            None => {
                // Training an untrained skill to d4 costs 1 point
                (1, swade_core::views::Die::d4())
            }
            Some(current_die) => {
                let next = current_die.increment();
                // Cost is 2 if next die exceeds linked attribute, else 1
                let cost = if next > linked_attr_die { 2 } else { 1 };
                (cost, next)
            }
        };

        // Check if we have points available
        let total_available = draft.skill_points_earned + draft.hindrance_points_to_skills;
        if draft.skill_points_spent + cost > total_available {
            if bypass {
                warnings.push(ValidationWarning::point_limit_exceeded(
                    "Not enough skill points".to_string(),
                ));
            } else {
                return Err(CommandError::Validation("Not enough skill points".to_string()));
            }
        }

        // Check max die
        let at_max = skill_value.die.map_or(false, |d| d >= skill_value.skill.max_die);
        if at_max {
            if bypass {
                warnings.push(ValidationWarning::point_limit_exceeded(
                    "Skill already at maximum".to_string(),
                ));
            } else {
                return Err(CommandError::Validation("Skill already at maximum".to_string()));
            }
        }

        if !at_max {
            // Increment the die
            skill_value.die = Some(next_die);
            draft.skill_points_spent += cost;
        }
    } else {
        // Decrement
        let current_die = skill_value
            .die
            .ok_or_else(|| CommandError::Validation("Skill is untrained".to_string()))?;

        // Core skills cannot go below d4
        if skill_value.skill.is_core_skill && current_die == swade_core::views::Die::d4() {
            if bypass {
                warnings.push(ValidationWarning::point_limit_exceeded(
                    "Core skills cannot go below d4".to_string(),
                ));
                // Don't decrement, just return with warning
                draft.compute_effective_values();
                return Ok(DraftResult::with_warnings(draft.clone(), warnings));
            } else {
                return Err(CommandError::Validation("Core skills cannot go below d4".to_string()));
            }
        }

        // Calculate refund based on what the cost was to reach current die
        let refund = if current_die > linked_attr_die { 2 } else { 1 };

        // Decrement the die (or set to None for non-core skills going from d4)
        let prev_die = current_die.decrement();
        if prev_die.is_none() && !skill_value.skill.is_core_skill {
            // Non-core skill going from d4 to untrained - refund is 1 (cost to train)
            skill_value.die = None;
            draft.skill_points_spent -= 1;
        } else {
            skill_value.die = prev_die;
            draft.skill_points_spent -= refund;
        }

        // Recompute effective values and remove edges that no longer meet requirements
        // (only if not bypassing validation)
        if !bypass {
            draft.compute_effective_values();
            remove_invalid_edges(draft);
        }
    }

    // Recompute effective values after the change
    draft.compute_effective_values();

    Ok(DraftResult::with_warnings(draft.clone(), warnings))
}

#[tauri::command]
#[specta::specta]
pub fn allocate_hindrance_points_to_skills(
    points: i64,
    state: State<Mutex<AppState>>,
) -> CommandResult<CharacterView> {
    let mut state = lock_state(&state)?;

    let draft = state.draft_mut()?;
    let available = available_hindrance_points(draft);

    if points < 0 {
        // Deallocating
        let new_total = draft.hindrance_points_to_skills + points;
        if new_total < 0 {
            return Err(CommandError::Validation("Cannot deallocate more points than allocated".to_string()));
        }
        // Check if removing these points would leave us with negative available skill points
        let new_skill_total = draft.skill_points_earned + new_total;
        if draft.skill_points_spent > new_skill_total {
            return Err(CommandError::Validation("Cannot deallocate: skill points already spent".to_string()));
        }
        draft.hindrance_points_to_skills = new_total;
    } else {
        // Allocating - 1:1 conversion for skills (unlike attributes which is 2:1)
        if points > available {
            return Err(CommandError::Validation(format!(
                "Not enough hindrance points. Need {}, have {}",
                points, available
            )));
        }
        draft.hindrance_points_to_skills += points;
    }

    // Recompute effective values so can_increment/can_decrement are updated
    draft.compute_effective_values();

    Ok(draft.clone())
}

/// Check which edges would be invalidated if a skill is decremented.
/// Returns the list of edge names that would lose their requirements.
#[tauri::command]
#[specta::specta]
pub fn check_skill_decrement_impact(
    skill_id: i64,
    state: State<Mutex<AppState>>,
) -> CommandResult<Vec<String>> {
    let state = lock_state(&state)?;

    let draft = state.draft()?;

    // Find the skill and check if it can be decremented
    let skill_value = draft
        .skills
        .iter()
        .find(|s| s.skill.id == skill_id)
        .ok_or_else(|| CommandError::NotFound("Skill not found".to_string()))?;

    let current_die = match skill_value.die {
        Some(d) => d,
        None => return Ok(vec![]), // Can't decrement untrained skill
    };

    // Simulate the decrement
    let new_die_size = current_die.decrement().map(|d| d.size());

    // Check each edge's requirements against the hypothetical new state
    let mut affected_edges = Vec::new();

    for edge_value in &draft.edges {
        // Build a modified context with the decremented skill
        let mut ctx = draft.to_requirement_context();

        // Update the skill die in the context
        if let Some(new_size) = new_die_size {
            ctx.skill_dies.insert(skill_id, Some(new_size));
        } else {
            ctx.skill_dies.insert(skill_id, None);
        }

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
