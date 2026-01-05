use serde::{Deserialize, Serialize};

use crate::models::Attribute;
use crate::views::Die;

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct AttributeView {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub base_die: Die,
}

impl AttributeView {
    pub fn new(attribute: Attribute) -> Self {
        Self {
            id: attribute.id,
            name: attribute.name,
            description: attribute.description,
            base_die: Die::new(attribute.base_value as u8).unwrap_or(Die::d4()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_attribute(base_value: i64) -> Attribute {
        Attribute {
            id: 1,
            name: "Agility".to_string(),
            description: "Nimbleness and dexterity".to_string(),
            base_value,
            created_at: "2024-01-01".to_string(),
            updated_at: "2024-01-01".to_string(),
        }
    }

    #[test]
    fn new_converts_base_value_to_die() {
        let attribute = create_test_attribute(4);
        let view = AttributeView::new(attribute);

        assert_eq!(view.id, 1);
        assert_eq!(view.name, "Agility");
        assert_eq!(view.description, "Nimbleness and dexterity");
        assert_eq!(view.base_die, Die::d4());
    }

    #[test]
    fn new_handles_different_base_values() {
        let view = AttributeView::new(create_test_attribute(6));
        assert_eq!(view.base_die, Die::d6());

        let view = AttributeView::new(create_test_attribute(8));
        assert_eq!(view.base_die, Die::d8());
    }

    #[test]
    fn new_defaults_to_d4_for_invalid_base_value() {
        let view = AttributeView::new(create_test_attribute(5));
        assert_eq!(view.base_die, Die::d4());
    }
}
