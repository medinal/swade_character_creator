use serde::{Deserialize, Serialize};

use crate::constants::{BASE_PACE, BASE_PARRY, BASE_TOUGHNESS};
use crate::models::{Modifier, Rank};
use crate::views::{
    AncestryView, CharacterAncestryChoiceValue, CharacterArcaneBackgroundChoiceValue,
    CharacterArcaneBackgroundValue, CharacterAttributeValue, CharacterEdgeValue,
    CharacterGearValue, CharacterHindranceValue, CharacterNoteValue, CharacterPowerValue,
    CharacterSkillValue, DerivedStatsView, Die, EncumbranceInfo, RequirementContext,
};

/// Complete view of a character with all related data resolved.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct CharacterView {
    pub id: i64,
    pub is_wild_card: bool,
    pub name: String,

    // Ancestry
    pub ancestry: Option<AncestryView>,
    pub ancestry_choices: Vec<CharacterAncestryChoiceValue>,

    // Rank and Advances
    pub rank: Rank,
    pub current_advances: i64,

    // Attributes and Skills
    pub attributes: Vec<CharacterAttributeValue>,
    pub skills: Vec<CharacterSkillValue>,

    // Edges and Hindrances
    pub edges: Vec<CharacterEdgeValue>,
    pub hindrances: Vec<CharacterHindranceValue>,

    // Arcane Backgrounds and Powers
    pub arcane_backgrounds: Vec<CharacterArcaneBackgroundValue>,
    pub arcane_background_choices: Vec<CharacterArcaneBackgroundChoiceValue>,
    pub powers: Vec<CharacterPowerValue>,
    pub power_points: i64,
    pub power_points_used: i64,

    // Status Tracking
    pub wounds: i64,
    pub fatigue: i64,

    // Notes
    pub notes: Vec<CharacterNoteValue>,

    // Gear and Encumbrance
    pub gear: Vec<CharacterGearValue>,
    pub encumbrance: EncumbranceInfo,

    // Modifiers (direct modifiers applied to this character)
    pub modifiers: Vec<Modifier>,

    // Derived Stats (computed from attributes, skills, and modifiers)
    pub derived_stats: DerivedStatsView,

    // Character Creation Tracking
    pub attribute_points_spent: i64,
    pub attribute_points_earned: i64,
    pub skill_points_spent: i64,
    pub skill_points_earned: i64,
    pub hindrance_points_spent: i64,
    pub hindrance_points_earned: i64,

    // Hindrance Points Conversion Tracking
    pub hindrance_points_to_edges: i64,
    pub hindrance_points_to_attributes: i64,
    pub hindrance_points_to_skills: i64,
    pub hindrance_points_to_wealth: i64,

    // Additional Character Info
    pub wealth: i64,
    pub background: Option<String>,
    pub description: Option<String>,

    // Portrait (as data URL for frontend display)
    pub portrait_data_url: Option<String>,
}

impl CharacterView {
    /// Aggregate modifier values from all sources (character, ancestry, edges, hindrances).
    ///
    /// Takes a predicate to filter which modifiers to include.
    fn aggregate_modifiers<P>(&self, predicate: P) -> i64
    where
        P: Fn(&Modifier) -> bool,
    {
        let mut total = 0i64;

        // Direct character modifiers
        for m in &self.modifiers {
            if predicate(m) {
                total += m.value.unwrap_or(0);
            }
        }

        // Ancestry modifiers
        if let Some(ancestry) = &self.ancestry {
            for m in &ancestry.modifiers {
                if predicate(m) {
                    total += m.value.unwrap_or(0);
                }
            }
        }

        // Edge modifiers
        for edge_value in &self.edges {
            for m in &edge_value.edge.modifiers {
                if predicate(m) {
                    total += m.value.unwrap_or(0);
                }
            }
        }

        // Hindrance modifiers
        for hindrance_value in &self.hindrances {
            for m in &hindrance_value.hindrance.modifiers {
                if predicate(m) {
                    total += m.value.unwrap_or(0);
                }
            }
        }

        // Equipped gear modifiers
        for gear_value in &self.gear {
            if gear_value.is_equipped {
                for m in &gear_value.gear.modifiers {
                    if predicate(m) {
                        total += m.value.unwrap_or(0);
                    }
                }
            }
        }

        total
    }

    /// Sum all die_increment modifiers for a specific target (attribute or skill).
    fn sum_die_increments(&self, target_type: &str, target_identifier: &str) -> i64 {
        self.aggregate_modifiers(|m| {
            m.value_type == "die_increment"
                && m.target_type.as_deref() == Some(target_type)
                && m.target_identifier.as_deref() == Some(target_identifier)
        })
    }

    /// Sum all flat_bonus modifiers for a derived stat (pace, parry, toughness, size).
    fn sum_flat_bonuses(&self, target_identifier: &str) -> i64 {
        self.aggregate_modifiers(|m| {
            m.value_type == "flat_bonus"
                && m.target_type.as_deref() == Some("derived_stat")
                && m.target_identifier.as_deref() == Some(target_identifier)
        })
    }

    /// Apply die increments (positive or negative) to a base die.
    fn apply_die_increments(base: Die, increments: i64) -> Die {
        let mut die = base;
        if increments > 0 {
            for _ in 0..increments {
                die = die.increment();
            }
        } else if increments < 0 {
            for _ in 0..(-increments) {
                if let Some(decremented) = die.decrement() {
                    die = decremented;
                } else {
                    // Can't go below d4
                    break;
                }
            }
        }
        die
    }

    /// Get the effective die for an attribute (purchased die + modifiers).
    ///
    /// Returns None if the attribute is not found.
    pub fn get_effective_attribute_die(&self, attribute_id: i64) -> Option<Die> {
        let attr = self.attributes.iter().find(|a| a.attribute.id == attribute_id)?;
        let increments = self.sum_die_increments("attribute", &attr.attribute.name);
        Some(Self::apply_die_increments(attr.die, increments))
    }

    /// Get the effective base die for an attribute (d4 + modifiers).
    ///
    /// This is the starting point for a character with this character's modifiers.
    /// Returns None if the attribute is not found.
    pub fn get_attribute_base_die(&self, attribute_id: i64) -> Option<Die> {
        let attr = self.attributes.iter().find(|a| a.attribute.id == attribute_id)?;
        let increments = self.sum_die_increments("attribute", &attr.attribute.name);
        Some(Self::apply_die_increments(attr.attribute.base_die, increments))
    }

    /// Get the effective max die for an attribute (d12 + modifiers).
    ///
    /// This is the ceiling for a character with this character's modifiers.
    /// Returns None if the attribute is not found.
    pub fn get_attribute_max_die(&self, attribute_id: i64) -> Option<Die> {
        let attr = self.attributes.iter().find(|a| a.attribute.id == attribute_id)?;
        let increments = self.sum_die_increments("attribute", &attr.attribute.name);
        Some(Self::apply_die_increments(Die::d12(), increments))
    }

    /// Get the effective die for a skill (purchased die + modifiers).
    ///
    /// For untrained skills (die is None), returns None - the UI should display "d4-2".
    /// Returns None if the skill is not found.
    pub fn get_effective_skill_die(&self, skill_id: i64) -> Option<Die> {
        let skill_value = self.skills.iter().find(|s| s.skill.id == skill_id)?;

        // If untrained, return None (UI displays d4-2)
        let purchased_die = skill_value.die?;

        let increments = self.sum_die_increments("skill", &skill_value.skill.name);
        Some(Self::apply_die_increments(purchased_die, increments))
    }

    /// Get the effective die for a skill's linked attribute.
    ///
    /// This is used for skill point cost calculations - costs 1 point per step
    /// up to the linked attribute's effective die, then 2 points per step after.
    pub fn get_linked_attribute_effective_die(&self, skill_id: i64) -> Option<Die> {
        let skill_value = self.skills.iter().find(|s| s.skill.id == skill_id)?;
        self.get_effective_attribute_die(skill_value.skill.linked_attribute_id)
    }

    /// Compute and update effective values for all attributes and skills.
    ///
    /// This should be called after building the CharacterView to populate
    /// the effective_die, base_die, max_die, and action state fields based on modifiers.
    pub fn compute_effective_values(&mut self) {
        // Include hindrance points converted to attribute points
        let attr_points_remaining = self.attribute_points_earned
            + self.hindrance_points_to_attributes
            - self.attribute_points_spent;

        // First, collect all attribute increments (to avoid borrow issues)
        let attr_increments: Vec<(usize, i64)> = self
            .attributes
            .iter()
            .enumerate()
            .map(|(i, attr_value)| {
                let increments = self.sum_die_increments("attribute", &attr_value.attribute.name);
                (i, increments)
            })
            .collect();

        // Apply attribute increments and compute action states
        for (i, increments) in attr_increments {
            let attr_value = &mut self.attributes[i];
            attr_value.effective_die = Self::apply_die_increments(attr_value.die, increments);
            attr_value.base_die =
                Self::apply_die_increments(attr_value.attribute.base_die, increments);
            attr_value.max_die = Self::apply_die_increments(Die::d12(), increments);

            // Action states for attributes
            attr_value.can_increment =
                attr_points_remaining > 0 && attr_value.effective_die < attr_value.max_die;
            attr_value.can_decrement = attr_value.die > attr_value.attribute.base_die;
        }

        // Build a map of attribute_id -> effective_die for skill cost calculation
        let attr_effective_dies: std::collections::HashMap<i64, Die> = self
            .attributes
            .iter()
            .map(|a| (a.attribute.id, a.effective_die))
            .collect();

        // Include hindrance points converted to skill points
        let skill_points_remaining = self.skill_points_earned
            + self.hindrance_points_to_skills
            - self.skill_points_spent;

        // Collect skill increments and linked attribute info
        let skill_data: Vec<(usize, Option<i64>, Die)> = self
            .skills
            .iter()
            .enumerate()
            .map(|(i, skill_value)| {
                let increments = if skill_value.die.is_some() {
                    Some(self.sum_die_increments("skill", &skill_value.skill.name))
                } else {
                    None
                };
                let linked_attr_die = attr_effective_dies
                    .get(&skill_value.skill.linked_attribute_id)
                    .copied()
                    .unwrap_or(Die::d4());
                (i, increments, linked_attr_die)
            })
            .collect();

        // Apply skill increments and compute action states
        for (i, maybe_increments, linked_attr_die) in skill_data {
            let skill_value = &mut self.skills[i];

            // Compute effective die
            if let (Some(purchased_die), Some(increments)) = (skill_value.die, maybe_increments) {
                skill_value.effective_die =
                    Some(Self::apply_die_increments(purchased_die, increments));
            } else {
                skill_value.effective_die = None;
            }

            // Compute is_above_attribute
            skill_value.is_above_attribute = skill_value
                .effective_die
                .map(|d| d > linked_attr_die)
                .unwrap_or(false);

            // Compute increment_cost
            // If untrained (no die), first increment costs 1
            // Otherwise: costs 1 if next die <= linked attr, 2 if above
            skill_value.increment_cost = match skill_value.effective_die {
                None => 1, // Untrained -> d4 costs 1
                Some(current_die) => {
                    let next_die = current_die.increment();
                    if next_die > linked_attr_die {
                        2
                    } else {
                        1
                    }
                }
            };

            // Compute can_increment
            // Has enough points and not at max die
            let at_max = skill_value
                .effective_die
                .map(|d| d >= skill_value.skill.max_die)
                .unwrap_or(false);
            skill_value.can_increment =
                skill_points_remaining >= skill_value.increment_cost && !at_max;

            // Compute can_decrement
            // Has a die and not (core skill at d4)
            let is_at_d4 = skill_value.die.map(|d| d == Die::d4()).unwrap_or(false);
            let is_core_at_min = skill_value.skill.is_core_skill && is_at_d4;
            skill_value.can_decrement = skill_value.die.is_some() && !is_core_at_min;
        }

        // Compute derived stats (must come after attributes/skills are computed)
        self.derived_stats = self.compute_derived_stats();

        // Compute encumbrance
        self.encumbrance = self.compute_encumbrance();
    }

    /// Compute encumbrance based on carried gear weight and Strength die.
    fn compute_encumbrance(&self) -> EncumbranceInfo {
        let total_weight: f64 = self.gear.iter().map(|g| g.total_weight()).sum();

        // Get Strength effective die size
        let strength_die_size = self
            .attributes
            .iter()
            .find(|a| a.attribute.name == "Strength")
            .map(|a| a.effective_die.size())
            .unwrap_or(4);

        EncumbranceInfo::from_weight_and_strength(total_weight, strength_die_size)
    }

    /// Create a RequirementContext for evaluating edge/power requirements.
    ///
    /// This extracts the minimal data needed to check requirements without
    /// passing the full CharacterView (which would cause circular dependencies).
    pub fn to_requirement_context(&self) -> RequirementContext {
        let mut attribute_dies = std::collections::HashMap::new();
        for attr in &self.attributes {
            attribute_dies.insert(attr.attribute.id, attr.effective_die.size());
        }

        let mut skill_dies = std::collections::HashMap::new();
        for skill in &self.skills {
            skill_dies.insert(skill.skill.id, skill.effective_die.map(|d| d.size()));
        }

        let edge_ids: std::collections::HashSet<i64> =
            self.edges.iter().map(|e| e.edge.id).collect();

        let arcane_background_ids: std::collections::HashSet<i64> = self
            .arcane_backgrounds
            .iter()
            .map(|ab| ab.arcane_background.id)
            .collect();

        // For arcane skills, we use the same skill_dies map
        // The arcane_skill requirement type checks against regular skills
        let arcane_skill_dies = skill_dies.clone();

        RequirementContext {
            rank_id: self.rank.id,
            is_wild_card: self.is_wild_card,
            attribute_dies,
            skill_dies,
            edge_ids,
            arcane_background_ids,
            arcane_skill_dies,
        }
    }

    /// Compute derived stats from attributes, skills, and modifiers.
    ///
    /// SWADE formulas:
    /// - Pace: 6 + flat_bonus modifiers
    /// - Parry: 2 + (Fighting die / 2) + flat_bonus modifiers
    /// - Toughness: 2 + (Vigor die / 2) + Size + flat_bonus modifiers
    /// - Size: sum of flat_bonus modifiers (normal is 0)
    fn compute_derived_stats(&self) -> DerivedStatsView {
        // Size (affects toughness, must be computed first)
        let size = self.sum_flat_bonuses("size") as i32;

        // Pace: base + modifiers
        let pace = BASE_PACE + self.sum_flat_bonuses("pace") as i32;

        // Parry: base + (Fighting die / 2) + modifiers
        let fighting_bonus = self
            .skills
            .iter()
            .find(|s| s.skill.name == "Fighting")
            .and_then(|s| s.effective_die)
            .map(|d| d.size() as i32 / 2)
            .unwrap_or(0);
        let parry = BASE_PARRY + fighting_bonus + self.sum_flat_bonuses("parry") as i32;

        // Toughness: base + (Vigor die / 2) + size + modifiers
        let vigor_bonus = self
            .attributes
            .iter()
            .find(|a| a.attribute.name == "Vigor")
            .map(|a| a.effective_die.size() as i32 / 2)
            .unwrap_or(0);
        let toughness = BASE_TOUGHNESS + vigor_bonus + size + self.sum_flat_bonuses("toughness") as i32;

        DerivedStatsView {
            pace,
            parry,
            toughness,
            size,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Modifier;
    use crate::views::{AttributeView, SkillView};

    fn create_test_rank() -> Rank {
        Rank {
            id: 1,
            name: "Novice".to_string(),
            min_advances: 0,
            max_advances: Some(3),
            description: "Starting rank".to_string(),
            created_at: "2024-01-01".to_string(),
            updated_at: "2024-01-01".to_string(),
        }
    }

    fn create_test_character() -> CharacterView {
        CharacterView {
            id: 1,
            is_wild_card: true,
            name: "Test Character".to_string(),
            ancestry: None,
            ancestry_choices: vec![],
            rank: create_test_rank(),
            current_advances: 0,
            attributes: vec![
                CharacterAttributeValue {
                    attribute: AttributeView {
                        id: 1,
                        name: "Agility".to_string(),
                        description: "".to_string(),
                        base_die: Die::d4(),
                    },
                    die: Die::d6(),
                    effective_die: Die::d6(),
                    base_die: Die::d4(),
                    max_die: Die::d12(),
                    can_increment: false,
                    can_decrement: false,
                },
                CharacterAttributeValue {
                    attribute: AttributeView {
                        id: 5,
                        name: "Vigor".to_string(),
                        description: "".to_string(),
                        base_die: Die::d4(),
                    },
                    die: Die::d4(),
                    effective_die: Die::d4(),
                    base_die: Die::d4(),
                    max_die: Die::d12(),
                    can_increment: false,
                    can_decrement: false,
                },
            ],
            skills: vec![
                CharacterSkillValue {
                    skill: SkillView {
                        id: 1,
                        name: "Fighting".to_string(),
                        description: "".to_string(),
                        linked_attribute_id: 1,
                        is_core_skill: true,
                        default_die: Some(Die::d4()),
                        max_die: Die::d12(),
                        source: "core".to_string(),
                    },
                    die: Some(Die::d6()),
                    effective_die: Some(Die::d6()),
                    is_above_attribute: false,
                    increment_cost: 1,
                    can_increment: false,
                    can_decrement: false,
                },
                CharacterSkillValue {
                    skill: SkillView {
                        id: 2,
                        name: "Notice".to_string(),
                        description: "".to_string(),
                        linked_attribute_id: 2,
                        is_core_skill: true,
                        default_die: Some(Die::d4()),
                        max_die: Die::d12(),
                        source: "core".to_string(),
                    },
                    die: None, // Untrained
                    effective_die: None,
                    is_above_attribute: false,
                    increment_cost: 1,
                    can_increment: false,
                    can_decrement: false,
                },
            ],
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
            encumbrance: EncumbranceInfo::empty(4),
            modifiers: vec![],
            derived_stats: DerivedStatsView::default(),
            attribute_points_spent: 1,
            attribute_points_earned: 5,
            skill_points_spent: 0,
            skill_points_earned: 12,
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
        }
    }

    fn create_die_increment_modifier(target_type: &str, target_name: &str, value: i64) -> Modifier {
        Modifier {
            id: 1,
            target_type: Some(target_type.to_string()),
            target_identifier: Some(target_name.to_string()),
            value_type: "die_increment".to_string(),
            value: Some(value),
            description: format!("+{} die to {}", value, target_name),
            created_at: "2024-01-01".to_string(),
            updated_at: "2024-01-01".to_string(),
        }
    }

    #[test]
    fn test_get_effective_attribute_die_no_modifiers() {
        let character = create_test_character();

        // Agility is d6, no modifiers
        let effective = character.get_effective_attribute_die(1).unwrap();
        assert_eq!(effective, Die::d6());
    }

    #[test]
    fn test_get_effective_attribute_die_with_modifier() {
        let mut character = create_test_character();
        character.modifiers.push(create_die_increment_modifier("attribute", "Vigor", 1));

        // Vigor is d4, +1 modifier = d6
        let effective = character.get_effective_attribute_die(5).unwrap();
        assert_eq!(effective, Die::d6());
    }

    #[test]
    fn test_get_attribute_base_die_with_modifier() {
        let mut character = create_test_character();
        character.modifiers.push(create_die_increment_modifier("attribute", "Vigor", 1));

        // Base is d4, +1 modifier = d6
        let base = character.get_attribute_base_die(5).unwrap();
        assert_eq!(base, Die::d6());
    }

    #[test]
    fn test_get_attribute_max_die_with_modifier() {
        let mut character = create_test_character();
        character.modifiers.push(create_die_increment_modifier("attribute", "Vigor", 1));

        // Max is d12, +1 modifier = d12+1
        let max = character.get_attribute_max_die(5).unwrap();
        assert_eq!(max, Die::with_modifier(12, 1).unwrap());
    }

    #[test]
    fn test_get_effective_skill_die_trained() {
        let character = create_test_character();

        // Fighting is d6, no modifiers
        let effective = character.get_effective_skill_die(1).unwrap();
        assert_eq!(effective, Die::d6());
    }

    #[test]
    fn test_get_effective_skill_die_untrained() {
        let character = create_test_character();

        // Notice is untrained, should return None
        let effective = character.get_effective_skill_die(2);
        assert!(effective.is_none());
    }

    #[test]
    fn test_get_effective_skill_die_with_modifier() {
        let mut character = create_test_character();
        character.modifiers.push(create_die_increment_modifier("skill", "Fighting", 1));

        // Fighting is d6, +1 modifier = d8
        let effective = character.get_effective_skill_die(1).unwrap();
        assert_eq!(effective, Die::d8());
    }

    #[test]
    fn test_get_linked_attribute_effective_die() {
        let mut character = create_test_character();
        character.modifiers.push(create_die_increment_modifier("attribute", "Agility", 1));

        // Fighting is linked to Agility (id 1), Agility is d6 + 1 = d8
        let linked = character.get_linked_attribute_effective_die(1).unwrap();
        assert_eq!(linked, Die::d8());
    }

    #[test]
    fn test_sum_die_increments_from_multiple_sources() {
        let mut character = create_test_character();

        // Add modifier from direct modifiers
        character.modifiers.push(create_die_increment_modifier("attribute", "Vigor", 1));

        // Add modifier from ancestry
        use crate::views::{AncestryView, RequirementTree};
        character.ancestry = Some(AncestryView {
            id: 1,
            name: "Dwarf".to_string(),
            source: "core".to_string(),
            description: "".to_string(),
            choices: vec![],
            modifiers: vec![create_die_increment_modifier("attribute", "Vigor", 1)],
            requirements: RequirementTree::none(),
        });

        // Vigor is d4, +1 from direct +1 from ancestry = d8
        let effective = character.get_effective_attribute_die(5).unwrap();
        assert_eq!(effective, Die::d8());
    }

    #[test]
    fn test_attribute_not_found() {
        let character = create_test_character();

        let result = character.get_effective_attribute_die(999);
        assert!(result.is_none());
    }

    #[test]
    fn test_skill_not_found() {
        let character = create_test_character();

        let result = character.get_effective_skill_die(999);
        assert!(result.is_none());
    }

    fn create_flat_bonus_modifier(target_identifier: &str, value: i64) -> Modifier {
        Modifier {
            id: 1,
            target_type: Some("derived_stat".to_string()),
            target_identifier: Some(target_identifier.to_string()),
            value_type: "flat_bonus".to_string(),
            value: Some(value),
            description: format!("+{} to {}", value, target_identifier),
            created_at: "2024-01-01".to_string(),
            updated_at: "2024-01-01".to_string(),
        }
    }

    #[test]
    fn test_compute_derived_stats_base_values() {
        let mut character = create_test_character();
        character.compute_effective_values();

        // Fighting is d6, Vigor is d4
        // Pace: 6 (base)
        // Parry: 2 + (6/2) = 5
        // Toughness: 2 + (4/2) = 4
        // Size: 0
        assert_eq!(character.derived_stats.pace, 6);
        assert_eq!(character.derived_stats.parry, 5);
        assert_eq!(character.derived_stats.toughness, 4);
        assert_eq!(character.derived_stats.size, 0);
    }

    #[test]
    fn test_compute_derived_stats_with_pace_modifier() {
        let mut character = create_test_character();
        character.modifiers.push(create_flat_bonus_modifier("pace", 2));
        character.compute_effective_values();

        // Pace: 6 + 2 = 8
        assert_eq!(character.derived_stats.pace, 8);
    }

    #[test]
    fn test_compute_derived_stats_with_parry_modifier() {
        let mut character = create_test_character();
        character.modifiers.push(create_flat_bonus_modifier("parry", 1));
        character.compute_effective_values();

        // Parry: 2 + (6/2) + 1 = 6
        assert_eq!(character.derived_stats.parry, 6);
    }

    #[test]
    fn test_compute_derived_stats_with_toughness_modifier() {
        let mut character = create_test_character();
        character.modifiers.push(create_flat_bonus_modifier("toughness", 1));
        character.compute_effective_values();

        // Toughness: 2 + (4/2) + 1 = 5
        assert_eq!(character.derived_stats.toughness, 5);
    }

    #[test]
    fn test_compute_derived_stats_size_affects_toughness() {
        let mut character = create_test_character();
        character.modifiers.push(create_flat_bonus_modifier("size", 1));
        character.compute_effective_values();

        // Size: 1
        // Toughness: 2 + (4/2) + 1 (size) = 5
        assert_eq!(character.derived_stats.size, 1);
        assert_eq!(character.derived_stats.toughness, 5);
    }

    #[test]
    fn test_compute_derived_stats_with_higher_vigor() {
        let mut character = create_test_character();
        // Increment Vigor from d4 to d8
        character.attributes[1].die = Die::d8();
        character.attributes[1].effective_die = Die::d8();
        character.compute_effective_values();

        // Toughness: 2 + (8/2) = 6
        assert_eq!(character.derived_stats.toughness, 6);
    }

    #[test]
    fn test_compute_derived_stats_with_higher_fighting() {
        let mut character = create_test_character();
        // Increment Fighting from d6 to d10
        character.skills[0].die = Some(Die::d10());
        character.skills[0].effective_die = Some(Die::d10());
        character.compute_effective_values();

        // Parry: 2 + (10/2) = 7
        assert_eq!(character.derived_stats.parry, 7);
    }
}
