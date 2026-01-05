use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

/// Types of ancestry choices.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "snake_case")]
pub enum AncestryChoiceType {
    FreeEdge,
    MandatoryHindrance,
    AncestralEnemy,
}

impl FromStr for AncestryChoiceType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "free_edge" => Ok(AncestryChoiceType::FreeEdge),
            "mandatory_hindrance" => Ok(AncestryChoiceType::MandatoryHindrance),
            "ancestral_enemy" => Ok(AncestryChoiceType::AncestralEnemy),
            _ => Err(format!("Invalid ancestry choice type: {}", s)),
        }
    }
}

impl fmt::Display for AncestryChoiceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AncestryChoiceType::FreeEdge => write!(f, "free_edge"),
            AncestryChoiceType::MandatoryHindrance => write!(f, "mandatory_hindrance"),
            AncestryChoiceType::AncestralEnemy => write!(f, "ancestral_enemy"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str_all_variants() {
        assert_eq!(
            AncestryChoiceType::from_str("free_edge").unwrap(),
            AncestryChoiceType::FreeEdge
        );
        assert_eq!(
            AncestryChoiceType::from_str("mandatory_hindrance").unwrap(),
            AncestryChoiceType::MandatoryHindrance
        );
        assert_eq!(
            AncestryChoiceType::from_str("ancestral_enemy").unwrap(),
            AncestryChoiceType::AncestralEnemy
        );
    }

    #[test]
    fn from_str_case_insensitive() {
        assert_eq!(
            AncestryChoiceType::from_str("FREE_EDGE").unwrap(),
            AncestryChoiceType::FreeEdge
        );
        assert_eq!(
            AncestryChoiceType::from_str("Mandatory_Hindrance").unwrap(),
            AncestryChoiceType::MandatoryHindrance
        );
    }

    #[test]
    fn from_str_invalid() {
        assert!(AncestryChoiceType::from_str("invalid").is_err());
    }

    #[test]
    fn display() {
        assert_eq!(AncestryChoiceType::FreeEdge.to_string(), "free_edge");
        assert_eq!(
            AncestryChoiceType::MandatoryHindrance.to_string(),
            "mandatory_hindrance"
        );
        assert_eq!(
            AncestryChoiceType::AncestralEnemy.to_string(),
            "ancestral_enemy"
        );
    }

    #[test]
    fn serde_roundtrip() {
        let value = AncestryChoiceType::FreeEdge;
        let json = serde_json::to_string(&value).unwrap();
        assert_eq!(json, "\"free_edge\"");
        let parsed: AncestryChoiceType = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, value);
    }
}
