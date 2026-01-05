use serde::{Deserialize, Serialize};

use crate::views::{Die, SkillView};

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct CharacterSkillValue {
    pub skill: SkillView,
    /// The purchased die value (None for untrained).
    pub die: Option<Die>,
    /// The effective die value (purchased + modifiers). None for untrained (UI displays d4-2).
    pub effective_die: Option<Die>,
    /// Whether the current die is above the linked attribute's die.
    pub is_above_attribute: bool,
    /// The cost to increment this skill (1 if at/below attribute, 2 if above).
    pub increment_cost: i64,
    /// Whether this skill can be incremented (has points, not at max).
    pub can_increment: bool,
    /// Whether this skill can be decremented (has die, and not a core skill at d4).
    pub can_decrement: bool,
}

impl CharacterSkillValue {
    pub fn new(skill: SkillView, die: Option<Die>) -> Self {
        // When no modifiers, effective equals purchased
        Self {
            skill,
            die,
            effective_die: die,
            is_above_attribute: false,
            increment_cost: 1,
            can_increment: false,
            can_decrement: false,
        }
    }

    /// Create with explicit effective value (when modifiers are applied).
    pub fn with_effective(
        skill: SkillView,
        die: Option<Die>,
        effective_die: Option<Die>,
        is_above_attribute: bool,
        increment_cost: i64,
        can_increment: bool,
        can_decrement: bool,
    ) -> Self {
        Self {
            skill,
            die,
            effective_die,
            is_above_attribute,
            increment_cost,
            can_increment,
            can_decrement,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Skill;

    fn create_test_skill() -> Skill {
        Skill {
            id: 1,
            name: "Fighting".to_string(),
            description: "Melee combat".to_string(),
            linked_attribute_id: 1,
            is_core_skill: true,
            default_die_size: Some(4),
            max_die_size: 12,
            max_die_modifier: 0,
            source: "core".to_string(),
            created_at: "2024-01-01".to_string(),
            updated_at: "2024-01-01".to_string(),
        }
    }

    #[test]
    fn new_with_trained_skill() {
        let skill_view = SkillView::new(create_test_skill());
        let value = CharacterSkillValue::new(skill_view, Some(Die::d8()));

        assert_eq!(value.skill.name, "Fighting");
        assert_eq!(value.die, Some(Die::d8()));
        assert_eq!(value.effective_die, Some(Die::d8()));
    }

    #[test]
    fn new_with_untrained_skill() {
        let skill_view = SkillView::new(create_test_skill());
        let value = CharacterSkillValue::new(skill_view, None);

        assert_eq!(value.skill.name, "Fighting");
        assert_eq!(value.die, None);
        assert_eq!(value.effective_die, None);
    }

    #[test]
    fn with_effective_creates_value_with_modifier() {
        let skill_view = SkillView::new(create_test_skill());
        let purchased = Some(Die::d6());
        let effective = Some(Die::d8()); // +1 from modifier

        let value = CharacterSkillValue::with_effective(
            skill_view,
            purchased,
            effective,
            true,  // is_above_attribute
            2,     // increment_cost (above attribute)
            true,  // can_increment
            true,  // can_decrement
        );

        assert_eq!(value.die, Some(Die::d6()));
        assert_eq!(value.effective_die, Some(Die::d8()));
        assert!(value.is_above_attribute);
        assert_eq!(value.increment_cost, 2);
        assert!(value.can_increment);
        assert!(value.can_decrement);
    }
}
