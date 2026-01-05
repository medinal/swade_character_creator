use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

/// Types of arcane background choice options.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "snake_case")]
pub enum ArcaneBackgroundOptionType {
    Power,
    Hindrance,
    Ability,
    EdgeCategory,
}

impl FromStr for ArcaneBackgroundOptionType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "power" => Ok(ArcaneBackgroundOptionType::Power),
            "hindrance" => Ok(ArcaneBackgroundOptionType::Hindrance),
            "ability" => Ok(ArcaneBackgroundOptionType::Ability),
            "edge_category" => Ok(ArcaneBackgroundOptionType::EdgeCategory),
            _ => Err(format!("Invalid arcane background option type: {}", s)),
        }
    }
}

impl fmt::Display for ArcaneBackgroundOptionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ArcaneBackgroundOptionType::Power => write!(f, "power"),
            ArcaneBackgroundOptionType::Hindrance => write!(f, "hindrance"),
            ArcaneBackgroundOptionType::Ability => write!(f, "ability"),
            ArcaneBackgroundOptionType::EdgeCategory => write!(f, "edge_category"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str_all_variants() {
        assert_eq!(
            ArcaneBackgroundOptionType::from_str("power").unwrap(),
            ArcaneBackgroundOptionType::Power
        );
        assert_eq!(
            ArcaneBackgroundOptionType::from_str("hindrance").unwrap(),
            ArcaneBackgroundOptionType::Hindrance
        );
        assert_eq!(
            ArcaneBackgroundOptionType::from_str("ability").unwrap(),
            ArcaneBackgroundOptionType::Ability
        );
        assert_eq!(
            ArcaneBackgroundOptionType::from_str("edge_category").unwrap(),
            ArcaneBackgroundOptionType::EdgeCategory
        );
    }

    #[test]
    fn from_str_case_insensitive() {
        assert_eq!(
            ArcaneBackgroundOptionType::from_str("POWER").unwrap(),
            ArcaneBackgroundOptionType::Power
        );
        assert_eq!(
            ArcaneBackgroundOptionType::from_str("Hindrance").unwrap(),
            ArcaneBackgroundOptionType::Hindrance
        );
    }

    #[test]
    fn from_str_invalid() {
        assert!(ArcaneBackgroundOptionType::from_str("invalid").is_err());
    }

    #[test]
    fn display() {
        assert_eq!(ArcaneBackgroundOptionType::Power.to_string(), "power");
        assert_eq!(ArcaneBackgroundOptionType::Hindrance.to_string(), "hindrance");
        assert_eq!(ArcaneBackgroundOptionType::Ability.to_string(), "ability");
        assert_eq!(ArcaneBackgroundOptionType::EdgeCategory.to_string(), "edge_category");
    }

    #[test]
    fn serde_roundtrip() {
        let value = ArcaneBackgroundOptionType::Hindrance;
        let json = serde_json::to_string(&value).unwrap();
        assert_eq!(json, "\"hindrance\"");
        let parsed: ArcaneBackgroundOptionType = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, value);
    }
}
