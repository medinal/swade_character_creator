use serde::{Deserialize, Serialize};

use crate::models::{Ancestry, Modifier};
use crate::views::{AncestryChoiceView, RequirementTree};

/// View model for an ancestry with all its related data.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct AncestryView {
    pub id: i64,
    pub name: String,
    pub source: String,
    pub description: String,
    pub choices: Vec<AncestryChoiceView>,
    pub modifiers: Vec<Modifier>,
    pub requirements: RequirementTree,
}

impl AncestryView {
    pub fn new(
        ancestry: Ancestry,
        choices: Vec<AncestryChoiceView>,
        modifiers: Vec<Modifier>,
        requirements: RequirementTree,
    ) -> Self {
        Self {
            id: ancestry.id,
            name: ancestry.name,
            source: ancestry.source,
            description: ancestry.description,
            choices,
            modifiers,
            requirements,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{AncestryChoice, AncestryChoiceOption, Requirement};
    use crate::views::{AncestryChoiceOptionView, AncestryChoiceType};

    fn create_test_ancestry() -> Ancestry {
        Ancestry {
            id: 1,
            name: "Human".to_string(),
            source: "core".to_string(),
            description: "Humans are the most adaptable race.".to_string(),
            created_at: "2024-01-01".to_string(),
            updated_at: "2024-01-01".to_string(),
        }
    }

    fn create_test_choice_view() -> AncestryChoiceView {
        AncestryChoiceView::new(
            AncestryChoice {
                id: 1,
                ancestry_id: 1,
                choice_type: "free_edge".to_string(),
                choice_category: Some("background".to_string()),
                min_selections: 1,
                max_selections: 1,
                description: "Choose one free Edge".to_string(),
                created_at: "2024-01-01".to_string(),
                updated_at: "2024-01-01".to_string(),
            },
            vec![AncestryChoiceOptionView::new(AncestryChoiceOption {
                id: 1,
                choice_id: 1,
                option_type: "edge".to_string(),
                option_id: None,
                option_description: Some("Any Background Edge".to_string()),
                created_at: "2024-01-01".to_string(),
                updated_at: "2024-01-01".to_string(),
            })],
        )
    }

    fn create_test_modifier() -> Modifier {
        Modifier {
            id: 1,
            target_type: Some("attribute".to_string()),
            target_identifier: Some("Agility".to_string()),
            value_type: "die_increment".to_string(),
            value: Some(1),
            description: "+1 die type to Agility".to_string(),
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
    fn new_creates_ancestry_view() {
        let ancestry = create_test_ancestry();
        let choices = vec![create_test_choice_view()];
        let modifiers = vec![create_test_modifier()];
        let requirements = RequirementTree::leaf(create_test_requirement());

        let view = AncestryView::new(ancestry, choices, modifiers, requirements);

        assert_eq!(view.id, 1);
        assert_eq!(view.name, "Human");
        assert_eq!(view.source, "core");
        assert_eq!(view.description, "Humans are the most adaptable race.");
        assert_eq!(view.choices.len(), 1);
        assert_eq!(view.modifiers.len(), 1);
        assert!(!view.requirements.is_empty());
    }

    #[test]
    fn new_with_no_choices() {
        let ancestry = create_test_ancestry();

        let view = AncestryView::new(ancestry, vec![], vec![], RequirementTree::none());

        assert!(view.choices.is_empty());
        assert!(view.modifiers.is_empty());
        assert!(view.requirements.is_empty());
    }

    #[test]
    fn new_with_multiple_choices() {
        let ancestry = create_test_ancestry();
        let choices = vec![
            create_test_choice_view(),
            AncestryChoiceView {
                id: 2,
                choice_type: AncestryChoiceType::MandatoryHindrance,
                choice_category: None,
                min_selections: 1,
                max_selections: 1,
                description: "Choose a hindrance".to_string(),
                options: vec![],
            },
        ];

        let view = AncestryView::new(ancestry, choices, vec![], RequirementTree::none());

        assert_eq!(view.choices.len(), 2);
    }

    #[test]
    fn new_with_multiple_modifiers() {
        let ancestry = create_test_ancestry();
        let modifiers = vec![create_test_modifier(), create_test_modifier()];

        let view = AncestryView::new(ancestry, vec![], modifiers, RequirementTree::none());

        assert_eq!(view.modifiers.len(), 2);
    }
}
