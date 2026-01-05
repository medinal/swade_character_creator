use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

/// Hindrance severity level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    Minor,
    Major,
}

impl Severity {
    pub fn point_value(&self) -> i64 {
        match self {
            Severity::Minor => 1,
            Severity::Major => 2,
        }
    }
}

impl FromStr for Severity {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "minor" => Ok(Severity::Minor),
            "major" => Ok(Severity::Major),
            _ => Err(format!("Invalid severity: {}", s)),
        }
    }
}

impl fmt::Display for Severity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Severity::Minor => write!(f, "minor"),
            Severity::Major => write!(f, "major"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str_minor() {
        assert_eq!(Severity::from_str("minor").unwrap(), Severity::Minor);
        assert_eq!(Severity::from_str("Minor").unwrap(), Severity::Minor);
        assert_eq!(Severity::from_str("MINOR").unwrap(), Severity::Minor);
    }

    #[test]
    fn from_str_major() {
        assert_eq!(Severity::from_str("major").unwrap(), Severity::Major);
        assert_eq!(Severity::from_str("Major").unwrap(), Severity::Major);
        assert_eq!(Severity::from_str("MAJOR").unwrap(), Severity::Major);
    }

    #[test]
    fn from_str_invalid() {
        assert!(Severity::from_str("invalid").is_err());
    }

    #[test]
    fn point_value() {
        assert_eq!(Severity::Minor.point_value(), 1);
        assert_eq!(Severity::Major.point_value(), 2);
    }

    #[test]
    fn display() {
        assert_eq!(Severity::Minor.to_string(), "minor");
        assert_eq!(Severity::Major.to_string(), "major");
    }

    #[test]
    fn serde_roundtrip() {
        let minor = Severity::Minor;
        let json = serde_json::to_string(&minor).unwrap();
        assert_eq!(json, "\"minor\"");
        let parsed: Severity = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, minor);
    }
}
