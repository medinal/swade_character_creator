use serde::{Deserialize, Serialize};
use std::str::FromStr;

use crate::models::{Hindrance, Modifier};
use crate::views::{RequirementTree, Severity};

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct HindranceView {
    pub id: i64,
    pub name: String,
    pub severity: Severity,
    pub point_value: i64,
    pub companion_hindrance_id: Option<i64>,
    pub source: String,
    pub description: String,
    pub modifiers: Vec<Modifier>,
    pub requirements: RequirementTree,
}

impl HindranceView {
    pub fn new(
        hindrance: Hindrance,
        modifiers: Vec<Modifier>,
        requirements: RequirementTree,
    ) -> Self {
        let severity = Severity::from_str(&hindrance.severity).unwrap_or(Severity::Minor);

        Self {
            id: hindrance.id,
            name: hindrance.name,
            severity,
            point_value: hindrance.point_value,
            companion_hindrance_id: hindrance.companion_hindrance_id,
            source: hindrance.source,
            description: hindrance.description,
            modifiers,
            requirements,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Requirement;

    fn create_test_hindrance() -> Hindrance {
        Hindrance {
            id: 1,
            name: "Blind".to_string(),
            severity: "Major".to_string(),
            point_value: 2,
            companion_hindrance_id: None,
            source: "core".to_string(),
            description: "The character is completely blind.".to_string(),
            created_at: "2024-01-01".to_string(),
            updated_at: "2024-01-01".to_string(),
        }
    }

    fn create_test_modifier() -> Modifier {
        Modifier {
            id: 1,
            target_type: Some("skill".to_string()),
            target_identifier: Some("Notice".to_string()),
            value_type: "roll_bonus".to_string(),
            value: Some(-2),
            description: "-2 to Notice rolls".to_string(),
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
    fn new_creates_hindrance_view() {
        let hindrance = create_test_hindrance();
        let modifiers = vec![create_test_modifier()];
        let requirements = RequirementTree::leaf(create_test_requirement());

        let view = HindranceView::new(hindrance, modifiers, requirements);

        assert_eq!(view.id, 1);
        assert_eq!(view.name, "Blind");
        assert_eq!(view.severity, Severity::Major);
        assert_eq!(view.point_value, 2);
        assert_eq!(view.companion_hindrance_id, None);
        assert_eq!(view.source, "core");
        assert_eq!(view.modifiers.len(), 1);
        assert!(!view.requirements.is_empty());
    }

    #[test]
    fn new_with_no_modifiers() {
        let hindrance = create_test_hindrance();
        let view = HindranceView::new(hindrance, vec![], RequirementTree::none());

        assert_eq!(view.modifiers.len(), 0);
        assert!(view.requirements.is_empty());
    }

    #[test]
    fn new_with_companion_hindrance() {
        let mut hindrance = create_test_hindrance();
        hindrance.name = "One Arm".to_string();
        hindrance.companion_hindrance_id = Some(5);

        let view = HindranceView::new(hindrance, vec![], RequirementTree::none());

        assert_eq!(view.companion_hindrance_id, Some(5));
    }
}
