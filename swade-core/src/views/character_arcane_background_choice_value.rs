use serde::{Deserialize, Serialize};

use crate::views::{ArcaneBackgroundChoiceOptionView, ArcaneBackgroundChoiceView};

/// Represents a character's selection for an arcane background choice.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct CharacterArcaneBackgroundChoiceValue {
    pub choice: ArcaneBackgroundChoiceView,
    pub selected_options: Vec<ArcaneBackgroundChoiceOptionView>,
}

impl CharacterArcaneBackgroundChoiceValue {
    pub fn new(
        choice: ArcaneBackgroundChoiceView,
        selected_options: Vec<ArcaneBackgroundChoiceOptionView>,
    ) -> Self {
        Self {
            choice,
            selected_options,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{ArcaneBackgroundChoice, ArcaneBackgroundChoiceOption};
    use crate::views::ArcaneBackgroundOptionType;

    fn create_test_choice_view() -> ArcaneBackgroundChoiceView {
        ArcaneBackgroundChoiceView::new(
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
            },
            vec![],
        )
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
    fn new_creates_value() {
        let choice = create_test_choice_view();
        let options = vec![create_test_option_view()];

        let value = CharacterArcaneBackgroundChoiceValue::new(choice, options);

        assert_eq!(value.choice.id, 1);
        assert_eq!(value.selected_options.len(), 1);
    }

    #[test]
    fn new_with_no_selections() {
        let choice = create_test_choice_view();

        let value = CharacterArcaneBackgroundChoiceValue::new(choice, vec![]);

        assert!(value.selected_options.is_empty());
    }

    #[test]
    fn new_with_multiple_selections() {
        let choice = create_test_choice_view();
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

        let value = CharacterArcaneBackgroundChoiceValue::new(choice, options);

        assert_eq!(value.selected_options.len(), 2);
    }
}
