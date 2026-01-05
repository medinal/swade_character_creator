use serde::{Deserialize, Serialize};
use std::str::FromStr;

use crate::models::ArcaneBackgroundChoice;
use crate::views::{ArcaneBackgroundChoiceOptionView, ArcaneBackgroundChoiceType};

/// View model for an arcane background choice with its options.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct ArcaneBackgroundChoiceView {
    pub id: i64,
    pub choice_type: ArcaneBackgroundChoiceType,
    pub choice_category: Option<String>,
    pub min_selections: i64,
    pub max_selections: i64,
    pub description: String,
    pub position: i64,
    pub options: Vec<ArcaneBackgroundChoiceOptionView>,
}

impl ArcaneBackgroundChoiceView {
    pub fn new(choice: ArcaneBackgroundChoice, options: Vec<ArcaneBackgroundChoiceOptionView>) -> Self {
        let choice_type = ArcaneBackgroundChoiceType::from_str(&choice.choice_type)
            .unwrap_or(ArcaneBackgroundChoiceType::AvailablePower);

        Self {
            id: choice.id,
            choice_type,
            choice_category: choice.choice_category,
            min_selections: choice.min_selections,
            max_selections: choice.max_selections,
            description: choice.description,
            position: choice.position,
            options,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::ArcaneBackgroundChoiceOption;
    use crate::views::ArcaneBackgroundOptionType;

    fn create_test_choice() -> ArcaneBackgroundChoice {
        ArcaneBackgroundChoice {
            id: 1,
            arcane_background_id: 1,
            choice_type: "available_power".to_string(),
            choice_category: None,
            min_selections: 0,
            max_selections: 55,
            description: "Powers available to the Wizard".to_string(),
            position: 0,
            created_at: "2024-01-01".to_string(),
            updated_at: "2024-01-01".to_string(),
        }
    }

    fn create_test_option_view() -> ArcaneBackgroundChoiceOptionView {
        ArcaneBackgroundChoiceOptionView::new(ArcaneBackgroundChoiceOption {
            id: 1,
            choice_id: 1,
            option_type: "power".to_string(),
            option_id: Some(1),
            option_description: Some("Bolt".to_string()),
            position: 0,
            created_at: "2024-01-01".to_string(),
            updated_at: "2024-01-01".to_string(),
        })
    }

    #[test]
    fn new_creates_view() {
        let choice = create_test_choice();
        let options = vec![create_test_option_view()];

        let view = ArcaneBackgroundChoiceView::new(choice, options);

        assert_eq!(view.id, 1);
        assert_eq!(view.choice_type, ArcaneBackgroundChoiceType::AvailablePower);
        assert_eq!(view.choice_category, None);
        assert_eq!(view.min_selections, 0);
        assert_eq!(view.max_selections, 55);
        assert_eq!(view.description, "Powers available to the Wizard");
        assert_eq!(view.position, 0);
        assert_eq!(view.options.len(), 1);
    }

    #[test]
    fn new_with_built_in_hindrance_type() {
        let mut choice = create_test_choice();
        choice.choice_type = "built_in_hindrance".to_string();
        choice.min_selections = 1;
        choice.max_selections = 1;

        let view = ArcaneBackgroundChoiceView::new(choice, vec![]);

        assert_eq!(view.choice_type, ArcaneBackgroundChoiceType::BuiltInHindrance);
    }

    #[test]
    fn new_with_special_ability_type() {
        let mut choice = create_test_choice();
        choice.choice_type = "special_ability".to_string();
        choice.min_selections = 0;
        choice.max_selections = 0;

        let view = ArcaneBackgroundChoiceView::new(choice, vec![]);

        assert_eq!(view.choice_type, ArcaneBackgroundChoiceType::SpecialAbility);
    }

    #[test]
    fn new_with_category() {
        let mut choice = create_test_choice();
        choice.choice_category = Some("domain".to_string());

        let view = ArcaneBackgroundChoiceView::new(choice, vec![]);

        assert_eq!(view.choice_category, Some("domain".to_string()));
    }

    #[test]
    fn new_with_multiple_options() {
        let choice = create_test_choice();
        let options = vec![
            create_test_option_view(),
            ArcaneBackgroundChoiceOptionView {
                id: 2,
                option_type: ArcaneBackgroundOptionType::Power,
                option_id: Some(2),
                description: Some("Blast".to_string()),
                position: 1,
            },
        ];

        let view = ArcaneBackgroundChoiceView::new(choice, options);

        assert_eq!(view.options.len(), 2);
    }

    #[test]
    fn new_with_no_options() {
        let choice = create_test_choice();

        let view = ArcaneBackgroundChoiceView::new(choice, vec![]);

        assert!(view.options.is_empty());
    }
}
