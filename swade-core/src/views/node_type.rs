use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

/// Node types for requirement expression trees.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "lowercase")]
pub enum NodeType {
    Requirement,
    And,
    Or,
    Not,
}

impl FromStr for NodeType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "requirement" => Ok(NodeType::Requirement),
            "and" => Ok(NodeType::And),
            "or" => Ok(NodeType::Or),
            "not" => Ok(NodeType::Not),
            _ => Err(format!("Invalid node type: {}", s)),
        }
    }
}

impl fmt::Display for NodeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NodeType::Requirement => write!(f, "requirement"),
            NodeType::And => write!(f, "and"),
            NodeType::Or => write!(f, "or"),
            NodeType::Not => write!(f, "not"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str_all_variants() {
        assert_eq!(
            NodeType::from_str("requirement").unwrap(),
            NodeType::Requirement
        );
        assert_eq!(NodeType::from_str("and").unwrap(), NodeType::And);
        assert_eq!(NodeType::from_str("or").unwrap(), NodeType::Or);
        assert_eq!(NodeType::from_str("not").unwrap(), NodeType::Not);
    }

    #[test]
    fn from_str_case_insensitive() {
        assert_eq!(
            NodeType::from_str("REQUIREMENT").unwrap(),
            NodeType::Requirement
        );
        assert_eq!(NodeType::from_str("And").unwrap(), NodeType::And);
        assert_eq!(NodeType::from_str("OR").unwrap(), NodeType::Or);
    }

    #[test]
    fn from_str_invalid() {
        assert!(NodeType::from_str("invalid").is_err());
    }

    #[test]
    fn display() {
        assert_eq!(NodeType::Requirement.to_string(), "requirement");
        assert_eq!(NodeType::And.to_string(), "and");
        assert_eq!(NodeType::Or.to_string(), "or");
        assert_eq!(NodeType::Not.to_string(), "not");
    }

    #[test]
    fn serde_roundtrip() {
        let value = NodeType::And;
        let json = serde_json::to_string(&value).unwrap();
        assert_eq!(json, "\"and\"");
        let parsed: NodeType = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, value);
    }
}
