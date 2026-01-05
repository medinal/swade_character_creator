use serde::{Deserialize, Serialize};
use std::str::FromStr;

use crate::models::{Edge, Modifier};
use crate::views::{EdgeCategory, RequirementTree};

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct EdgeView {
    pub id: i64,
    pub name: String,
    pub category: EdgeCategory,
    pub source: String,
    pub description: String,
    pub can_take_multiple_times: bool,
    pub modifiers: Vec<Modifier>,
    pub requirements: RequirementTree,
}

impl EdgeView {
    pub fn new(edge: Edge, modifiers: Vec<Modifier>, requirements: RequirementTree) -> Self {
        let category = EdgeCategory::from_str(&edge.background).unwrap_or(EdgeCategory::Background);

        Self {
            id: edge.id,
            name: edge.name,
            category,
            source: edge.source,
            description: edge.description,
            can_take_multiple_times: edge.can_take_multiple_times,
            modifiers,
            requirements,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Requirement;

    fn create_test_edge() -> Edge {
        Edge {
            id: 1,
            name: "Alertness".to_string(),
            background: "Background".to_string(),
            source: "core".to_string(),
            description: "+2 to Notice rolls".to_string(),
            can_take_multiple_times: false,
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
            value: Some(2),
            description: "+2 to Notice rolls".to_string(),
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
    fn new_creates_edge_view() {
        let edge = create_test_edge();
        let modifiers = vec![create_test_modifier()];
        let requirements = RequirementTree::leaf(create_test_requirement());

        let view = EdgeView::new(edge, modifiers, requirements);

        assert_eq!(view.id, 1);
        assert_eq!(view.name, "Alertness");
        assert_eq!(view.category, EdgeCategory::Background);
        assert_eq!(view.source, "core");
        assert_eq!(view.description, "+2 to Notice rolls");
        assert_eq!(view.can_take_multiple_times, false);
        assert_eq!(view.modifiers.len(), 1);
        assert!(!view.requirements.is_empty());
    }

    #[test]
    fn new_with_no_modifiers() {
        let edge = create_test_edge();
        let view = EdgeView::new(edge, vec![], RequirementTree::none());

        assert_eq!(view.modifiers.len(), 0);
        assert!(view.requirements.is_empty());
    }

    #[test]
    fn new_with_multiple_modifiers() {
        let edge = create_test_edge();
        let modifiers = vec![create_test_modifier(), create_test_modifier()];
        let view = EdgeView::new(edge, modifiers, RequirementTree::none());

        assert_eq!(view.modifiers.len(), 2);
    }
}
