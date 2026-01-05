use serde::{Deserialize, Serialize};

use crate::models::ArcaneBackground;
use crate::views::{ArcaneBackgroundChoiceView, RequirementTree};

/// View model for an arcane background with its requirements and choices.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct ArcaneBackgroundView {
    pub id: i64,
    pub name: String,
    pub arcane_skill_id: i64,
    pub arcane_skill_name: Option<String>,
    pub starting_powers: i64,
    pub starting_power_points: i64,
    pub has_power_list: bool,
    pub source: String,
    pub description: String,
    pub requirements: RequirementTree,
    pub choices: Vec<ArcaneBackgroundChoiceView>,
}

impl ArcaneBackgroundView {
    pub fn new(
        arcane_background: ArcaneBackground,
        arcane_skill_name: Option<String>,
        requirements: RequirementTree,
        choices: Vec<ArcaneBackgroundChoiceView>,
    ) -> Self {
        Self {
            id: arcane_background.id,
            name: arcane_background.name,
            arcane_skill_id: arcane_background.arcane_skill_id,
            arcane_skill_name,
            starting_powers: arcane_background.starting_powers,
            starting_power_points: arcane_background.starting_power_points,
            has_power_list: arcane_background.has_power_list,
            source: arcane_background.source,
            description: arcane_background.description,
            requirements,
            choices,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Requirement;

    fn create_test_arcane_background() -> ArcaneBackground {
        ArcaneBackground {
            id: 1,
            name: "Magic".to_string(),
            arcane_skill_id: 10,
            starting_powers: 3,
            starting_power_points: 10,
            has_power_list: false,
            source: "core".to_string(),
            description: "Wizards and sorcerers".to_string(),
            created_at: "2024-01-01".to_string(),
            updated_at: "2024-01-01".to_string(),
        }
    }

    fn create_test_requirement() -> Requirement {
        Requirement {
            id: 1,
            requirement_type: "rank".to_string(),
            target_id: Some(1),
            value: Some(1),
            description: "Novice".to_string(),
            created_at: "2024-01-01".to_string(),
            updated_at: "2024-01-01".to_string(),
        }
    }

    #[test]
    fn new_creates_arcane_background_view() {
        let ab = create_test_arcane_background();
        let requirements = RequirementTree::leaf(create_test_requirement());

        let view =
            ArcaneBackgroundView::new(ab, Some("Spellcasting".to_string()), requirements, vec![]);

        assert_eq!(view.id, 1);
        assert_eq!(view.name, "Magic");
        assert_eq!(view.arcane_skill_id, 10);
        assert_eq!(view.arcane_skill_name, Some("Spellcasting".to_string()));
        assert_eq!(view.starting_powers, 3);
        assert_eq!(view.starting_power_points, 10);
        assert!(!view.has_power_list);
        assert_eq!(view.source, "core");
        assert_eq!(view.description, "Wizards and sorcerers");
        assert!(!view.requirements.is_empty());
        assert!(view.choices.is_empty());
    }

    #[test]
    fn new_with_no_skill_name() {
        let ab = create_test_arcane_background();

        let view = ArcaneBackgroundView::new(ab, None, RequirementTree::none(), vec![]);

        assert_eq!(view.arcane_skill_name, None);
    }

    #[test]
    fn new_with_no_requirements() {
        let ab = create_test_arcane_background();

        let view = ArcaneBackgroundView::new(
            ab,
            Some("Spellcasting".to_string()),
            RequirementTree::none(),
            vec![],
        );

        assert!(view.requirements.is_empty());
    }

    #[test]
    fn new_with_power_list() {
        let mut ab = create_test_arcane_background();
        ab.has_power_list = true;

        let view = ArcaneBackgroundView::new(ab, None, RequirementTree::none(), vec![]);

        assert!(view.has_power_list);
    }
}
