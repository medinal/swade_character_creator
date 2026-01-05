//! Character data loading helpers.
//!
//! These functions handle loading character-related data from the database
//! into view models.

use rusqlite::Connection;

use crate::error::Result;
use crate::repositories::{
    AncestryChoiceRepository, AttributeRepository, CharacterAncestryChoiceRepository,
    CharacterArcaneBackgroundRepository, CharacterAttributeRepository, CharacterEdgeRepository,
    CharacterGearRepository, CharacterHindranceRepository, CharacterModifierRepository,
    CharacterNoteRepository, CharacterPowerRepository, CharacterSkillRepository, ModifierRepository,
};
use crate::services::{
    ArcaneBackgroundService, EdgeService, GearService, HindranceService, PowerService, SkillService,
};
use crate::views::{
    AncestryChoiceOptionView, AncestryChoiceView, CharacterAncestryChoiceValue,
    CharacterArcaneBackgroundValue, CharacterAttributeValue, CharacterEdgeValue,
    CharacterGearValue, CharacterHindranceValue, CharacterNoteValue, CharacterPowerValue,
    CharacterSkillValue, Die,
};

/// Load ancestry choices for a character.
pub fn load_ancestry_choices(
    conn: &Connection,
    character_id: i64,
) -> Result<Vec<CharacterAncestryChoiceValue>> {
    let character_choices =
        CharacterAncestryChoiceRepository::get_by_character_id(conn, character_id)?;

    let mut values = Vec::new();
    for cc in character_choices {
        // Load the choice
        let choice = match AncestryChoiceRepository::get_choice_by_id(conn, cc.choice_id)? {
            Some(c) => c,
            None => continue,
        };

        // Load the choice's options
        let options = AncestryChoiceRepository::get_options_by_choice_id(conn, cc.choice_id)?;
        let option_views: Vec<AncestryChoiceOptionView> = options
            .into_iter()
            .map(AncestryChoiceOptionView::new)
            .collect();

        let choice_view = AncestryChoiceView::new(choice, option_views);

        // Load the selected option
        let selected_option = AncestryChoiceRepository::get_option_by_id(conn, cc.selected_option_id)?
            .map(AncestryChoiceOptionView::new);

        values.push(CharacterAncestryChoiceValue::new(choice_view, selected_option));
    }

    Ok(values)
}

/// Load attributes for a character.
pub fn load_attributes(
    conn: &Connection,
    character_id: i64,
) -> Result<Vec<CharacterAttributeValue>> {
    let character_attributes =
        CharacterAttributeRepository::get_by_character_id(conn, character_id)?;

    let mut values = Vec::new();
    for ca in character_attributes {
        if let Some(attribute) = AttributeRepository::get_by_id(conn, ca.attribute_id)? {
            let attr_view = crate::views::AttributeView::new(attribute);
            // Start with base die and increment for each step taken
            let mut die = attr_view.base_die;
            for _ in 0..ca.steps_incremented {
                die = die.increment();
            }
            values.push(CharacterAttributeValue::new(attr_view, die));
        }
    }

    Ok(values)
}

/// Load skills for a character (all skills, with character's training applied).
pub fn load_skills(conn: &Connection, character_id: i64) -> Result<Vec<CharacterSkillValue>> {
    // Load all skills from the database
    let all_skills = SkillService::get_all(conn)?;

    // Load character's trained skills
    let character_skills = CharacterSkillRepository::get_by_character_id(conn, character_id)?;

    // Create a map of skill_id -> character skill data for quick lookup
    let char_skill_map: std::collections::HashMap<i64, _> = character_skills
        .into_iter()
        .map(|cs| (cs.skill_id, cs))
        .collect();

    // Build skill values for all skills, merging with character data where available
    let values = all_skills
        .into_iter()
        .map(|skill_view| {
            let die = char_skill_map.get(&skill_view.id).and_then(|cs| {
                cs.current_die_size
                    .and_then(|size| Die::with_modifier(size as u8, cs.current_die_modifier as u8))
            });
            CharacterSkillValue::new(skill_view, die)
        })
        .collect();

    Ok(values)
}

/// Load edges for a character.
pub fn load_edges(conn: &Connection, character_id: i64) -> Result<Vec<CharacterEdgeValue>> {
    let character_edges = CharacterEdgeRepository::get_by_character_id(conn, character_id)?;

    let mut values = Vec::new();
    for ce in character_edges {
        if let Some(edge_view) = EdgeService::get_by_id(conn, ce.edge_id)? {
            values.push(CharacterEdgeValue::new(
                edge_view,
                ce.advance_taken,
                ce.notes,
                ce.source,
            ));
        }
    }

    Ok(values)
}

/// Load hindrances for a character.
pub fn load_hindrances(
    conn: &Connection,
    character_id: i64,
) -> Result<Vec<CharacterHindranceValue>> {
    let character_hindrances =
        CharacterHindranceRepository::get_by_character_id(conn, character_id)?;

    let mut values = Vec::new();
    for ch in character_hindrances {
        if let Some(hindrance_view) = HindranceService::get_by_id(conn, ch.hindrance_id)? {
            values.push(CharacterHindranceValue::new(hindrance_view, ch.source));
        }
    }

    Ok(values)
}

/// Load arcane backgrounds for a character.
pub fn load_arcane_backgrounds(
    conn: &Connection,
    character_id: i64,
) -> Result<Vec<CharacterArcaneBackgroundValue>> {
    let character_abs =
        CharacterArcaneBackgroundRepository::get_by_character_id(conn, character_id)?;

    let mut values = Vec::new();
    for cab in character_abs {
        if let Some(ab_view) = ArcaneBackgroundService::get_by_id(conn, cab.arcane_background_id)? {
            values.push(CharacterArcaneBackgroundValue::new(ab_view, cab.advance_taken));
        }
    }

    Ok(values)
}

/// Load powers for a character.
pub fn load_powers(conn: &Connection, character_id: i64) -> Result<Vec<CharacterPowerValue>> {
    let character_powers = CharacterPowerRepository::get_by_character_id(conn, character_id)?;

    let mut values = Vec::new();
    for cp in character_powers {
        if let Some(power_view) = PowerService::get_by_id(conn, cp.power_id)? {
            values.push(CharacterPowerValue::new(power_view, cp.advance_taken));
        }
    }

    Ok(values)
}

/// Load modifiers directly attached to a character.
pub fn load_modifiers(conn: &Connection, character_id: i64) -> Result<Vec<crate::models::Modifier>> {
    let character_modifiers =
        CharacterModifierRepository::get_by_character_id(conn, character_id)?;

    let mut modifiers = Vec::new();
    for cm in character_modifiers {
        if let Some(modifier) = ModifierRepository::get_by_id(conn, cm.modifier_id)? {
            modifiers.push(modifier);
        }
    }

    Ok(modifiers)
}

/// Load notes for a character (newest first).
pub fn load_notes(conn: &Connection, character_id: i64) -> Result<Vec<CharacterNoteValue>> {
    let notes = CharacterNoteRepository::get_by_character_id(conn, character_id)?;

    Ok(notes
        .into_iter()
        .map(|n| CharacterNoteValue::new(n.id, n.title, n.body, n.created_at, n.updated_at))
        .collect())
}

/// Load gear for a character.
pub fn load_gear(conn: &Connection, character_id: i64) -> Result<Vec<CharacterGearValue>> {
    let character_gear = CharacterGearRepository::get_by_character_id(conn, character_id)?;

    let mut values = Vec::new();
    for cg in character_gear {
        if let Some(gear_view) = GearService::get_by_id(conn, cg.gear_id)? {
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
