use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

/// Edge category types in SWADE.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, specta::Type)]
pub enum EdgeCategory {
    Background,
    Combat,
    Leadership,
    Power,
    Professional,
    Social,
    Weird,
}

impl FromStr for EdgeCategory {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "background" => Ok(EdgeCategory::Background),
            "combat" => Ok(EdgeCategory::Combat),
            "leadership" => Ok(EdgeCategory::Leadership),
            "power" => Ok(EdgeCategory::Power),
            "professional" => Ok(EdgeCategory::Professional),
            "social" => Ok(EdgeCategory::Social),
            "weird" => Ok(EdgeCategory::Weird),
            _ => Err(format!("Invalid edge category: {}", s)),
        }
    }
}

impl fmt::Display for EdgeCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EdgeCategory::Background => write!(f, "Background"),
            EdgeCategory::Combat => write!(f, "Combat"),
            EdgeCategory::Leadership => write!(f, "Leadership"),
            EdgeCategory::Power => write!(f, "Power"),
            EdgeCategory::Professional => write!(f, "Professional"),
            EdgeCategory::Social => write!(f, "Social"),
            EdgeCategory::Weird => write!(f, "Weird"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str_all_variants() {
        assert_eq!(
            EdgeCategory::from_str("Background").unwrap(),
            EdgeCategory::Background
        );
        assert_eq!(
            EdgeCategory::from_str("combat").unwrap(),
            EdgeCategory::Combat
        );
        assert_eq!(
            EdgeCategory::from_str("LEADERSHIP").unwrap(),
            EdgeCategory::Leadership
        );
        assert_eq!(
            EdgeCategory::from_str("Power").unwrap(),
            EdgeCategory::Power
        );
        assert_eq!(
            EdgeCategory::from_str("professional").unwrap(),
            EdgeCategory::Professional
        );
        assert_eq!(
            EdgeCategory::from_str("Social").unwrap(),
            EdgeCategory::Social
        );
        assert_eq!(
            EdgeCategory::from_str("weird").unwrap(),
            EdgeCategory::Weird
        );
    }

    #[test]
    fn from_str_invalid() {
        assert!(EdgeCategory::from_str("invalid").is_err());
    }

    #[test]
    fn display() {
        assert_eq!(EdgeCategory::Background.to_string(), "Background");
        assert_eq!(EdgeCategory::Combat.to_string(), "Combat");
    }

    #[test]
    fn serde_roundtrip() {
        let cat = EdgeCategory::Combat;
        let json = serde_json::to_string(&cat).unwrap();
        assert_eq!(json, "\"Combat\"");
        let parsed: EdgeCategory = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, cat);
    }
}
