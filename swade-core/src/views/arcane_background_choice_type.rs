use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

/// Types of arcane background choices.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "snake_case")]
pub enum ArcaneBackgroundChoiceType {
    AvailablePower,
    RequiredStartingPower,
    ChoosableStartingPower,
    BuiltInHindrance,
    SpecialAbility,
    EdgeCategory,
}

impl FromStr for ArcaneBackgroundChoiceType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "available_power" => Ok(ArcaneBackgroundChoiceType::AvailablePower),
            "required_starting_power" => Ok(ArcaneBackgroundChoiceType::RequiredStartingPower),
            "choosable_starting_power" => Ok(ArcaneBackgroundChoiceType::ChoosableStartingPower),
            "built_in_hindrance" => Ok(ArcaneBackgroundChoiceType::BuiltInHindrance),
            "special_ability" => Ok(ArcaneBackgroundChoiceType::SpecialAbility),
            "edge_category" => Ok(ArcaneBackgroundChoiceType::EdgeCategory),
            _ => Err(format!("Invalid arcane background choice type: {}", s)),
        }
    }
}

impl fmt::Display for ArcaneBackgroundChoiceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ArcaneBackgroundChoiceType::AvailablePower => write!(f, "available_power"),
            ArcaneBackgroundChoiceType::RequiredStartingPower => write!(f, "required_starting_power"),
            ArcaneBackgroundChoiceType::ChoosableStartingPower => write!(f, "choosable_starting_power"),
            ArcaneBackgroundChoiceType::BuiltInHindrance => write!(f, "built_in_hindrance"),
            ArcaneBackgroundChoiceType::SpecialAbility => write!(f, "special_ability"),
            ArcaneBackgroundChoiceType::EdgeCategory => write!(f, "edge_category"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str_all_variants() {
        assert_eq!(
            ArcaneBackgroundChoiceType::from_str("available_power").unwrap(),
            ArcaneBackgroundChoiceType::AvailablePower
        );
        assert_eq!(
            ArcaneBackgroundChoiceType::from_str("required_starting_power").unwrap(),
            ArcaneBackgroundChoiceType::RequiredStartingPower
        );
        assert_eq!(
            ArcaneBackgroundChoiceType::from_str("choosable_starting_power").unwrap(),
            ArcaneBackgroundChoiceType::ChoosableStartingPower
        );
        assert_eq!(
            ArcaneBackgroundChoiceType::from_str("built_in_hindrance").unwrap(),
            ArcaneBackgroundChoiceType::BuiltInHindrance
        );
        assert_eq!(
            ArcaneBackgroundChoiceType::from_str("special_ability").unwrap(),
            ArcaneBackgroundChoiceType::SpecialAbility
        );
        assert_eq!(
            ArcaneBackgroundChoiceType::from_str("edge_category").unwrap(),
            ArcaneBackgroundChoiceType::EdgeCategory
        );
    }

    #[test]
    fn from_str_case_insensitive() {
        assert_eq!(
            ArcaneBackgroundChoiceType::from_str("AVAILABLE_POWER").unwrap(),
            ArcaneBackgroundChoiceType::AvailablePower
        );
        assert_eq!(
            ArcaneBackgroundChoiceType::from_str("Built_In_Hindrance").unwrap(),
            ArcaneBackgroundChoiceType::BuiltInHindrance
        );
    }

    #[test]
    fn from_str_invalid() {
        assert!(ArcaneBackgroundChoiceType::from_str("invalid").is_err());
    }

    #[test]
    fn display() {
        assert_eq!(ArcaneBackgroundChoiceType::AvailablePower.to_string(), "available_power");
        assert_eq!(ArcaneBackgroundChoiceType::RequiredStartingPower.to_string(), "required_starting_power");
        assert_eq!(ArcaneBackgroundChoiceType::ChoosableStartingPower.to_string(), "choosable_starting_power");
        assert_eq!(ArcaneBackgroundChoiceType::BuiltInHindrance.to_string(), "built_in_hindrance");
        assert_eq!(ArcaneBackgroundChoiceType::SpecialAbility.to_string(), "special_ability");
        assert_eq!(ArcaneBackgroundChoiceType::EdgeCategory.to_string(), "edge_category");
    }

    #[test]
    fn serde_roundtrip() {
        let value = ArcaneBackgroundChoiceType::BuiltInHindrance;
        let json = serde_json::to_string(&value).unwrap();
        assert_eq!(json, "\"built_in_hindrance\"");
        let parsed: ArcaneBackgroundChoiceType = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, value);
    }
}
