use serde::{Deserialize, Serialize};
use std::str::FromStr;

use crate::models::ArcaneBackgroundChoiceOption;
use crate::views::ArcaneBackgroundOptionType;

/// View model for an arcane background choice option.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct ArcaneBackgroundChoiceOptionView {
    pub id: i64,
    pub option_type: ArcaneBackgroundOptionType,
    pub option_id: Option<i64>,
    pub description: Option<String>,
    pub position: i64,
}

impl ArcaneBackgroundChoiceOptionView {
    pub fn new(option: ArcaneBackgroundChoiceOption) -> Self {
        let option_type =
            ArcaneBackgroundOptionType::from_str(&option.option_type).unwrap_or(ArcaneBackgroundOptionType::Power);

        Self {
            id: option.id,
            option_type,
            option_id: option.option_id,
            description: option.option_description,
            position: option.position,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_option() -> ArcaneBackgroundChoiceOption {
        ArcaneBackgroundChoiceOption {
            id: 1,
            choice_id: 1,
            option_type: "power".to_string(),
            option_id: Some(42),
            option_description: Some("Bolt".to_string()),
            position: 0,
            created_at: "2024-01-01".to_string(),
            updated_at: "2024-01-01".to_string(),
        }
    }

    #[test]
    fn new_creates_view() {
        let option = create_test_option();
        let view = ArcaneBackgroundChoiceOptionView::new(option);

        assert_eq!(view.id, 1);
        assert_eq!(view.option_type, ArcaneBackgroundOptionType::Power);
        assert_eq!(view.option_id, Some(42));
        assert_eq!(view.description, Some("Bolt".to_string()));
        assert_eq!(view.position, 0);
    }

    #[test]
    fn new_with_hindrance_type() {
        let mut option = create_test_option();
        option.option_type = "hindrance".to_string();

        let view = ArcaneBackgroundChoiceOptionView::new(option);

        assert_eq!(view.option_type, ArcaneBackgroundOptionType::Hindrance);
    }

    #[test]
    fn new_with_ability_type() {
        let mut option = create_test_option();
        option.option_type = "ability".to_string();
        option.option_id = None;
        option.option_description = Some("Can take Magic edges".to_string());

        let view = ArcaneBackgroundChoiceOptionView::new(option);

        assert_eq!(view.option_type, ArcaneBackgroundOptionType::Ability);
        assert_eq!(view.option_id, None);
        assert_eq!(view.description, Some("Can take Magic edges".to_string()));
    }

    #[test]
    fn new_with_null_values() {
        let option = ArcaneBackgroundChoiceOption {
            id: 1,
            choice_id: 1,
            option_type: "ability".to_string(),
            option_id: None,
            option_description: None,
            position: 0,
            created_at: "2024-01-01".to_string(),
            updated_at: "2024-01-01".to_string(),
        };

        let view = ArcaneBackgroundChoiceOptionView::new(option);

        assert_eq!(view.option_id, None);
        assert_eq!(view.description, None);
    }
}
