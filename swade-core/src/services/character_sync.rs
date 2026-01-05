//! Character data synchronization helpers.
//!
//! These functions handle syncing character data between the in-memory view
//! and the database, including attributes, skills, edges, hindrances, and ancestry choices.

use std::collections::HashSet;

use rusqlite::Connection;

use crate::constants::{SOURCE_ANCESTRY, SOURCE_ARCANE_BACKGROUND, SOURCE_CHOSEN, SOURCE_HINDRANCE_POINTS};
use crate::error::Result;
use crate::models::{
    CharacterAncestryChoice, CharacterAttribute, CharacterEdge, CharacterHindrance, CharacterSkill,
};
use crate::repositories::{
    CharacterAncestryChoiceRepository, CharacterAttributeRepository, CharacterEdgeRepository,
    CharacterHindranceRepository, CharacterSkillRepository,
};
use crate::views::{
    AncestryOptionType, CharacterAncestryChoiceValue,
    CharacterAttributeValue, CharacterEdgeValue, CharacterHindranceValue, CharacterSkillValue,
    CharacterView,
};

/// Insert ancestry choices and create the resulting edges/hindrances.
pub fn insert_ancestry_choices(
    conn: &Connection,
    character_id: i64,
    ancestry_choices: &[CharacterAncestryChoiceValue],
    now: &str,
) -> Result<()> {
    for choice_value in ancestry_choices {
        if let Some(selected_option) = &choice_value.selected_option {
            // Save the choice record
            let char_choice = CharacterAncestryChoice {
                id: 0,
                character_id,
                choice_id: choice_value.choice.id,
                selected_option_id: selected_option.id,
                created_at: now.to_string(),
                updated_at: now.to_string(),
            };
            CharacterAncestryChoiceRepository::insert(conn, &char_choice)?;

            // Also insert the edge/hindrance based on option type
            if let Some(option_id) = selected_option.option_id {
                insert_ancestry_selection(
                    conn,
                    character_id,
                    selected_option.option_type,
                    option_id,
                    now,
                )?;
            }
        }
    }
    Ok(())
}

/// Remove an ancestry-sourced edge or hindrance.
fn remove_ancestry_selection(
    conn: &Connection,
    character_id: i64,
    option_type: AncestryOptionType,
    option_id: i64,
) -> Result<()> {
    match option_type {
        AncestryOptionType::Edge => {
            let edges = CharacterEdgeRepository::get_by_character_id(conn, character_id)?;
            for edge in edges {
                if edge.edge_id == option_id && edge.source == SOURCE_ANCESTRY {
                    CharacterEdgeRepository::delete(conn, edge.id)?;
                    break;
                }
            }
        }
        AncestryOptionType::Hindrance => {
            let hindrances = CharacterHindranceRepository::get_by_character_id(conn, character_id)?;
            for h in hindrances {
                if h.hindrance_id == option_id && h.source == SOURCE_ANCESTRY {
                    CharacterHindranceRepository::delete(conn, h.id)?;
                    break;
                }
            }
        }
        AncestryOptionType::Ancestry => {}
    }
    Ok(())
}

/// Insert an ancestry-sourced edge or hindrance.
fn insert_ancestry_selection(
    conn: &Connection,
    character_id: i64,
    option_type: AncestryOptionType,
    option_id: i64,
    now: &str,
) -> Result<()> {
    match option_type {
        AncestryOptionType::Edge => {
            let char_edge = CharacterEdge {
                id: 0,
                character_id,
                edge_id: option_id,
                advance_taken: 0,
                notes: None,
                source: SOURCE_ANCESTRY.to_string(),
                created_at: now.to_string(),
                updated_at: now.to_string(),
            };
            CharacterEdgeRepository::insert(conn, &char_edge)?;
        }
        AncestryOptionType::Hindrance => {
            let char_hindrance = CharacterHindrance {
                id: 0,
                character_id,
                hindrance_id: option_id,
                source: SOURCE_ANCESTRY.to_string(),
                created_at: now.to_string(),
                updated_at: now.to_string(),
            };
            CharacterHindranceRepository::insert(conn, &char_hindrance)?;
        }
        AncestryOptionType::Ancestry => {}
    }
    Ok(())
}

/// Sync ancestry choices when ancestry hasn't changed but selections may have.
pub fn sync_ancestry_choices(
    conn: &Connection,
    character_id: i64,
    existing: &CharacterView,
    new: &CharacterView,
    now: &str,
) -> Result<()> {
    // Build maps of choice_id -> selected_option for comparison
    let existing_selections: std::collections::HashMap<i64, Option<i64>> = existing
        .ancestry_choices
        .iter()
        .map(|c| (c.choice.id, c.selected_option.as_ref().map(|o| o.id)))
        .collect();

    // For each choice in the new character
    for choice_value in &new.ancestry_choices {
        let choice_id = choice_value.choice.id;
        let new_option = choice_value.selected_option.as_ref();
        let old_option_id = existing_selections.get(&choice_id).copied().flatten();
        let new_option_id = new_option.map(|o| o.id);

        if old_option_id != new_option_id {
            // Selection changed - remove old selection
            if let Some(old_choice_value) = existing
                .ancestry_choices
                .iter()
                .find(|c| c.choice.id == choice_id)
                && let Some(old_selected) = &old_choice_value.selected_option
                && let Some(old_entity_id) = old_selected.option_id
            {
                remove_ancestry_selection(
                    conn,
                    character_id,
                    old_selected.option_type,
                    old_entity_id,
                )?;
            }

            // Delete old choice record
            let old_choices =
                CharacterAncestryChoiceRepository::get_by_character_id(conn, character_id)?;
            for old_choice in old_choices {
                if old_choice.choice_id == choice_id {
                    CharacterAncestryChoiceRepository::delete(conn, old_choice.id)?;
                    break;
                }
            }

            // Insert new choice and edge/hindrance if a new option was selected
            if let Some(selected_option) = new_option {
                let char_choice = CharacterAncestryChoice {
                    id: 0,
                    character_id,
                    choice_id,
                    selected_option_id: selected_option.id,
                    created_at: now.to_string(),
                    updated_at: now.to_string(),
                };
                CharacterAncestryChoiceRepository::insert(conn, &char_choice)?;

                if let Some(option_id) = selected_option.option_id {
                    insert_ancestry_selection(
                        conn,
                        character_id,
                        selected_option.option_type,
                        option_id,
                        now,
                    )?;
                }
            }
        }
    }

    Ok(())
}

/// Sync character attributes by updating steps_incremented.
pub fn sync_attributes(
    conn: &Connection,
    character_id: i64,
    _existing: &[CharacterAttributeValue],
    new: &[CharacterAttributeValue],
    now: &str,
) -> Result<()> {
    // Get existing DB records to get their IDs
    let db_records = CharacterAttributeRepository::get_by_character_id(conn, character_id)?;

    for new_attr in new {
        // Find the corresponding DB record
        if let Some(db_record) = db_records
            .iter()
            .find(|r| r.attribute_id == new_attr.attribute.id)
        {
            let new_steps = new_attr.die.steps_from(new_attr.attribute.base_die) as i64;

            // Only update if steps changed
            if db_record.steps_incremented != new_steps {
                let updated = CharacterAttribute {
                    id: db_record.id,
                    character_id,
                    attribute_id: new_attr.attribute.id,
                    steps_incremented: new_steps,
                    created_at: db_record.created_at.clone(),
                    updated_at: now.to_string(),
                };
                CharacterAttributeRepository::update(conn, &updated)?;
            }
        }
    }

    Ok(())
}

/// Sync character skills by updating die values.
pub fn sync_skills(
    conn: &Connection,
    character_id: i64,
    _existing: &[CharacterSkillValue],
    new: &[CharacterSkillValue],
    now: &str,
) -> Result<()> {
    // Get existing DB records
    let db_records = CharacterSkillRepository::get_by_character_id(conn, character_id)?;

    // Build set of existing skill IDs
    let existing_skill_ids: HashSet<i64> = db_records.iter().map(|r| r.skill_id).collect();

    // Build set of new skill IDs
    let new_skill_ids: HashSet<i64> = new.iter().map(|s| s.skill.id).collect();

    // Update existing skills
    for new_skill in new {
        if let Some(db_record) = db_records
            .iter()
            .find(|r| r.skill_id == new_skill.skill.id)
        {
            let new_die_size = new_skill.die.as_ref().map(|d| d.size() as i64);
            let new_die_mod = new_skill
                .die
                .as_ref()
                .map(|d| d.modifier() as i64)
                .unwrap_or(0);

            // Only update if die changed
            if db_record.current_die_size != new_die_size
                || db_record.current_die_modifier != new_die_mod
            {
                let updated = CharacterSkill {
                    id: db_record.id,
                    character_id,
                    skill_id: new_skill.skill.id,
                    current_die_size: new_die_size,
                    current_die_modifier: new_die_mod,
                    created_at: db_record.created_at.clone(),
                    updated_at: now.to_string(),
                };
                CharacterSkillRepository::update(conn, &updated)?;
            }
        }
    }

    // Insert new skills (skills that weren't in the DB before)
    for new_skill in new {
        if !existing_skill_ids.contains(&new_skill.skill.id) {
            let char_skill = CharacterSkill {
                id: 0,
                character_id,
                skill_id: new_skill.skill.id,
                current_die_size: new_skill.die.as_ref().map(|d| d.size() as i64),
                current_die_modifier: new_skill
                    .die
                    .as_ref()
                    .map(|d| d.modifier() as i64)
                    .unwrap_or(0),
                created_at: now.to_string(),
                updated_at: now.to_string(),
            };
            CharacterSkillRepository::insert(conn, &char_skill)?;
        }
    }

    // Delete removed skills (skills that are no longer in the new list)
    for db_record in &db_records {
        if !new_skill_ids.contains(&db_record.skill_id) {
            CharacterSkillRepository::delete(conn, db_record.id)?;
        }
    }

    Ok(())
}

/// Sync hindrances (chosen and arcane_background sources, not ancestry-sourced).
pub fn sync_hindrances(
    conn: &Connection,
    character_id: i64,
    _existing: &[CharacterHindranceValue],
    new: &[CharacterHindranceValue],
    now: &str,
) -> Result<()> {
    // Get existing DB records
    let db_records = CharacterHindranceRepository::get_by_character_id(conn, character_id)?;

    // Sources we handle in this function (ancestry hindrances handled separately)
    let synced_sources = [SOURCE_CHOSEN, SOURCE_ARCANE_BACKGROUND];

    // Build sets of existing hindrance IDs by source
    let existing_ids_by_source: HashSet<(i64, &str)> = db_records
        .iter()
        .filter(|r| synced_sources.contains(&r.source.as_str()))
        .map(|r| (r.hindrance_id, r.source.as_str()))
        .collect();

    // Build sets of new hindrance IDs by source
    let new_ids_by_source: HashSet<(i64, &str)> = new
        .iter()
        .filter(|h| synced_sources.contains(&h.source.as_str()))
        .map(|h| (h.hindrance.id, h.source.as_str()))
        .collect();

    // Delete removed hindrances
    for db_record in &db_records {
        if synced_sources.contains(&db_record.source.as_str()) {
            let key = (db_record.hindrance_id, db_record.source.as_str());
            if !new_ids_by_source.contains(&key) {
                CharacterHindranceRepository::delete(conn, db_record.id)?;
            }
        }
    }

    // Insert new hindrances
    for new_hindrance in new {
        if synced_sources.contains(&new_hindrance.source.as_str()) {
            let key = (new_hindrance.hindrance.id, new_hindrance.source.as_str());
            if !existing_ids_by_source.contains(&key) {
                let char_hindrance = CharacterHindrance {
                    id: 0,
                    character_id,
                    hindrance_id: new_hindrance.hindrance.id,
                    source: new_hindrance.source.clone(),
                    created_at: now.to_string(),
                    updated_at: now.to_string(),
                };
                CharacterHindranceRepository::insert(conn, &char_hindrance)?;
            }
        }
    }

    Ok(())
}

/// Sync edges from hindrance_points (not ancestry-sourced).
pub fn sync_edges(
    conn: &Connection,
    character_id: i64,
    _existing: &[CharacterEdgeValue],
    new: &[CharacterEdgeValue],
    now: &str,
) -> Result<()> {
    // Get existing DB records
    let db_records = CharacterEdgeRepository::get_by_character_id(conn, character_id)?;

    // Only consider "hindrance_points" edges for sync (ancestry edges handled separately)
    // For edges that can be taken multiple times, we need to track by (edge_id, notes) pair
    let existing_hp_edges: Vec<_> = db_records
        .iter()
        .filter(|r| r.source == SOURCE_HINDRANCE_POINTS)
        .collect();

    let new_hp_edges: Vec<_> = new
        .iter()
        .filter(|e| e.source == SOURCE_HINDRANCE_POINTS)
        .collect();

    // Build sets for comparison - using (edge_id, notes) as the key
    let existing_keys: HashSet<(i64, Option<&str>)> = existing_hp_edges
        .iter()
        .map(|e| (e.edge_id, e.notes.as_deref()))
        .collect();

    let new_keys: HashSet<(i64, Option<&str>)> = new_hp_edges
        .iter()
        .map(|e| (e.edge.id, e.notes.as_deref()))
        .collect();

    // Delete removed edges
    for db_record in &existing_hp_edges {
        let key = (db_record.edge_id, db_record.notes.as_deref());
        if !new_keys.contains(&key) {
            CharacterEdgeRepository::delete(conn, db_record.id)?;
        }
    }

    // Insert new edges
    for new_edge in &new_hp_edges {
        let key = (new_edge.edge.id, new_edge.notes.as_deref());
        if !existing_keys.contains(&key) {
            let char_edge = CharacterEdge {
                id: 0,
                character_id,
                edge_id: new_edge.edge.id,
                advance_taken: new_edge.advance_taken,
                notes: new_edge.notes.clone(),
                source: SOURCE_HINDRANCE_POINTS.to_string(),
                created_at: now.to_string(),
                updated_at: now.to_string(),
            };
            CharacterEdgeRepository::insert(conn, &char_edge)?;
        }
    }

    Ok(())
}
