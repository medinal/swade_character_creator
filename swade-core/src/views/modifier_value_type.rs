use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

/// Types of modifier values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "snake_case")]
pub enum ModifierValueType {
    DieIncrement,
    RollBonus,
    FlatBonus,
    Description,
    BonusSelection,
    MandatorySelection,
}

impl FromStr for ModifierValueType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "die_increment" => Ok(ModifierValueType::DieIncrement),
            "roll_bonus" => Ok(ModifierValueType::RollBonus),
            "flat_bonus" => Ok(ModifierValueType::FlatBonus),
            "description" => Ok(ModifierValueType::Description),
            "bonus_selection" => Ok(ModifierValueType::BonusSelection),
            "mandatory_selection" => Ok(ModifierValueType::MandatorySelection),
            _ => Err(format!("Invalid modifier value type: {}", s)),
        }
    }
}

impl fmt::Display for ModifierValueType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ModifierValueType::DieIncrement => write!(f, "die_increment"),
            ModifierValueType::RollBonus => write!(f, "roll_bonus"),
            ModifierValueType::FlatBonus => write!(f, "flat_bonus"),
            ModifierValueType::Description => write!(f, "description"),
            ModifierValueType::BonusSelection => write!(f, "bonus_selection"),
            ModifierValueType::MandatorySelection => write!(f, "mandatory_selection"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str_all_variants() {
        assert_eq!(
            ModifierValueType::from_str("die_increment").unwrap(),
            ModifierValueType::DieIncrement
        );
        assert_eq!(
            ModifierValueType::from_str("roll_bonus").unwrap(),
            ModifierValueType::RollBonus
        );
        assert_eq!(
            ModifierValueType::from_str("flat_bonus").unwrap(),
            ModifierValueType::FlatBonus
        );
        assert_eq!(
            ModifierValueType::from_str("description").unwrap(),
            ModifierValueType::Description
        );
        assert_eq!(
            ModifierValueType::from_str("bonus_selection").unwrap(),
            ModifierValueType::BonusSelection
        );
        assert_eq!(
            ModifierValueType::from_str("mandatory_selection").unwrap(),
            ModifierValueType::MandatorySelection
        );
    }

    #[test]
    fn from_str_case_insensitive() {
        assert_eq!(
            ModifierValueType::from_str("DIE_INCREMENT").unwrap(),
            ModifierValueType::DieIncrement
        );
        assert_eq!(
            ModifierValueType::from_str("Roll_Bonus").unwrap(),
            ModifierValueType::RollBonus
        );
    }

    #[test]
    fn from_str_invalid() {
        assert!(ModifierValueType::from_str("invalid").is_err());
    }

    #[test]
    fn display() {
        assert_eq!(ModifierValueType::DieIncrement.to_string(), "die_increment");
        assert_eq!(ModifierValueType::RollBonus.to_string(), "roll_bonus");
        assert_eq!(ModifierValueType::FlatBonus.to_string(), "flat_bonus");
    }

    #[test]
    fn serde_roundtrip() {
        let value = ModifierValueType::RollBonus;
        let json = serde_json::to_string(&value).unwrap();
        assert_eq!(json, "\"roll_bonus\"");
        let parsed: ModifierValueType = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, value);
    }
}
