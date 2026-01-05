//! Character service for managing character views.

use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use rusqlite::Connection;

use crate::constants::{SOURCE_CHOSEN, SOURCE_HINDRANCE_POINTS};
use crate::error::{Result, SwadeError};
use crate::models::{
    Character, CharacterArcaneBackground, CharacterAttribute, CharacterEdge, CharacterHindrance,
    CharacterPower, CharacterSkill,
};
use crate::repositories::{
    AttributeRepository, CharacterAdvanceRepository, CharacterAncestryChoiceRepository,
    CharacterArcaneBackgroundRepository, CharacterAttributeRepository, CharacterEdgeRepository,
    CharacterHindranceRepository, CharacterPowerRepository, CharacterRepository,
    CharacterSkillRepository, RankRepository, SkillRepository,
};
use crate::services::AncestryService;
use crate::views::{
    CharacterAttributeValue, CharacterSkillValue, CharacterView, DerivedStatsView, Die,
    EncumbranceInfo,
};

use super::character_load;
use super::character_sync;

pub struct CharacterService;

/// Convert portrait bytes and mime type to a data URL.
fn portrait_to_data_url(portrait: Option<&[u8]>, mime_type: Option<&str>) -> Option<String> {
    match (portrait, mime_type) {
        (Some(bytes), Some(mime)) if !bytes.is_empty() => {
            let encoded = BASE64.encode(bytes);
            Some(format!("data:{};base64,{}", mime, encoded))
        }
        _ => None,
    }
}

impl CharacterService {
    /// Get all characters.
    pub fn get_all(conn: &Connection) -> Result<Vec<CharacterView>> {
        let characters = CharacterRepository::get_all(conn)?;

        let mut views = Vec::new();
        for character in characters {
            if let Some(view) = Self::get_by_id(conn, character.id)? {
                views.push(view);
            }
        }

        Ok(views)
    }

    /// Get a character by ID.
    pub fn get_by_id(conn: &Connection, id: i64) -> Result<Option<CharacterView>> {
        let character = match CharacterRepository::get_by_id(conn, id)? {
            Some(c) => c,
            None => return Ok(None),
        };

        // Count advances and derive rank
        let current_advances = CharacterAdvanceRepository::count_by_character_id(conn, id)?;
        let rank = Self::get_rank_for_advances(conn, current_advances)?;

        // Load ancestry if present
        let ancestry = match character.ancestry_id {
            Some(ancestry_id) => AncestryService::get_by_id(conn, ancestry_id)?,
            None => None,
        };

        // Load all related data using helper functions
        let ancestry_choices = character_load::load_ancestry_choices(conn, id)?;
        let attributes = character_load::load_attributes(conn, id)?;
        let skills = character_load::load_skills(conn, id)?;
        let edges = character_load::load_edges(conn, id)?;
        let hindrances = character_load::load_hindrances(conn, id)?;
        let arcane_backgrounds = character_load::load_arcane_backgrounds(conn, id)?;
        let powers = character_load::load_powers(conn, id)?;
        let modifiers = character_load::load_modifiers(conn, id)?;
        let notes = character_load::load_notes(conn, id)?;
        let gear = character_load::load_gear(conn, id)?;

        // Convert portrait to data URL
        let portrait_data_url = portrait_to_data_url(
            character.portrait.as_deref(),
            character.portrait_mime_type.as_deref(),
        );

        let mut character_view = CharacterView {
            id: character.id,
            is_wild_card: character.is_wild_card,
            name: character.name,
            ancestry,
            ancestry_choices,
            rank,
            current_advances,
            attributes,
            skills,
            edges,
            hindrances,
            arcane_backgrounds,
            arcane_background_choices: vec![], // TODO: Load from character_arcane_background_choices
            powers,
            power_points: character.power_points,
            power_points_used: character.power_points_used,
            wounds: character.wounds,
            fatigue: character.fatigue,
            notes,
            gear,
            encumbrance: EncumbranceInfo::empty(4), // Will be computed by compute_effective_values
            modifiers,
            derived_stats: DerivedStatsView::default(),
            attribute_points_spent: character.attribute_points_spent,
            attribute_points_earned: character.attribute_points_earned,
            skill_points_spent: character.skill_points_spent,
            skill_points_earned: character.skill_points_earned,
            hindrance_points_spent: character.hindrance_points_spent,
            hindrance_points_earned: character.hindrance_points_earned,
            hindrance_points_to_edges: character.hindrance_points_to_edges,
            hindrance_points_to_attributes: character.hindrance_points_to_attributes,
            hindrance_points_to_skills: character.hindrance_points_to_skills,
            hindrance_points_to_wealth: character.hindrance_points_to_wealth,
            wealth: character.wealth,
            background: character.background,
            description: character.description,
            portrait_data_url,
        };

        // Compute effective values based on modifiers
        character_view.compute_effective_values();

        Ok(Some(character_view))
    }

    /// Build a new character in memory with default attributes and core skills (does not persist).
    pub fn build_new(conn: &Connection, name: String) -> Result<CharacterView> {
        // Get the Novice rank (first rank with min_advances = 0)
        let ranks = RankRepository::get_all(conn)?;
        let novice_rank = ranks
            .into_iter()
            .find(|r| r.min_advances == 0)
            .ok_or_else(|| SwadeError::NotFound("Novice rank (min_advances = 0)".to_string()))?;

        // Build attributes at base d4
        let attributes = AttributeRepository::get_all(conn)?;
        let attribute_values: Vec<CharacterAttributeValue> = attributes
            .into_iter()
            .map(|attr| {
                let attr_view = crate::views::AttributeView::new(attr);
                CharacterAttributeValue::new(attr_view.clone(), attr_view.base_die)
            })
            .collect();

        // Build all skills - core skills at d4, non-core skills untrained (None)
        let all_skills = SkillRepository::get_all(conn)?;
        let skill_values: Vec<CharacterSkillValue> = all_skills
            .into_iter()
            .map(|skill| {
                let die = if skill.is_core_skill {
                    Some(Die::d4())
                } else {
                    None
                };
                let skill_view = crate::views::SkillView::new(skill);
                CharacterSkillValue::new(skill_view, die)
            })
            .collect();

        let mut character_view = CharacterView {
            id: 0, // Not yet persisted
            is_wild_card: true,
            name,
            ancestry: None,
            ancestry_choices: vec![],
            rank: novice_rank,
            current_advances: 0,
            attributes: attribute_values,
            skills: skill_values,
            edges: vec![],
            hindrances: vec![],
            arcane_backgrounds: vec![],
            arcane_background_choices: vec![],
            powers: vec![],
            power_points: 0,
            power_points_used: 0,
            wounds: 0,
            fatigue: 0,
            notes: vec![],
            gear: vec![],
            encumbrance: EncumbranceInfo::empty(4), // Will be computed by compute_effective_values
            modifiers: vec![],
            derived_stats: DerivedStatsView::default(),
            attribute_points_spent: 0,
            attribute_points_earned: 5, // Standard starting attribute points
            skill_points_spent: 0,
            skill_points_earned: 12, // Standard starting skill points
            hindrance_points_spent: 0,
            hindrance_points_earned: 0,
            hindrance_points_to_edges: 0,
            hindrance_points_to_attributes: 0,
            hindrance_points_to_skills: 0,
            hindrance_points_to_wealth: 0,
            wealth: 500,
            background: None,
            description: None,
            portrait_data_url: None,
        };

        // Compute effective values (no-op for new character without modifiers)
        character_view.compute_effective_values();

        Ok(character_view)
    }

    /// Save a character view to the database (persists a new character or updates existing).
    pub fn save(conn: &Connection, character: &CharacterView) -> Result<CharacterView> {
        // Use a transaction for atomicity
        conn.execute("BEGIN TRANSACTION", [])?;

        let result = Self::save_inner(conn, character);

        match result {
            Ok(saved) => {
                conn.execute("COMMIT", [])?;
                Ok(saved)
            }
            Err(e) => {
                conn.execute("ROLLBACK", [])?;
                Err(e)
            }
        }
    }

    fn save_inner(conn: &Connection, character: &CharacterView) -> Result<CharacterView> {
        let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();

        // Create the character model
        // Note: portrait is NOT included here - it's updated via a separate command
        // to avoid sending large blobs with every save
        let char_model = Character {
            id: character.id,
            is_wild_card: character.is_wild_card,
            name: character.name.clone(),
            ancestry_id: character.ancestry.as_ref().map(|a| a.id),
            attribute_points_spent: character.attribute_points_spent,
            attribute_points_earned: character.attribute_points_earned,
            skill_points_spent: character.skill_points_spent,
            skill_points_earned: character.skill_points_earned,
            hindrance_points_spent: character.hindrance_points_spent,
            hindrance_points_earned: character.hindrance_points_earned,
            hindrance_points_to_edges: character.hindrance_points_to_edges,
            hindrance_points_to_attributes: character.hindrance_points_to_attributes,
            hindrance_points_to_skills: character.hindrance_points_to_skills,
            hindrance_points_to_wealth: character.hindrance_points_to_wealth,
            power_points: character.power_points,
            power_points_used: character.power_points_used,
            wounds: character.wounds,
            fatigue: character.fatigue,
            wealth: character.wealth,
            background: character.background.clone(),
            description: character.description.clone(),
            portrait: None, // Updated via separate command
            portrait_mime_type: None,
            created_at: now.clone(),
            updated_at: now.clone(),
        };

        if character.id == 0 {
            // New character - use insert logic
            Self::save_new_character(conn, &char_model, character, &now)
        } else {
            // Existing character - use update logic
            Self::save_existing_character(conn, &char_model, character, &now)
        }
    }

    fn save_new_character(
        conn: &Connection,
        char_model: &Character,
        character: &CharacterView,
        now: &str,
    ) -> Result<CharacterView> {
        let character_id = CharacterRepository::insert(conn, char_model)?;

        // Insert attributes
        for attr_value in &character.attributes {
            let steps = attr_value.die.steps_from(attr_value.attribute.base_die);
            let char_attr = CharacterAttribute {
                id: 0,
                character_id,
                attribute_id: attr_value.attribute.id,
                steps_incremented: steps as i64,
                created_at: now.to_string(),
                updated_at: now.to_string(),
            };
            CharacterAttributeRepository::insert(conn, &char_attr)?;
        }

        // Insert skills
        for skill_value in &character.skills {
            let char_skill = CharacterSkill {
                id: 0,
                character_id,
                skill_id: skill_value.skill.id,
                current_die_size: skill_value.die.as_ref().map(|d| d.size() as i64),
                current_die_modifier: skill_value
                    .die
                    .as_ref()
                    .map(|d| d.modifier() as i64)
                    .unwrap_or(0),
                created_at: now.to_string(),
                updated_at: now.to_string(),
            };
            CharacterSkillRepository::insert(conn, &char_skill)?;
        }

        // Insert ancestry choices and their resulting edges/hindrances
        character_sync::insert_ancestry_choices(conn, character_id, &character.ancestry_choices, now)?;

        // Insert hindrances (chosen during character creation)
        for hindrance_value in &character.hindrances {
            if hindrance_value.source == SOURCE_CHOSEN {
                let char_hindrance = CharacterHindrance {
                    id: 0,
                    character_id,
                    hindrance_id: hindrance_value.hindrance.id,
                    source: hindrance_value.source.clone(),
                    created_at: now.to_string(),
                    updated_at: now.to_string(),
                };
                CharacterHindranceRepository::insert(conn, &char_hindrance)?;
            }
        }

        // Insert edges (from hindrance_points during character creation)
        for edge_value in &character.edges {
            if edge_value.source == SOURCE_HINDRANCE_POINTS {
                let char_edge = CharacterEdge {
                    id: 0,
                    character_id,
                    edge_id: edge_value.edge.id,
                    advance_taken: edge_value.advance_taken,
                    notes: edge_value.notes.clone(),
                    source: edge_value.source.clone(),
                    created_at: now.to_string(),
                    updated_at: now.to_string(),
                };
                CharacterEdgeRepository::insert(conn, &char_edge)?;
            }
        }

        // Insert arcane backgrounds
        for ab_value in &character.arcane_backgrounds {
            let char_ab = CharacterArcaneBackground {
                id: 0,
                character_id,
                arcane_background_id: ab_value.arcane_background.id,
                advance_taken: ab_value.advance_taken,
                created_at: now.to_string(),
                updated_at: now.to_string(),
            };
            CharacterArcaneBackgroundRepository::insert(conn, &char_ab)?;
        }

        // Insert powers
        for power_value in &character.powers {
            let char_power = CharacterPower {
                id: 0,
                character_id,
                power_id: power_value.power.id,
                advance_taken: power_value.advance_taken,
                created_at: now.to_string(),
                updated_at: now.to_string(),
            };
            CharacterPowerRepository::insert(conn, &char_power)?;
        }

        Self::get_by_id(conn, character_id)?
            .ok_or_else(|| crate::error::SwadeError::NotFound("Character".to_string()))
    }

    fn save_existing_character(
        conn: &Connection,
        char_model: &Character,
        character: &CharacterView,
        now: &str,
    ) -> Result<CharacterView> {
        let character_id = character.id;

        // Load the existing character to detect changes
        let existing = Self::get_by_id(conn, character_id)?
            .ok_or_else(|| crate::error::SwadeError::NotFound("Character".to_string()))?;

        // Update the main character record
        CharacterRepository::update(conn, char_model)?;

        // Check if ancestry changed
        let old_ancestry_id = existing.ancestry.as_ref().map(|a| a.id);
        let new_ancestry_id = character.ancestry.as_ref().map(|a| a.id);
        let ancestry_changed = old_ancestry_id != new_ancestry_id;

        if ancestry_changed {
            // Delete ancestry-sourced edges and hindrances
            CharacterEdgeRepository::delete_by_character_id_and_source(
                conn,
                character_id,
                "ancestry",
            )?;
            CharacterHindranceRepository::delete_by_character_id_and_source(
                conn,
                character_id,
                "ancestry",
            )?;
            // Delete old ancestry choices
            CharacterAncestryChoiceRepository::delete_by_character_id(conn, character_id)?;

            // Insert new ancestry choices (if new ancestry is set)
            if character.ancestry.is_some() {
                character_sync::insert_ancestry_choices(
                    conn,
                    character_id,
                    &character.ancestry_choices,
                    now,
                )?;
            }
        } else {
            // Ancestry didn't change - sync ancestry choices (selected options may have changed)
            character_sync::sync_ancestry_choices(conn, character_id, &existing, character, now)?;
        }

        // Sync attributes
        character_sync::sync_attributes(
            conn,
            character_id,
            &existing.attributes,
            &character.attributes,
            now,
        )?;

        // Sync skills
        character_sync::sync_skills(conn, character_id, &existing.skills, &character.skills, now)?;

        // Sync chosen hindrances
        character_sync::sync_hindrances(
            conn,
            character_id,
            &existing.hindrances,
            &character.hindrances,
            now,
        )?;

        // Sync edges from hindrance_points
        character_sync::sync_edges(conn, character_id, &existing.edges, &character.edges, now)?;

        // Sync arcane backgrounds (delete and re-insert for simplicity)
        CharacterArcaneBackgroundRepository::delete_by_character_id(conn, character_id)?;
        for ab_value in &character.arcane_backgrounds {
            let char_ab = CharacterArcaneBackground {
                id: 0,
                character_id,
                arcane_background_id: ab_value.arcane_background.id,
                advance_taken: ab_value.advance_taken,
                created_at: now.to_string(),
                updated_at: now.to_string(),
            };
            CharacterArcaneBackgroundRepository::insert(conn, &char_ab)?;
        }

        // Sync powers (delete and re-insert for simplicity)
        CharacterPowerRepository::delete_by_character_id(conn, character_id)?;
        for power_value in &character.powers {
            let char_power = CharacterPower {
                id: 0,
                character_id,
                power_id: power_value.power.id,
                advance_taken: power_value.advance_taken,
                created_at: now.to_string(),
                updated_at: now.to_string(),
            };
            CharacterPowerRepository::insert(conn, &char_power)?;
        }

        Self::get_by_id(conn, character_id)?
            .ok_or_else(|| crate::error::SwadeError::NotFound("Character".to_string()))
    }

    /// Get the rank for a given number of advances.
    fn get_rank_for_advances(conn: &Connection, advances: i64) -> Result<crate::models::Rank> {
        let ranks = RankRepository::get_all(conn)?;

        // Find the rank where advances falls within [min_advances, max_advances]
        // Ranks are ordered by min_advances
        for rank in ranks.iter().rev() {
            if advances >= rank.min_advances {
                // Check max_advances (None means no upper limit, i.e., Legendary)
                if rank.max_advances.is_none() || advances <= rank.max_advances.unwrap() {
                    return Ok(rank.clone());
                }
            }
        }

        // Default to first rank (Novice) if something goes wrong
        ranks
            .into_iter()
            .next()
            .ok_or_else(|| SwadeError::NotFound("No ranks found".to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::setup_test_db;
    use rusqlite::params;

    fn insert_test_rank(conn: &Connection, id: i64, name: &str) {
        conn.execute(
            "INSERT INTO ranks (id, name, min_advances, max_advances, description, created_at, updated_at)
             VALUES (?, ?, 0, 3, 'Test rank', '2024-01-01', '2024-01-01')",
            params![id, name],
        )
        .unwrap();
    }

    fn insert_test_character(conn: &Connection, id: i64, name: &str) {
        conn.execute(
            "INSERT INTO characters (id, is_wild_card, name, ancestry_id,
                                    attribute_points_spent, attribute_points_earned,
                                    skill_points_spent, skill_points_earned, hindrance_points_spent,
                                    hindrance_points_earned, hindrance_points_to_edges,
                                    hindrance_points_to_attributes, hindrance_points_to_skills,
                                    hindrance_points_to_wealth, power_points, wealth,
                                    created_at, updated_at)
             VALUES (?, 1, ?, NULL, 0, 5, 0, 12, 0, 0, 0, 0, 0, 0, 0, 500,
                     '2024-01-01', '2024-01-01')",
            params![id, name],
        )
        .unwrap();
    }

    fn insert_test_attribute(conn: &Connection, id: i64, name: &str) {
        conn.execute(
            "INSERT INTO attributes (id, name, description, base_value, created_at, updated_at)
             VALUES (?, ?, 'Test attribute', 4, '2024-01-01', '2024-01-01')",
            params![id, name],
        )
        .unwrap();
    }

    fn insert_character_attribute(
        conn: &Connection,
        character_id: i64,
        attribute_id: i64,
        steps: i64,
    ) {
        conn.execute(
            "INSERT INTO character_attributes (character_id, attribute_id, steps_incremented,
                                               created_at, updated_at)
             VALUES (?, ?, ?, '2024-01-01', '2024-01-01')",
            params![character_id, attribute_id, steps],
        )
        .unwrap();
    }

    #[test]
    fn get_by_id_returns_none_when_not_found() {
        let conn = setup_test_db();

        let result = CharacterService::get_by_id(&conn, 999).unwrap();

        assert!(result.is_none());
    }

    #[test]
    fn get_by_id_returns_character_with_basic_info() {
        let conn = setup_test_db();
        insert_test_rank(&conn, 1, "Novice");
        insert_test_character(&conn, 1, "Test Hero");

        let character = CharacterService::get_by_id(&conn, 1).unwrap().unwrap();

        assert_eq!(character.id, 1);
        assert_eq!(character.name, "Test Hero");
        assert!(character.is_wild_card);
        assert_eq!(character.rank.name, "Novice");
        assert!(character.ancestry.is_none());
    }

    #[test]
    fn get_by_id_includes_attributes() {
        let conn = setup_test_db();
        insert_test_rank(&conn, 1, "Novice");
        insert_test_character(&conn, 1, "Test Hero");
        insert_test_attribute(&conn, 1, "Agility");
        insert_character_attribute(&conn, 1, 1, 2); // d4 base + 2 steps = d8

        let character = CharacterService::get_by_id(&conn, 1).unwrap().unwrap();

        assert_eq!(character.attributes.len(), 1);
        assert_eq!(character.attributes[0].attribute.name, "Agility");
        assert_eq!(character.attributes[0].die, Die::d8());
    }

    #[test]
    fn get_all_returns_empty_when_no_characters() {
        let conn = setup_test_db();

        let characters = CharacterService::get_all(&conn).unwrap();

        assert_eq!(characters.len(), 0);
    }

    #[test]
    fn get_all_returns_all_characters() {
        let conn = setup_test_db();
        insert_test_rank(&conn, 1, "Novice");
        insert_test_character(&conn, 1, "Hero One");
        insert_test_character(&conn, 2, "Hero Two");

        let characters = CharacterService::get_all(&conn).unwrap();

        assert_eq!(characters.len(), 2);
    }
}
