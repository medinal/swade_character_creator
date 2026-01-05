use serde::{Deserialize, Serialize};
use std::str::FromStr;

use crate::models::AncestryChoiceOption;
use crate::views::AncestryOptionType;

/// View model for an ancestry choice option.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct AncestryChoiceOptionView {
    pub id: i64,
    pub option_type: AncestryOptionType,
    pub option_id: Option<i64>,
    pub description: Option<String>,
}

impl AncestryChoiceOptionView {
    pub fn new(option: AncestryChoiceOption) -> Self {
        let option_type =
            AncestryOptionType::from_str(&option.option_type).unwrap_or(AncestryOptionType::Edge);

        Self {
            id: option.id,
            option_type,
            option_id: option.option_id,
            description: option.option_description,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_option() -> AncestryChoiceOption {
        AncestryChoiceOption {
            id: 1,
            choice_id: 1,
            option_type: "edge".to_string(),
            option_id: Some(42),
            option_description: Some("Alertness".to_string()),
            created_at: "2024-01-01".to_string(),
            updated_at: "2024-01-01".to_string(),
        }
    }

    #[test]
    fn new_creates_view() {
        let option = create_test_option();
        let view = AncestryChoiceOptionView::new(option);

        assert_eq!(view.id, 1);
        assert_eq!(view.option_type, AncestryOptionType::Edge);
        assert_eq!(view.option_id, Some(42));
        assert_eq!(view.description, Some("Alertness".to_string()));
    }

    #[test]
    fn new_with_hindrance_type() {
        let mut option = create_test_option();
        option.option_type = "hindrance".to_string();

        let view = AncestryChoiceOptionView::new(option);

        assert_eq!(view.option_type, AncestryOptionType::Hindrance);
    }

    #[test]
    fn new_with_ancestry_type() {
        let mut option = create_test_option();
        option.option_type = "ancestry".to_string();

        let view = AncestryChoiceOptionView::new(option);

        assert_eq!(view.option_type, AncestryOptionType::Ancestry);
    }

    #[test]
    fn new_with_null_values() {
        let option = AncestryChoiceOption {
            id: 1,
            choice_id: 1,
            option_type: "edge".to_string(),
            option_id: None,
            option_description: None,
            created_at: "2024-01-01".to_string(),
            updated_at: "2024-01-01".to_string(),
        };

        let view = AncestryChoiceOptionView::new(option);

        assert_eq!(view.option_id, None);
        assert_eq!(view.description, None);
    }
}
