use serde::{Deserialize, Serialize};

use crate::views::{AttributeView, Die};

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct CharacterAttributeValue {
    pub attribute: AttributeView,
    /// The purchased die value (what the player spent points on).
    pub die: Die,
    /// The effective die value (purchased + modifiers from ancestry/edges/etc).
    pub effective_die: Die,
    /// The effective base die (d4 + modifiers) - the starting point with modifiers.
    pub base_die: Die,
    /// The effective max die (d12 + modifiers) - the ceiling with modifiers.
    pub max_die: Die,
    /// Whether this attribute can be incremented (has points, not at max).
    pub can_increment: bool,
    /// Whether this attribute can be decremented (not at base).
    pub can_decrement: bool,
}

impl CharacterAttributeValue {
    pub fn new(attribute: AttributeView, die: Die) -> Self {
        // When no modifiers, effective values equal purchased/base values
        Self {
            base_die: attribute.base_die,
            max_die: Die::d12(),
            attribute,
            die,
            effective_die: die,
            can_increment: false,
            can_decrement: false,
        }
    }

    /// Create with explicit effective values (when modifiers are applied).
    pub fn with_effective(
        attribute: AttributeView,
        die: Die,
        effective_die: Die,
        base_die: Die,
        max_die: Die,
        can_increment: bool,
        can_decrement: bool,
    ) -> Self {
        Self {
            attribute,
            die,
            effective_die,
            base_die,
            max_die,
            can_increment,
            can_decrement,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Attribute;

    fn create_test_attribute() -> Attribute {
        Attribute {
            id: 1,
            name: "Agility".to_string(),
            description: "Nimbleness and dexterity".to_string(),
            base_value: 4,
            created_at: "2024-01-01".to_string(),
            updated_at: "2024-01-01".to_string(),
        }
    }

    #[test]
    fn new_creates_value_with_attribute_and_die() {
        let attribute_view = AttributeView::new(create_test_attribute());
        let die = Die::d8();

        let value = CharacterAttributeValue::new(attribute_view.clone(), die);

        assert_eq!(value.attribute.id, 1);
        assert_eq!(value.attribute.name, "Agility");
        assert_eq!(value.die, Die::d8());
        // Without modifiers, effective equals purchased
        assert_eq!(value.effective_die, Die::d8());
        assert_eq!(value.base_die, Die::d4());
        assert_eq!(value.max_die, Die::d12());
    }

    #[test]
    fn with_effective_creates_value_with_modifiers() {
        let attribute_view = AttributeView::new(create_test_attribute());
        let purchased = Die::d4();
        let effective = Die::d6(); // +1 from modifier
        let base = Die::d6(); // d4 + 1 step
        let max = Die::with_modifier(12, 1).unwrap(); // d12+1

        let value = CharacterAttributeValue::with_effective(
            attribute_view.clone(),
            purchased,
            effective,
            base,
            max,
            true,  // can_increment
            false, // can_decrement (at base)
        );

        assert_eq!(value.die, Die::d4());
        assert_eq!(value.effective_die, Die::d6());
        assert_eq!(value.base_die, Die::d6());
        assert_eq!(value.max_die, Die::with_modifier(12, 1).unwrap());
        assert!(value.can_increment);
        assert!(!value.can_decrement);
    }
}
