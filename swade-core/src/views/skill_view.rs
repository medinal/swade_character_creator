use serde::{Deserialize, Serialize};

use crate::models::Skill;
use crate::views::Die;

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct SkillView {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub linked_attribute_id: i64,
    pub is_core_skill: bool,
    pub default_die: Option<Die>,
    pub max_die: Die,
    pub source: String,
}

impl SkillView {
    pub fn new(skill: Skill) -> Self {
        let default_die = skill.default_die_size.and_then(|size| Die::new(size as u8));

        let max_die = Die::with_modifier(skill.max_die_size as u8, skill.max_die_modifier as u8)
            .unwrap_or(Die::d12());

        Self {
            id: skill.id,
            name: skill.name,
            description: skill.description,
            linked_attribute_id: skill.linked_attribute_id,
            is_core_skill: skill.is_core_skill,
            default_die,
            max_die,
            source: skill.source,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_skill(is_core: bool, default_die_size: Option<i64>) -> Skill {
        Skill {
            id: 1,
            name: "Athletics".to_string(),
            description: "Running, jumping, climbing".to_string(),
            linked_attribute_id: 1,
            is_core_skill: is_core,
            default_die_size,
            max_die_size: 12,
            max_die_modifier: 0,
            source: "core".to_string(),
            created_at: "2024-01-01".to_string(),
            updated_at: "2024-01-01".to_string(),
        }
    }

    #[test]
    fn new_core_skill_has_default_die() {
        let skill = create_test_skill(true, Some(4));
        let view = SkillView::new(skill);

        assert_eq!(view.id, 1);
        assert_eq!(view.name, "Athletics");
        assert_eq!(view.is_core_skill, true);
        assert_eq!(view.default_die, Some(Die::d4()));
    }

    #[test]
    fn new_non_core_skill_has_no_default_die() {
        let skill = create_test_skill(false, None);
        let view = SkillView::new(skill);

        assert_eq!(view.is_core_skill, false);
        assert_eq!(view.default_die, None);
    }

    #[test]
    fn new_converts_max_die() {
        let skill = create_test_skill(true, Some(4));
        let view = SkillView::new(skill);

        assert_eq!(view.max_die, Die::d12());
    }

    #[test]
    fn new_handles_max_die_with_modifier() {
        let mut skill = create_test_skill(true, Some(4));
        skill.max_die_size = 12;
        skill.max_die_modifier = 2;

        let view = SkillView::new(skill);

        assert_eq!(view.max_die, Die::with_modifier(12, 2).unwrap());
    }
}
