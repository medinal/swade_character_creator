//! Power and arcane background commands.

use std::sync::Mutex;
use swade_core::constants::SOURCE_ARCANE_BACKGROUND;
use swade_core::services::{ArcaneBackgroundService, HindranceService, PowerService};
use swade_core::views::{
    ArcaneBackgroundChoiceType, ArcaneBackgroundOptionType, ArcaneBackgroundView,
    CharacterArcaneBackgroundChoiceValue, CharacterArcaneBackgroundValue, CharacterHindranceValue,
    CharacterPowerValue, CharacterView, PowerView, RequirementStatus,
};
use tauri::State;

use crate::error::{CommandError, CommandResult};
use crate::state::{lock_state, AppState};

/// Arcane background with its availability status for the current character.
#[derive(serde::Serialize, specta::Type)]
pub struct ArcaneBackgroundWithAvailability {
    pub arcane_background: ArcaneBackgroundView,
    pub is_available: bool,
    pub requirement_statuses: Vec<RequirementStatus>,
}

#[tauri::command]
#[specta::specta]
pub fn get_arcane_backgrounds(
    state: State<Mutex<AppState>>,
) -> CommandResult<Vec<ArcaneBackgroundWithAvailability>> {
    let state = lock_state(&state)?;
    let conn = state.connection()?;
    let arcane_backgrounds = ArcaneBackgroundService::get_all(&conn)?;

    // If there's a draft character, evaluate requirements against it
    let results = match &state.draft_character {
        Some(draft) => {
            let ctx = draft.to_requirement_context();
            arcane_backgrounds
                .into_iter()
                .map(|ab| {
                    let is_available = ab.requirements.evaluate(&ctx);
                    let requirement_statuses = ab.requirements.evaluate_detailed(&ctx);
                    ArcaneBackgroundWithAvailability {
                        arcane_background: ab,
                        is_available,
                        requirement_statuses,
                    }
                })
                .collect()
        }
        None => arcane_backgrounds
            .into_iter()
            .map(|ab| ArcaneBackgroundWithAvailability {
                is_available: true,
                requirement_statuses: vec![],
                arcane_background: ab,
            })
            .collect(),
    };

    Ok(results)
}

#[tauri::command]
#[specta::specta]
pub fn add_draft_arcane_background(
    arcane_background_id: i64,
    state: State<Mutex<AppState>>,
) -> CommandResult<CharacterView> {
    let mut state = lock_state(&state)?;
    let conn = state.connection()?;

    let draft = state.draft_mut()?;

    // Check if arcane background is already added
    if draft
        .arcane_backgrounds
        .iter()
        .any(|ab| ab.arcane_background.id == arcane_background_id)
    {
        return Err(CommandError::Validation("Arcane background already added".to_string()));
    }

    // Load the arcane background
    let arcane_background = ArcaneBackgroundService::get_by_id(&conn, arcane_background_id)?
        .ok_or_else(|| CommandError::NotFound("Arcane background not found".to_string()))?;

    // Check requirements
    let ctx = draft.to_requirement_context();
    if !arcane_background.requirements.evaluate(&ctx) {
        return Err(CommandError::Validation("Character does not meet arcane background requirements".to_string()));
    }

    // Initialize arcane_background_choices from the AB's choices
    // Auto-select required options (built_in_hindrance, required_starting_power)
    for choice in &arcane_background.choices {
        let mut selected_options = Vec::new();

        match choice.choice_type {
            // Built-in hindrances are auto-selected and added to draft.hindrances
            ArcaneBackgroundChoiceType::BuiltInHindrance => {
                for option in &choice.options {
                    if option.option_type == ArcaneBackgroundOptionType::Hindrance {
                        if let Some(hindrance_id) = option.option_id {
                            // Load and add the hindrance
                            if let Ok(Some(hindrance)) = HindranceService::get_by_id(&conn, hindrance_id) {
                                // Don't add hindrance points for built-in hindrances
                                // (they're part of the AB, not chosen by player)
                                draft.hindrances.push(CharacterHindranceValue::new(
                                    hindrance,
                                    SOURCE_ARCANE_BACKGROUND.to_string(),
                                ));
                            }
                        }
                        selected_options.push(option.clone());
                    }
                }
            }
            // Required starting powers are auto-selected and added to draft.powers
            ArcaneBackgroundChoiceType::RequiredStartingPower => {
                for option in &choice.options {
                    if option.option_type == ArcaneBackgroundOptionType::Power {
                        if let Some(power_id) = option.option_id {
                            // Load and add the power
                            if let Ok(Some(power)) = PowerService::get_by_id(&conn, power_id) {
                                // Mark as locked (required power can't be removed)
                                let mut power_value = CharacterPowerValue::new(power, None);
                                power_value.is_locked = true;
                                draft.powers.push(power_value);
                            }
                        }
                        selected_options.push(option.clone());
                    }
                }
            }
            // Other choice types start with no selection
            _ => {}
        }

        draft.arcane_background_choices.push(
            CharacterArcaneBackgroundChoiceValue::new(choice.clone(), selected_options),
        );
    }

    // Add the arcane background (advance_taken is None for character creation)
    draft
        .arcane_backgrounds
        .push(CharacterArcaneBackgroundValue::new(
            arcane_background.clone(),
            None,
        ));

    // Add the starting power points
    draft.power_points += arcane_background.starting_power_points;

    Ok(draft.clone())
}

#[tauri::command]
#[specta::specta]
pub fn remove_draft_arcane_background(
    arcane_background_id: i64,
    state: State<Mutex<AppState>>,
) -> CommandResult<CharacterView> {
    let mut state = lock_state(&state)?;

    let draft = state.draft_mut()?;

    // Find the arcane background
    let position = draft
        .arcane_backgrounds
        .iter()
        .position(|ab| ab.arcane_background.id == arcane_background_id);

    match position {
        Some(idx) => {
            let removed = draft.arcane_backgrounds.remove(idx);

            // Remove power points from this arcane background
            draft.power_points -= removed.arcane_background.starting_power_points;
            if draft.power_points < 0 {
                draft.power_points = 0;
            }

            // Remove all powers (since we no longer have the arcane background)
            draft.powers.clear();

            // Remove hindrances that came from this arcane background
            draft
                .hindrances
                .retain(|h| h.source != SOURCE_ARCANE_BACKGROUND);

            // Remove arcane background choices for this AB
            let choice_ids: Vec<i64> = removed
                .arcane_background
                .choices
                .iter()
                .map(|c| c.id)
                .collect();
            draft
                .arcane_background_choices
                .retain(|c| !choice_ids.contains(&c.choice.id));

            Ok(draft.clone())
        }
        None => Err(CommandError::NotFound("Arcane background not found".to_string())),
    }
}

/// Update a character's arcane background choice selection.
///
/// For choosable_starting_power choices, this toggles the power selection.
/// Built-in hindrances and required starting powers cannot be changed.
#[tauri::command]
#[specta::specta]
pub fn update_draft_arcane_background_choice(
    choice_id: i64,
    selected_option_id: i64,
    is_selecting: bool,
    state: State<Mutex<AppState>>,
) -> CommandResult<CharacterView> {
    let mut state = lock_state(&state)?;
    let conn = state.connection()?;

    let draft = state.draft_mut()?;

    // Find the choice
    let choice_value = draft
        .arcane_background_choices
        .iter_mut()
        .find(|c| c.choice.id == choice_id)
        .ok_or_else(|| CommandError::NotFound("Choice not found".to_string()))?;

    // Prevent modifying built-in hindrances and required starting powers
    match choice_value.choice.choice_type {
        ArcaneBackgroundChoiceType::BuiltInHindrance
        | ArcaneBackgroundChoiceType::RequiredStartingPower => {
            return Err(CommandError::Validation(
                "Cannot modify built-in or required choices".to_string(),
            ));
        }
        _ => {}
    }

    // Find the option in the choice
    let option = choice_value
        .choice
        .options
        .iter()
        .find(|o| o.id == selected_option_id)
        .ok_or_else(|| CommandError::NotFound("Option not found in choice".to_string()))?
        .clone();

    if is_selecting {
        // Check if already selected
        if choice_value
            .selected_options
            .iter()
            .any(|o| o.id == selected_option_id)
        {
            return Err(CommandError::Validation("Option already selected".to_string()));
        }

        // Check max_selections limit
        if choice_value.selected_options.len() as i64 >= choice_value.choice.max_selections {
            return Err(CommandError::Validation(format!(
                "Maximum {} selections allowed for this choice",
                choice_value.choice.max_selections
            )));
        }

        // Handle the option based on type
        match option.option_type {
            ArcaneBackgroundOptionType::Power => {
                if let Some(power_id) = option.option_id {
                    // Load and add the power
                    let power = PowerService::get_by_id(&conn, power_id)?
                        .ok_or_else(|| CommandError::NotFound("Power not found".to_string()))?;

                    // Check if power is already in draft.powers
                    if draft.powers.iter().any(|p| p.power.id == power_id) {
                        return Err(CommandError::Validation("Power already added".to_string()));
                    }

                    draft.powers.push(CharacterPowerValue::new(power, None));
                }
            }
            ArcaneBackgroundOptionType::Hindrance => {
                if let Some(hindrance_id) = option.option_id {
                    // Load and add the hindrance
                    let hindrance = HindranceService::get_by_id(&conn, hindrance_id)?
                        .ok_or_else(|| CommandError::NotFound("Hindrance not found".to_string()))?;

                    draft.hindrances.push(CharacterHindranceValue::new(
                        hindrance,
                        SOURCE_ARCANE_BACKGROUND.to_string(),
                    ));
                }
            }
            // Ability and EdgeCategory types are informational, no side effects
            _ => {}
        }

        choice_value.selected_options.push(option);
    } else {
        // Deselecting
        let option_position = choice_value
            .selected_options
            .iter()
            .position(|o| o.id == selected_option_id);

        match option_position {
            Some(idx) => {
                let removed_option = choice_value.selected_options.remove(idx);

                // Handle side effects based on option type
                match removed_option.option_type {
                    ArcaneBackgroundOptionType::Power => {
                        if let Some(power_id) = removed_option.option_id {
                            // Remove the power from draft.powers (only non-locked)
                            if let Some(pos) = draft.powers.iter().position(|p| {
                                p.power.id == power_id && !p.is_locked
                            }) {
                                draft.powers.remove(pos);
                            }
                        }
                    }
                    ArcaneBackgroundOptionType::Hindrance => {
                        if let Some(hindrance_id) = removed_option.option_id {
                            // Remove the hindrance from draft.hindrances
                            if let Some(pos) = draft.hindrances.iter().position(|h| {
                                h.hindrance.id == hindrance_id
                                    && h.source == SOURCE_ARCANE_BACKGROUND
                            }) {
                                draft.hindrances.remove(pos);
                            }
                        }
                    }
                    _ => {}
                }
            }
            None => {
                return Err(CommandError::Validation("Option not currently selected".to_string()));
            }
        }
    }

    Ok(draft.clone())
}

/// Power with its availability status for the current character.
#[derive(serde::Serialize, specta::Type)]
pub struct PowerWithAvailability {
    pub power: PowerView,
    pub is_available: bool,
    pub requirement_statuses: Vec<RequirementStatus>,
}

#[tauri::command]
#[specta::specta]
pub fn get_powers(state: State<Mutex<AppState>>) -> CommandResult<Vec<PowerWithAvailability>> {
    let state = lock_state(&state)?;
    let conn = state.connection()?;
    let powers = PowerService::get_all(&conn)?;

    // If there's a draft character, evaluate requirements against it
    let results = match &state.draft_character {
        Some(draft) => {
            let ctx = draft.to_requirement_context();
            powers
                .into_iter()
                .map(|power| {
                    let is_available = power.requirements.evaluate(&ctx);
                    let requirement_statuses = power.requirements.evaluate_detailed(&ctx);
                    PowerWithAvailability {
                        power,
                        is_available,
                        requirement_statuses,
                    }
                })
                .collect()
        }
        None => powers
            .into_iter()
            .map(|power| PowerWithAvailability {
                is_available: true,
                requirement_statuses: vec![],
                power,
            })
            .collect(),
    };

    Ok(results)
}

#[tauri::command]
#[specta::specta]
pub fn add_draft_power(
    power_id: i64,
    state: State<Mutex<AppState>>,
) -> CommandResult<CharacterView> {
    let mut state = lock_state(&state)?;
    let conn = state.connection()?;

    let draft = state.draft_mut()?;

    // Check if power is already added
    if draft.powers.iter().any(|p| p.power.id == power_id) {
        return Err(CommandError::Validation("Power already added".to_string()));
    }

    // Check if character has an arcane background
    if draft.arcane_backgrounds.is_empty() {
        return Err(CommandError::Validation("Character must have an Arcane Background to select powers".to_string()));
    }

    // Check if the arcane background has a power list restriction
    let has_power_list = draft
        .arcane_backgrounds
        .iter()
        .any(|ab| ab.arcane_background.has_power_list);

    if has_power_list {
        // Validate that the power is in the available_power choices
        let available_power_ids: Vec<i64> = draft
            .arcane_background_choices
            .iter()
            .filter(|c| c.choice.choice_type == ArcaneBackgroundChoiceType::AvailablePower)
            .flat_map(|c| &c.choice.options)
            .filter(|opt| opt.option_type == ArcaneBackgroundOptionType::Power)
            .filter_map(|opt| opt.option_id)
            .collect();

        if !available_power_ids.contains(&power_id) {
            return Err(CommandError::Validation(
                "This power is not available for your Arcane Background".to_string(),
            ));
        }
    }

    // Check if character has room for more starting powers
    // Base slots from arcane backgrounds
    let base_power_slots: i64 = draft
        .arcane_backgrounds
        .iter()
        .map(|ab| ab.arcane_background.starting_powers)
        .sum();

    // Bonus slots from edges like "New Powers"
    let bonus_power_slots: i64 = draft
        .edges
        .iter()
        .flat_map(|e| &e.edge.modifiers)
        .filter(|m| m.target_type.as_deref() == Some("power_slots"))
        .filter_map(|m| m.value)
        .sum();

    let total_starting_powers = base_power_slots + bonus_power_slots;
    let current_power_count = draft.powers.len() as i64;

    if current_power_count >= total_starting_powers {
        return Err(CommandError::Validation(format!(
            "Cannot add more powers. You have {} of {} starting powers.",
            current_power_count, total_starting_powers
        )));
    }

    // Load the power
    let power = PowerService::get_by_id(&conn, power_id)?
        .ok_or_else(|| CommandError::NotFound("Power not found".to_string()))?;

    // Check requirements
    let ctx = draft.to_requirement_context();
    if !power.requirements.evaluate(&ctx) {
        return Err(CommandError::Validation("Character does not meet power requirements".to_string()));
    }

    // Add the power (advance_taken is None for character creation/starting powers)
    draft.powers.push(CharacterPowerValue::new(power, None));

    Ok(draft.clone())
}

#[tauri::command]
#[specta::specta]
pub fn remove_draft_power(
    power_id: i64,
    state: State<Mutex<AppState>>,
) -> CommandResult<CharacterView> {
    let mut state = lock_state(&state)?;

    let draft = state.draft_mut()?;

    // Find the power
    let position = draft.powers.iter().position(|p| p.power.id == power_id);

    match position {
        Some(idx) => {
            // Check if the power is locked (required starting power)
            if draft.powers[idx].is_locked {
                return Err(CommandError::Validation(
                    "Cannot remove a required starting power".to_string(),
                ));
            }

            draft.powers.remove(idx);
            Ok(draft.clone())
        }
        None => Err(CommandError::NotFound("Power not found".to_string())),
    }
}
