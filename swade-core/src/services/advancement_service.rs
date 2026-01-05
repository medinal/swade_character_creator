//! Advancement service for managing character progression.
//!
//! This service handles all advancement-related operations including:
//! - Calculating available advancement options
//! - Applying advances (edges, attributes, skills, hindrances)
//! - Validating advancement rules
//! - Undo functionality

use rusqlite::Connection;

use crate::error::{Result, SwadeError};
use crate::models::CharacterAdvance;
use crate::repositories::{
    AttributeRepository, CharacterAdvanceRepository, CharacterAttributeRepository,
    CharacterEdgeRepository, CharacterHindranceRepository, CharacterSkillRepository,
    HindranceRepository, RankRepository, SkillRepository,
};
use crate::services::{CharacterService, EdgeService};
use crate::views::{
    AdvanceType, AdvancementOptions, AttributeAdvanceOption, CharacterAdvanceValue,
    HindranceAction, HindranceAdvanceOption, SkillAdvanceOption,
};

pub struct AdvancementService;

impl AdvancementService {
    /// Get the available advancement options for a character's next advance.
    pub fn get_advancement_options(conn: &Connection, character_id: i64) -> Result<AdvancementOptions> {
        let current_advances = CharacterAdvanceRepository::count_by_character_id(conn, character_id)?;
        let next_advance_number = current_advances + 1;

        let current_rank = Self::get_rank_for_advances(conn, current_advances)?;
        let next_rank = Self::get_rank_for_advances(conn, next_advance_number)?;

        // Load character for attribute/skill/hindrance data
        let character = CharacterService::get_by_id(conn, character_id)?
            .ok_or_else(|| SwadeError::NotFound(format!("Character with id {}", character_id)))?;

        // Check if attribute advance is available
        let (can_increase_attribute, attribute_blocked_reason) =
            Self::check_attribute_advance_available(conn, character_id, current_advances, &current_rank)?;

        // Build attribute options
        let attribute_options: Vec<AttributeAdvanceOption> = character
            .attributes
            .iter()
            .map(|attr| {
                let current_die = attr.die.size();
                let effective_die = attr.effective_die.size();
                let is_maxed = current_die >= 12;
                let next_die = if is_maxed { current_die } else { current_die + 2 };
                // Effective next = apply same modifier bonus to next_die
                let modifier_bonus = effective_die.saturating_sub(current_die);
                let effective_next_die = if is_maxed { effective_die } else { next_die + modifier_bonus };
                AttributeAdvanceOption {
                    id: attr.attribute.id,
                    name: attr.attribute.name.clone(),
                    current_die,
                    effective_die,
                    next_die,
                    effective_next_die,
                    is_maxed,
                }
            })
            .collect();

        // Build skill options - need to classify as expensive or cheap
        let mut expensive_skill_options = Vec::new();
        let mut cheap_skill_options = Vec::new();

        for skill_value in &character.skills {
            let skill_die = skill_value.die.map(|d| d.size()).unwrap_or(0);
            let effective_skill_die = skill_value.effective_die.map(|d| d.size()).unwrap_or(0);
            let linked_attr = character
                .attributes
                .iter()
                .find(|a| a.attribute.id == skill_value.skill.linked_attribute_id);
            let attr_die = linked_attr.map(|a| a.die.size()).unwrap_or(4);

            let is_maxed = skill_die >= 12;
            let next_die = if skill_die == 0 { 4 } else { skill_die + 2 };
            let next_die = if is_maxed { skill_die } else { next_die };
            // Effective next = apply same modifier bonus to next_die
            let modifier_bonus = effective_skill_die.saturating_sub(skill_die);
            let effective_next_die = if is_maxed { effective_skill_die } else { next_die + modifier_bonus };

            let option = SkillAdvanceOption {
                id: skill_value.skill.id,
                name: skill_value.skill.name.clone(),
                current_die: skill_die,
                effective_die: effective_skill_die,
                next_die,
                effective_next_die,
                is_maxed,
            };

            if skill_die == 0 {
                // Untrained skills are cheap (going to d4)
                cheap_skill_options.push(option);
            } else if skill_die >= attr_die {
                // At or above linked attribute = expensive
                expensive_skill_options.push(option);
            } else {
                // Below linked attribute = cheap
                cheap_skill_options.push(option);
            }
        }

        // Check for banked hindrance removals
        let banked_hindrance_ids = Self::get_all_banked_hindrance_ids(conn, character_id)?;

        // Build hindrance options (any hindrance can be removed via advancement)
        let hindrance_options: Vec<HindranceAdvanceOption> = character
            .hindrances
            .iter()
            .map(|h| {
                let is_banked = banked_hindrance_ids.contains(&h.hindrance.id);
                let has_companion = h.hindrance.companion_hindrance_id.is_some();
                let is_major = matches!(h.hindrance.severity, crate::views::Severity::Major);

                let action = if is_banked {
                    HindranceAction::CompleteMajorRemoval
                } else if !is_major {
                    HindranceAction::RemoveMinor
                } else if has_companion {
                    HindranceAction::ReduceMajor
                } else {
                    HindranceAction::RemoveMajorHalf
                };

                HindranceAdvanceOption {
                    id: h.hindrance.id,
                    name: h.hindrance.name.clone(),
                    description: h.hindrance.description.clone(),
                    severity: h.hindrance.severity.to_string(),
                    action_label: action.label().to_string(),
                    action,
                    is_banked,
                }
            })
            .collect();

        let can_modify_hindrance = !hindrance_options.is_empty();

        Ok(AdvancementOptions {
            can_take_edge: true,
            can_increase_attribute,
            attribute_blocked_reason,
            attribute_options,
            can_increase_expensive_skill: !expensive_skill_options.is_empty(),
            expensive_skill_options,
            can_increase_cheap_skills: !cheap_skill_options.is_empty(),
            cheap_skill_options,
            can_modify_hindrance,
            hindrance_options,
            next_advance_number,
            current_rank: current_rank.name,
            rank_after_advance: next_rank.name,
        })
    }

    /// Apply an edge advancement.
    pub fn apply_edge_advance(
        conn: &Connection,
        character_id: i64,
        edge_id: i64,
        notes: Option<String>,
    ) -> Result<CharacterAdvanceValue> {
        let current_advances = CharacterAdvanceRepository::count_by_character_id(conn, character_id)?;
        let advance_number = current_advances + 1;

        // Load the character for requirement checking
        let character = CharacterService::get_by_id(conn, character_id)?
            .ok_or_else(|| SwadeError::NotFound(format!("Character with id {}", character_id)))?;

        // Load the edge with requirements
        let edge = EdgeService::get_by_id(conn, edge_id)?
            .ok_or_else(|| SwadeError::NotFound(format!("Edge with id {}", edge_id)))?;

        // Validate edge requirements
        if !edge.requirements.is_empty() {
            let req_context = character.to_requirement_context();
            if !edge.requirements.evaluate(&req_context) {
                // Get detailed status of which requirements failed
                let statuses = edge.requirements.evaluate_detailed(&req_context);
                let unmet: Vec<String> = statuses
                    .iter()
                    .filter(|s| !s.is_met)
                    .map(|s| s.description.clone())
                    .collect();

                return Err(SwadeError::Validation(format!(
                    "Requirements not met for {}: {}",
                    edge.name,
                    unmet.join(", ")
                )));
            }
        }

        // Create the advance record
        let advance = CharacterAdvance {
            id: 0,
            character_id,
            advance_number,
            advance_type: "edge".to_string(),
            edge_id: Some(edge_id),
            attribute_id: None,
            skill_id_1: None,
            skill_id_2: None,
            hindrance_id: None,
            hindrance_action: None,
            notes: notes.clone(),
            created_at: String::new(),
            updated_at: String::new(),
        };

        let advance_id = CharacterAdvanceRepository::insert(conn, &advance)?;

        // Create the character_edges record
        let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let char_edge = crate::models::CharacterEdge {
            id: 0,
            character_id,
            edge_id,
            advance_taken: advance_number,
            notes,
            source: "advancement".to_string(),
            created_at: now.clone(),
            updated_at: now,
        };
        CharacterEdgeRepository::insert(conn, &char_edge)?;

        Ok(CharacterAdvanceValue {
            id: advance_id,
            advance_number,
            advance_type: AdvanceType::Edge,
            description: format!("Gained edge: {}", edge.name),
            created_at: chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        })
    }

    /// Apply an attribute advancement.
    pub fn apply_attribute_advance(
        conn: &Connection,
        character_id: i64,
        attribute_id: i64,
    ) -> Result<CharacterAdvanceValue> {
        let current_advances = CharacterAdvanceRepository::count_by_character_id(conn, character_id)?;
        let advance_number = current_advances + 1;
        let current_rank = Self::get_rank_for_advances(conn, current_advances)?;

        // Check if attribute advance is allowed
        let (can_advance, blocked_reason) =
            Self::check_attribute_advance_available(conn, character_id, current_advances, &current_rank)?;

        if !can_advance {
            return Err(SwadeError::Validation(
                blocked_reason.unwrap_or_else(|| "Cannot take attribute advance".to_string()),
            ));
        }

        // Verify the attribute exists
        let attribute = AttributeRepository::get_by_id(conn, attribute_id)?
            .ok_or_else(|| SwadeError::NotFound(format!("Attribute with id {}", attribute_id)))?;

        // Get current attribute value
        let char_attrs = CharacterAttributeRepository::get_by_character_id(conn, character_id)?;
        let char_attr = char_attrs
            .iter()
            .find(|a| a.attribute_id == attribute_id)
            .ok_or_else(|| SwadeError::NotFound(format!("Character attribute {}", attribute_id)))?;

        // Check if attribute is at max (d12)
        let current_steps = char_attr.steps_incremented;
        if current_steps >= 4 {
            // d4 + 4 steps = d12
            return Err(SwadeError::Validation(format!(
                "{} is already at maximum (d12)",
                attribute.name
            )));
        }

        // Create the advance record
        let advance = CharacterAdvance {
            id: 0,
            character_id,
            advance_number,
            advance_type: "attribute".to_string(),
            edge_id: None,
            attribute_id: Some(attribute_id),
            skill_id_1: None,
            skill_id_2: None,
            hindrance_id: None,
            hindrance_action: None,
            notes: None,
            created_at: String::new(),
            updated_at: String::new(),
        };

        let advance_id = CharacterAdvanceRepository::insert(conn, &advance)?;

        // Update the character_attributes record
        let mut updated_attr = char_attr.clone();
        updated_attr.steps_incremented += 1;
        updated_attr.updated_at = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
        CharacterAttributeRepository::update(conn, &updated_attr)?;

        Ok(CharacterAdvanceValue {
            id: advance_id,
            advance_number,
            advance_type: AdvanceType::Attribute,
            description: format!("Increased {}", attribute.name),
            created_at: chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        })
    }

    /// Apply an expensive skill advancement (one skill at or above linked attribute).
    pub fn apply_expensive_skill_advance(
        conn: &Connection,
        character_id: i64,
        skill_id: i64,
    ) -> Result<CharacterAdvanceValue> {
        let current_advances = CharacterAdvanceRepository::count_by_character_id(conn, character_id)?;
        let advance_number = current_advances + 1;

        // Verify the skill exists
        let skill = SkillRepository::get_by_id(conn, skill_id)?
            .ok_or_else(|| SwadeError::NotFound(format!("Skill with id {}", skill_id)))?;

        // Get current skill and attribute values
        let char_skills = CharacterSkillRepository::get_by_character_id(conn, character_id)?;
        let char_skill = char_skills
            .iter()
            .find(|s| s.skill_id == skill_id)
            .ok_or_else(|| SwadeError::NotFound(format!("Character skill {}", skill_id)))?;

        let char_attrs = CharacterAttributeRepository::get_by_character_id(conn, character_id)?;
        let linked_attr = char_attrs
            .iter()
            .find(|a| a.attribute_id == skill.linked_attribute_id)
            .ok_or_else(|| {
                SwadeError::NotFound(format!("Linked attribute {}", skill.linked_attribute_id))
            })?;

        // Calculate die sizes
        let skill_die_size = char_skill.current_die_size.unwrap_or(0);
        let attr_die_size = 4 + (linked_attr.steps_incremented * 2); // d4=4, d6=6, etc.

        // Verify skill is at or above linked attribute (expensive)
        if skill_die_size < attr_die_size {
            return Err(SwadeError::Validation(format!(
                "{} (d{}) is below linked attribute (d{}). Use cheap skill advance instead.",
                skill.name, skill_die_size, attr_die_size
            )));
        }

        // Check if skill is at max
        if skill_die_size >= 12 {
            return Err(SwadeError::Validation(format!(
                "{} is already at maximum (d12)",
                skill.name
            )));
        }

        // Create the advance record
        let advance = CharacterAdvance {
            id: 0,
            character_id,
            advance_number,
            advance_type: "skill_expensive".to_string(),
            edge_id: None,
            attribute_id: None,
            skill_id_1: Some(skill_id),
            skill_id_2: None,
            hindrance_id: None,
            hindrance_action: None,
            notes: None,
            created_at: String::new(),
            updated_at: String::new(),
        };

        let advance_id = CharacterAdvanceRepository::insert(conn, &advance)?;

        // Update the character_skills record
        let mut updated_skill = char_skill.clone();
        let new_die_size = skill_die_size + 2; // Increment one die step
        updated_skill.current_die_size = Some(new_die_size);
        updated_skill.updated_at = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
        CharacterSkillRepository::update(conn, &updated_skill)?;

        Ok(CharacterAdvanceValue {
            id: advance_id,
            advance_number,
            advance_type: AdvanceType::SkillExpensive,
            description: format!("Increased {} to d{}", skill.name, new_die_size),
            created_at: chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        })
    }

    /// Apply a cheap skill advancement (two skills below linked attribute).
    pub fn apply_cheap_skill_advance(
        conn: &Connection,
        character_id: i64,
        skill_id_1: i64,
        skill_id_2: i64,
    ) -> Result<CharacterAdvanceValue> {
        let current_advances = CharacterAdvanceRepository::count_by_character_id(conn, character_id)?;
        let advance_number = current_advances + 1;

        // Validate both skills
        let (skill1, new_die_1) = Self::validate_cheap_skill(conn, character_id, skill_id_1)?;
        let (skill2, new_die_2) = Self::validate_cheap_skill(conn, character_id, skill_id_2)?;

        // Create the advance record
        let advance = CharacterAdvance {
            id: 0,
            character_id,
            advance_number,
            advance_type: "skill_cheap".to_string(),
            edge_id: None,
            attribute_id: None,
            skill_id_1: Some(skill_id_1),
            skill_id_2: Some(skill_id_2),
            hindrance_id: None,
            hindrance_action: None,
            notes: None,
            created_at: String::new(),
            updated_at: String::new(),
        };

        let advance_id = CharacterAdvanceRepository::insert(conn, &advance)?;

        // Update both skills
        Self::apply_skill_increase(conn, character_id, skill_id_1)?;
        Self::apply_skill_increase(conn, character_id, skill_id_2)?;

        Ok(CharacterAdvanceValue {
            id: advance_id,
            advance_number,
            advance_type: AdvanceType::SkillCheap,
            description: format!(
                "Increased {} to d{} and {} to d{}",
                skill1.name, new_die_1, skill2.name, new_die_2
            ),
            created_at: chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        })
    }

    /// Apply a hindrance advancement (remove minor, reduce major, or bank toward major removal).
    pub fn apply_hindrance_advance(
        conn: &Connection,
        character_id: i64,
        hindrance_id: i64,
        action: &str,
    ) -> Result<CharacterAdvanceValue> {
        let current_advances = CharacterAdvanceRepository::count_by_character_id(conn, character_id)?;
        let advance_number = current_advances + 1;

        // Verify the hindrance exists on the character
        let char_hindrances = CharacterHindranceRepository::get_by_character_id(conn, character_id)?;
        let char_hindrance = char_hindrances
            .iter()
            .find(|h| h.hindrance_id == hindrance_id);

        // Store the original source for undo purposes
        let original_source = char_hindrance.map(|h| h.source.clone());

        let hindrance = HindranceRepository::get_by_id(conn, hindrance_id)?
            .ok_or_else(|| SwadeError::NotFound(format!("Hindrance with id {}", hindrance_id)))?;

        let description = match action {
            "remove_minor" => {
                // Verify it's a minor hindrance
                if hindrance.severity.to_lowercase() != "minor" {
                    return Err(SwadeError::Validation(format!(
                        "{} is not a Minor hindrance",
                        hindrance.name
                    )));
                }
                let char_h = char_hindrance.ok_or_else(|| {
                    SwadeError::Validation(format!("Character doesn't have {}", hindrance.name))
                })?;

                // Delete the hindrance from character
                CharacterHindranceRepository::delete(conn, char_h.id)?;
                format!("Removed minor hindrance: {}", hindrance.name)
            }
            "reduce_major" => {
                // Verify it's a major hindrance with a minor companion
                if hindrance.severity.to_lowercase() != "major" {
                    return Err(SwadeError::Validation(format!(
                        "{} is not a Major hindrance",
                        hindrance.name
                    )));
                }
                let companion_id = hindrance.companion_hindrance_id.ok_or_else(|| {
                    SwadeError::Validation(format!(
                        "{} cannot be reduced (no minor version exists)",
                        hindrance.name
                    ))
                })?;

                let char_h = char_hindrance.ok_or_else(|| {
                    SwadeError::Validation(format!("Character doesn't have {}", hindrance.name))
                })?;

                // Delete the major hindrance
                CharacterHindranceRepository::delete(conn, char_h.id)?;

                // Add the minor version
                let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
                let minor_hindrance = crate::models::CharacterHindrance {
                    id: 0,
                    character_id,
                    hindrance_id: companion_id,
                    source: "advancement_reduced".to_string(),
                    created_at: now.clone(),
                    updated_at: now,
                };
                CharacterHindranceRepository::insert(conn, &minor_hindrance)?;

                let minor = HindranceRepository::get_by_id(conn, companion_id)?;
                format!(
                    "Reduced {} to {}",
                    hindrance.name,
                    minor.map(|h| h.name).unwrap_or_else(|| "minor version".to_string())
                )
            }
            "remove_major_half" | "complete_major_removal" => {
                // Check if this is completing a banked removal or starting a new one
                let banked = CharacterAdvanceRepository::get_banked_hindrance_removal(
                    conn,
                    character_id,
                    hindrance_id,
                )?;

                if banked.is_some() {
                    // This is the second half - complete the removal
                    let char_h = char_hindrance.ok_or_else(|| {
                        SwadeError::Validation(format!("Character doesn't have {}", hindrance.name))
                    })?;
                    CharacterHindranceRepository::delete(conn, char_h.id)?;
                    format!("Removed major hindrance: {} (2nd advance)", hindrance.name)
                } else {
                    // This is the first half - just bank it
                    // Verify the character has this hindrance
                    char_hindrance.ok_or_else(|| {
                        SwadeError::Validation(format!("Character doesn't have {}", hindrance.name))
                    })?;
                    format!(
                        "Banked advance toward removing: {} (1 of 2)",
                        hindrance.name
                    )
                }
            }
            _ => {
                return Err(SwadeError::Validation(format!(
                    "Invalid hindrance action: {}",
                    action
                )));
            }
        };

        // Create the advance record (store original source in notes for undo)
        let advance = CharacterAdvance {
            id: 0,
            character_id,
            advance_number,
            advance_type: "hindrance".to_string(),
            edge_id: None,
            attribute_id: None,
            skill_id_1: None,
            skill_id_2: None,
            hindrance_id: Some(hindrance_id),
            hindrance_action: Some(action.to_string()),
            notes: original_source,
            created_at: String::new(),
            updated_at: String::new(),
        };

        let advance_id = CharacterAdvanceRepository::insert(conn, &advance)?;

        Ok(CharacterAdvanceValue {
            id: advance_id,
            advance_number,
            advance_type: AdvanceType::Hindrance,
            description,
            created_at: chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        })
    }

    /// Undo the most recent advance for a character.
    pub fn undo_advance(conn: &Connection, character_id: i64) -> Result<bool> {
        let advances = CharacterAdvanceRepository::get_by_character_id(conn, character_id)?;

        let latest = advances.last().ok_or_else(|| {
            SwadeError::Validation("No advances to undo".to_string())
        })?;

        // Revert the changes based on advance type
        match latest.advance_type.as_str() {
            "edge" => {
                if let Some(edge_id) = latest.edge_id {
                    // Find and delete the character_edges record for this advance
                    let char_edges = CharacterEdgeRepository::get_by_character_id(conn, character_id)?;
                    if let Some(char_edge) = char_edges
                        .iter()
                        .find(|e| e.edge_id == edge_id && e.advance_taken == latest.advance_number)
                    {
                        CharacterEdgeRepository::delete(conn, char_edge.id)?;
                    }
                }
            }
            "attribute" => {
                if let Some(attribute_id) = latest.attribute_id {
                    // Decrement the attribute
                    let char_attrs = CharacterAttributeRepository::get_by_character_id(conn, character_id)?;
                    if let Some(char_attr) = char_attrs.iter().find(|a| a.attribute_id == attribute_id) {
                        let mut updated = char_attr.clone();
                        updated.steps_incremented = (updated.steps_incremented - 1).max(0);
                        updated.updated_at = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
                        CharacterAttributeRepository::update(conn, &updated)?;
                    }
                }
            }
            "skill_expensive" => {
                if let Some(skill_id) = latest.skill_id_1 {
                    Self::revert_skill_increase(conn, character_id, skill_id)?;
                }
            }
            "skill_cheap" => {
                if let Some(skill_id_1) = latest.skill_id_1 {
                    Self::revert_skill_increase(conn, character_id, skill_id_1)?;
                }
                if let Some(skill_id_2) = latest.skill_id_2 {
                    Self::revert_skill_increase(conn, character_id, skill_id_2)?;
                }
            }
            "hindrance" => {
                if let (Some(hindrance_id), Some(action)) = (latest.hindrance_id, &latest.hindrance_action) {
                    // The original source is stored in notes field
                    let original_source = latest.notes.clone().unwrap_or_else(|| "chosen".to_string());

                    match action.as_str() {
                        "remove_minor" => {
                            // Re-add the minor hindrance with its original source
                            let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
                            let char_hindrance = crate::models::CharacterHindrance {
                                id: 0,
                                character_id,
                                hindrance_id,
                                source: original_source,
                                created_at: now.clone(),
                                updated_at: now,
                            };
                            CharacterHindranceRepository::insert(conn, &char_hindrance)?;
                        }
                        "reduce_major" => {
                            // Get the hindrance to find its companion
                            if let Some(hindrance) = HindranceRepository::get_by_id(conn, hindrance_id)?
                                && let Some(companion_id) = hindrance.companion_hindrance_id
                            {
                                // Delete the minor companion from character
                                let char_hindrances = CharacterHindranceRepository::get_by_character_id(conn, character_id)?;
                                if let Some(minor_h) = char_hindrances.iter().find(|h| h.hindrance_id == companion_id) {
                                    CharacterHindranceRepository::delete(conn, minor_h.id)?;
                                }
                                // Re-add the major hindrance with its original source
                                let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
                                let char_hindrance = crate::models::CharacterHindrance {
                                    id: 0,
                                    character_id,
                                    hindrance_id,
                                    source: original_source,
                                    created_at: now.clone(),
                                    updated_at: now,
                                };
                                CharacterHindranceRepository::insert(conn, &char_hindrance)?;
                            }
                        }
                        "remove_major_half" => {
                            // Check if this was the completing advance (hindrance was deleted)
                            let char_hindrances = CharacterHindranceRepository::get_by_character_id(conn, character_id)?;
                            let has_hindrance = char_hindrances.iter().any(|h| h.hindrance_id == hindrance_id);

                            if !has_hindrance {
                                // Hindrance was deleted, re-add it with its original source
                                let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
                                let char_hindrance = crate::models::CharacterHindrance {
                                    id: 0,
                                    character_id,
                                    hindrance_id,
                                    source: original_source,
                                    created_at: now.clone(),
                                    updated_at: now,
                                };
                                CharacterHindranceRepository::insert(conn, &char_hindrance)?;
                            }
                            // If hindrance exists, this was just the first half - just delete the advance record
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }

        // Delete the advance record
        CharacterAdvanceRepository::delete(conn, latest.id)?;

        Ok(true)
    }

    /// Get the advancement history for a character.
    pub fn get_advancement_history(
        conn: &Connection,
        character_id: i64,
    ) -> Result<Vec<CharacterAdvanceValue>> {
        let advances = CharacterAdvanceRepository::get_by_character_id(conn, character_id)?;
        let mut result = Vec::new();

        for advance in advances {
            let description = Self::build_advance_description(conn, &advance)?;
            result.push(CharacterAdvanceValue {
                id: advance.id,
                advance_number: advance.advance_number,
                advance_type: advance.advance_type.parse().unwrap_or(AdvanceType::Edge),
                description,
                created_at: advance.created_at,
            });
        }

        Ok(result)
    }

    // ========== Helper Functions ==========

    fn get_rank_for_advances(conn: &Connection, advances: i64) -> Result<crate::models::Rank> {
        let ranks = RankRepository::get_all(conn)?;

        for rank in ranks.iter().rev() {
            if advances >= rank.min_advances
                && (rank.max_advances.is_none() || advances <= rank.max_advances.unwrap())
            {
                return Ok(rank.clone());
            }
        }

        ranks
            .into_iter()
            .next()
            .ok_or_else(|| SwadeError::NotFound("No ranks found".to_string()))
    }

    fn check_attribute_advance_available(
        conn: &Connection,
        character_id: i64,
        current_advances: i64,
        current_rank: &crate::models::Rank,
    ) -> Result<(bool, Option<String>)> {
        // Get the rank boundaries
        let min_advance_in_rank = current_rank.min_advances;
        let max_advance_in_rank = current_rank.max_advances.unwrap_or(i64::MAX);

        if current_rank.name == "Legendary" {
            // Legendary: can take attribute every other advance after becoming Legendary
            let advances_since_legendary = current_advances - 16 + 1; // +1 for the advance about to be taken
            let attr_advances_since_legendary = CharacterAdvanceRepository::count_attribute_advances_in_range(
                conn,
                character_id,
                16, // Legendary starts at advance 16
                current_advances,
            )?;

            // Can take attribute if: (advances_since_legendary / 2) > attr_advances_since_legendary
            if advances_since_legendary / 2 > attr_advances_since_legendary {
                Ok((true, None))
            } else {
                Ok((
                    false,
                    Some("Can only increase an attribute every other Legendary advance".to_string()),
                ))
            }
        } else {
            // Other ranks: once per rank
            let attr_advances_in_rank = CharacterAdvanceRepository::count_attribute_advances_in_range(
                conn,
                character_id,
                min_advance_in_rank,
                max_advance_in_rank,
            )?;

            if attr_advances_in_rank == 0 {
                Ok((true, None))
            } else {
                Ok((
                    false,
                    Some(format!(
                        "Already increased an attribute this rank ({})",
                        current_rank.name
                    )),
                ))
            }
        }
    }

    fn get_all_banked_hindrance_ids(conn: &Connection, character_id: i64) -> Result<Vec<i64>> {
        let advances = CharacterAdvanceRepository::get_by_character_id(conn, character_id)?;
        let mut banked = Vec::new();

        // Count remove_major_half for each hindrance
        let mut counts: std::collections::HashMap<i64, i64> = std::collections::HashMap::new();
        for advance in &advances {
            if advance.advance_type == "hindrance"
                && advance.hindrance_action.as_deref() == Some("remove_major_half")
                && let Some(h_id) = advance.hindrance_id
            {
                *counts.entry(h_id).or_insert(0) += 1;
            }
        }

        // Banked = odd count (1, 3, 5, etc.)
        for (hindrance_id, count) in counts {
            if count % 2 == 1 {
                banked.push(hindrance_id);
            }
        }

        Ok(banked)
    }

    fn validate_cheap_skill(
        conn: &Connection,
        character_id: i64,
        skill_id: i64,
    ) -> Result<(crate::models::Skill, i64)> {
        let skill = SkillRepository::get_by_id(conn, skill_id)?
            .ok_or_else(|| SwadeError::NotFound(format!("Skill with id {}", skill_id)))?;

        let char_skills = CharacterSkillRepository::get_by_character_id(conn, character_id)?;
        let char_skill = char_skills.iter().find(|s| s.skill_id == skill_id);

        let char_attrs = CharacterAttributeRepository::get_by_character_id(conn, character_id)?;
        let linked_attr = char_attrs
            .iter()
            .find(|a| a.attribute_id == skill.linked_attribute_id)
            .ok_or_else(|| {
                SwadeError::NotFound(format!("Linked attribute {}", skill.linked_attribute_id))
            })?;

        let skill_die_size = char_skill.and_then(|s| s.current_die_size).unwrap_or(0);
        let attr_die_size = 4 + (linked_attr.steps_incremented * 2);

        // For cheap skills, the skill must be below linked attribute
        // OR be a new skill (die size 0 -> d4)
        if skill_die_size == 0 {
            // New skill, going to d4
            Ok((skill, 4))
        } else if skill_die_size < attr_die_size {
            // Below attribute, can increase
            if skill_die_size >= 12 {
                return Err(SwadeError::Validation(format!(
                    "{} is already at maximum (d12)",
                    skill.name
                )));
            }
            Ok((skill, skill_die_size + 2))
        } else {
            Err(SwadeError::Validation(format!(
                "{} (d{}) is at or above linked attribute (d{}). Use expensive skill advance.",
                skill.name, skill_die_size, attr_die_size
            )))
        }
    }

    fn apply_skill_increase(conn: &Connection, character_id: i64, skill_id: i64) -> Result<()> {
        let char_skills = CharacterSkillRepository::get_by_character_id(conn, character_id)?;

        if let Some(char_skill) = char_skills.iter().find(|s| s.skill_id == skill_id) {
            let current_die = char_skill.current_die_size.unwrap_or(0);
            let new_die = if current_die == 0 { 4 } else { current_die + 2 };

            let mut updated = char_skill.clone();
            updated.current_die_size = Some(new_die);
            updated.updated_at = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
            CharacterSkillRepository::update(conn, &updated)?;
        }

        Ok(())
    }

    fn revert_skill_increase(conn: &Connection, character_id: i64, skill_id: i64) -> Result<()> {
        let char_skills = CharacterSkillRepository::get_by_character_id(conn, character_id)?;

        if let Some(char_skill) = char_skills.iter().find(|s| s.skill_id == skill_id)
            && let Some(current_die) = char_skill.current_die_size
        {
            let new_die = if current_die <= 4 {
                None // Revert to untrained
            } else {
                Some(current_die - 2)
            };

            let mut updated = char_skill.clone();
            updated.current_die_size = new_die;
            updated.updated_at = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
            CharacterSkillRepository::update(conn, &updated)?;
        }

        Ok(())
    }

    fn build_advance_description(conn: &Connection, advance: &CharacterAdvance) -> Result<String> {
        let description = match advance.advance_type.as_str() {
            "edge" => {
                if let Some(edge_id) = advance.edge_id {
                    let edge = EdgeService::get_by_id(conn, edge_id)?;
                    format!(
                        "Gained edge: {}",
                        edge.map(|e| e.name).unwrap_or_else(|| "Unknown".to_string())
                    )
                } else {
                    "Gained edge".to_string()
                }
            }
            "attribute" => {
                if let Some(attr_id) = advance.attribute_id {
                    let attr = AttributeRepository::get_by_id(conn, attr_id)?;
                    format!(
                        "Increased {}",
                        attr.map(|a| a.name).unwrap_or_else(|| "attribute".to_string())
                    )
                } else {
                    "Increased attribute".to_string()
                }
            }
            "skill_expensive" => {
                if let Some(skill_id) = advance.skill_id_1 {
                    let skill = SkillRepository::get_by_id(conn, skill_id)?;
                    format!(
                        "Increased {} (expensive)",
                        skill.map(|s| s.name).unwrap_or_else(|| "skill".to_string())
                    )
                } else {
                    "Increased skill (expensive)".to_string()
                }
            }
            "skill_cheap" => {
                let skill1_name = advance
                    .skill_id_1
                    .and_then(|id| SkillRepository::get_by_id(conn, id).ok().flatten())
                    .map(|s| s.name)
                    .unwrap_or_else(|| "skill 1".to_string());
                let skill2_name = advance
                    .skill_id_2
                    .and_then(|id| SkillRepository::get_by_id(conn, id).ok().flatten())
                    .map(|s| s.name)
                    .unwrap_or_else(|| "skill 2".to_string());
                format!("Increased {} and {}", skill1_name, skill2_name)
            }
            "hindrance" => {
                let hindrance_name = advance
                    .hindrance_id
                    .and_then(|id| HindranceRepository::get_by_id(conn, id).ok().flatten())
                    .map(|h| h.name)
                    .unwrap_or_else(|| "hindrance".to_string());

                match advance.hindrance_action.as_deref() {
                    Some("remove_minor") => format!("Removed minor: {}", hindrance_name),
                    Some("reduce_major") => format!("Reduced major: {}", hindrance_name),
                    Some("remove_major_half") => {
                        format!("Progress toward removing: {}", hindrance_name)
                    }
                    _ => format!("Modified: {}", hindrance_name),
                }
            }
            _ => "Unknown advance".to_string(),
        };

        Ok(description)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::{setup_test_db, insert_test_rank, insert_test_character, insert_test_attribute, insert_test_edge};
    use rusqlite::params;

    fn insert_character_attribute(conn: &Connection, character_id: i64, attribute_id: i64, steps: i64) {
        conn.execute(
            "INSERT INTO character_attributes (character_id, attribute_id, steps_incremented,
                                               created_at, updated_at)
             VALUES (?, ?, ?, '2024-01-01', '2024-01-01')",
            params![character_id, attribute_id, steps],
        )
        .unwrap();
    }

    #[test]
    fn test_get_advancement_options_novice() {
        let conn = setup_test_db();
        insert_test_rank(&conn, 1, "Novice");
        insert_test_character(&conn, 1, "Test Hero");

        let options = AdvancementService::get_advancement_options(&conn, 1).unwrap();

        assert!(options.can_take_edge);
        assert!(options.can_increase_attribute);
        // With no skills on character, no skill advances available
        assert!(!options.can_increase_expensive_skill);
        // Cheap skills include untrained skills, so there may be some available
        // depending on what skills exist in the test DB
        assert_eq!(options.next_advance_number, 1);
        assert_eq!(options.current_rank, "Novice");
    }

    #[test]
    fn test_attribute_advance_blocked_after_one_per_rank() {
        let conn = setup_test_db();
        insert_test_rank(&conn, 1, "Novice");
        insert_test_character(&conn, 1, "Test Hero");
        insert_test_attribute(&conn, 1, "Agility");
        insert_character_attribute(&conn, 1, 1, 0);

        // Take an attribute advance
        let _ = AdvancementService::apply_attribute_advance(&conn, 1, 1).unwrap();

        // Check options - attribute should now be blocked
        let options = AdvancementService::get_advancement_options(&conn, 1).unwrap();
        assert!(!options.can_increase_attribute);
        assert!(options.attribute_blocked_reason.is_some());
    }

    #[test]
    fn test_apply_edge_advance() {
        let conn = setup_test_db();
        insert_test_rank(&conn, 1, "Novice");
        insert_test_character(&conn, 1, "Test Hero");
        insert_test_edge(&conn, 1, "Alertness", "Background");

        let result = AdvancementService::apply_edge_advance(&conn, 1, 1, None).unwrap();

        assert_eq!(result.advance_number, 1);
        assert_eq!(result.advance_type, AdvanceType::Edge);
        assert!(result.description.contains("Alertness"));

        // Verify edge was added to character
        let char_edges = CharacterEdgeRepository::get_by_character_id(&conn, 1).unwrap();
        assert_eq!(char_edges.len(), 1);
        assert_eq!(char_edges[0].edge_id, 1);
    }

    #[test]
    fn test_get_advancement_history() {
        let conn = setup_test_db();
        insert_test_rank(&conn, 1, "Novice");
        insert_test_character(&conn, 1, "Test Hero");
        insert_test_edge(&conn, 1, "Alertness", "Background");
        insert_test_edge(&conn, 2, "Brawny", "Background");

        // Take two advances
        AdvancementService::apply_edge_advance(&conn, 1, 1, None).unwrap();
        AdvancementService::apply_edge_advance(&conn, 1, 2, None).unwrap();

        let history = AdvancementService::get_advancement_history(&conn, 1).unwrap();

        assert_eq!(history.len(), 2);
        assert_eq!(history[0].advance_number, 1);
        assert_eq!(history[1].advance_number, 2);
    }

    #[test]
    fn test_undo_edge_advance() {
        let conn = setup_test_db();
        insert_test_rank(&conn, 1, "Novice");
        insert_test_character(&conn, 1, "Test Hero");
        insert_test_edge(&conn, 1, "Alertness", "Background");

        // Take an advance
        AdvancementService::apply_edge_advance(&conn, 1, 1, None).unwrap();

        // Verify edge exists
        let edges_before = CharacterEdgeRepository::get_by_character_id(&conn, 1).unwrap();
        assert_eq!(edges_before.len(), 1);

        // Undo
        AdvancementService::undo_advance(&conn, 1).unwrap();

        // Verify edge was removed
        let edges_after = CharacterEdgeRepository::get_by_character_id(&conn, 1).unwrap();
        assert_eq!(edges_after.len(), 0);

        // Verify advance was removed
        let history = AdvancementService::get_advancement_history(&conn, 1).unwrap();
        assert_eq!(history.len(), 0);
    }
}
