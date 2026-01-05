use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

/// Types of ancestry choice options.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "snake_case")]
pub enum AncestryOptionType {
    Edge,
    Hindrance,
    Ancestry,
}

impl FromStr for AncestryOptionType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "edge" => Ok(AncestryOptionType::Edge),
            "hindrance" => Ok(AncestryOptionType::Hindrance),
            "ancestry" => Ok(AncestryOptionType::Ancestry),
            _ => Err(format!("Invalid ancestry option type: {}", s)),
        }
    }
}

impl fmt::Display for AncestryOptionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AncestryOptionType::Edge => write!(f, "edge"),
            AncestryOptionType::Hindrance => write!(f, "hindrance"),
            AncestryOptionType::Ancestry => write!(f, "ancestry"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str_all_variants() {
        assert_eq!(
            AncestryOptionType::from_str("edge").unwrap(),
            AncestryOptionType::Edge
        );
        assert_eq!(
            AncestryOptionType::from_str("hindrance").unwrap(),
            AncestryOptionType::Hindrance
        );
        assert_eq!(
            AncestryOptionType::from_str("ancestry").unwrap(),
            AncestryOptionType::Ancestry
        );
    }

    #[test]
    fn from_str_case_insensitive() {
        assert_eq!(
            AncestryOptionType::from_str("EDGE").unwrap(),
            AncestryOptionType::Edge
        );
        assert_eq!(
            AncestryOptionType::from_str("Hindrance").unwrap(),
            AncestryOptionType::Hindrance
        );
    }

    #[test]
    fn from_str_invalid() {
        assert!(AncestryOptionType::from_str("invalid").is_err());
    }

    #[test]
    fn display() {
        assert_eq!(AncestryOptionType::Edge.to_string(), "edge");
        assert_eq!(AncestryOptionType::Hindrance.to_string(), "hindrance");
        assert_eq!(AncestryOptionType::Ancestry.to_string(), "ancestry");
    }

    #[test]
    fn serde_roundtrip() {
        let value = AncestryOptionType::Edge;
        let json = serde_json::to_string(&value).unwrap();
        assert_eq!(json, "\"edge\"");
        let parsed: AncestryOptionType = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, value);
    }
}
