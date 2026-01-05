use serde::{Deserialize, Serialize};
use std::str::FromStr;

use crate::models::AncestryChoice;
use crate::views::{AncestryChoiceOptionView, AncestryChoiceType};

/// View model for an ancestry choice with its options.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct AncestryChoiceView {
    pub id: i64,
    pub choice_type: AncestryChoiceType,
    pub choice_category: Option<String>,
    pub min_selections: i64,
    pub max_selections: i64,
    pub description: String,
    pub options: Vec<AncestryChoiceOptionView>,
}

impl AncestryChoiceView {
    pub fn new(choice: AncestryChoice, options: Vec<AncestryChoiceOptionView>) -> Self {
        let choice_type = AncestryChoiceType::from_str(&choice.choice_type)
            .unwrap_or(AncestryChoiceType::FreeEdge);

        Self {
            id: choice.id,
            choice_type,
            choice_category: choice.choice_category,
            min_selections: choice.min_selections,
            max_selections: choice.max_selections,
            description: choice.description,
            options,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::AncestryChoiceOption;
    use crate::views::AncestryOptionType;

    fn create_test_choice() -> AncestryChoice {
        AncestryChoice {
            id: 1,
            ancestry_id: 1,
            choice_type: "free_edge".to_string(),
            choice_category: Some("background".to_string()),
            min_selections: 1,
            max_selections: 1,
            description: "Choose one free Background Edge".to_string(),
            created_at: "2024-01-01".to_string(),
            updated_at: "2024-01-01".to_string(),
        }
    }

    fn create_test_option_view() -> AncestryChoiceOptionView {
        AncestryChoiceOptionView::new(AncestryChoiceOption {
            id: 1,
            choice_id: 1,
            option_type: "edge".to_string(),
            option_id: Some(10),
            option_description: Some("Alertness".to_string()),
            created_at: "2024-01-01".to_string(),
            updated_at: "2024-01-01".to_string(),
        })
    }

    #[test]
    fn new_creates_view() {
        let choice = create_test_choice();
        let options = vec![create_test_option_view()];

        let view = AncestryChoiceView::new(choice, options);

        assert_eq!(view.id, 1);
        assert_eq!(view.choice_type, AncestryChoiceType::FreeEdge);
        assert_eq!(view.choice_category, Some("background".to_string()));
        assert_eq!(view.min_selections, 1);
        assert_eq!(view.max_selections, 1);
        assert_eq!(view.description, "Choose one free Background Edge");
        assert_eq!(view.options.len(), 1);
    }

    #[test]
    fn new_with_mandatory_hindrance_type() {
        let mut choice = create_test_choice();
        choice.choice_type = "mandatory_hindrance".to_string();

        let view = AncestryChoiceView::new(choice, vec![]);

        assert_eq!(view.choice_type, AncestryChoiceType::MandatoryHindrance);
    }

    #[test]
    fn new_with_ancestral_enemy_type() {
        let mut choice = create_test_choice();
        choice.choice_type = "ancestral_enemy".to_string();

        let view = AncestryChoiceView::new(choice, vec![]);

        assert_eq!(view.choice_type, AncestryChoiceType::AncestralEnemy);
    }

    #[test]
    fn new_with_null_category() {
        let mut choice = create_test_choice();
        choice.choice_category = None;

        let view = AncestryChoiceView::new(choice, vec![]);

        assert_eq!(view.choice_category, None);
    }

    #[test]
    fn new_with_multiple_options() {
        let choice = create_test_choice();
        let options = vec![
            create_test_option_view(),
            AncestryChoiceOptionView {
                id: 2,
                option_type: AncestryOptionType::Edge,
                option_id: Some(20),
                description: Some("Brawny".to_string()),
            },
        ];

        let view = AncestryChoiceView::new(choice, options);

        assert_eq!(view.options.len(), 2);
    }

    #[test]
    fn new_with_no_options() {
        let choice = create_test_choice();

        let view = AncestryChoiceView::new(choice, vec![]);

        assert!(view.options.is_empty());
    }
}
