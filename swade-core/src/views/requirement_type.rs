use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

/// Types of requirements.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "snake_case")]
pub enum RequirementType {
    Attribute,
    Skill,
    Rank,
    ArcaneSkill,
    Edge,
    ArcaneBackground,
    WildCard,
    Description,
}

impl FromStr for RequirementType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "attribute" => Ok(RequirementType::Attribute),
            "skill" => Ok(RequirementType::Skill),
            "rank" => Ok(RequirementType::Rank),
            "arcane_skill" => Ok(RequirementType::ArcaneSkill),
            "edge" => Ok(RequirementType::Edge),
            "arcane_background" => Ok(RequirementType::ArcaneBackground),
            "wild_card" => Ok(RequirementType::WildCard),
            "description" => Ok(RequirementType::Description),
            _ => Err(format!("Invalid requirement type: {}", s)),
        }
    }
}

impl fmt::Display for RequirementType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RequirementType::Attribute => write!(f, "attribute"),
            RequirementType::Skill => write!(f, "skill"),
            RequirementType::Rank => write!(f, "rank"),
            RequirementType::ArcaneSkill => write!(f, "arcane_skill"),
            RequirementType::Edge => write!(f, "edge"),
            RequirementType::ArcaneBackground => write!(f, "arcane_background"),
            RequirementType::WildCard => write!(f, "wild_card"),
            RequirementType::Description => write!(f, "description"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str_all_variants() {
        assert_eq!(
            RequirementType::from_str("attribute").unwrap(),
            RequirementType::Attribute
        );
        assert_eq!(
            RequirementType::from_str("skill").unwrap(),
            RequirementType::Skill
        );
        assert_eq!(
            RequirementType::from_str("rank").unwrap(),
            RequirementType::Rank
        );
        assert_eq!(
            RequirementType::from_str("arcane_skill").unwrap(),
            RequirementType::ArcaneSkill
        );
        assert_eq!(
            RequirementType::from_str("edge").unwrap(),
            RequirementType::Edge
        );
        assert_eq!(
            RequirementType::from_str("arcane_background").unwrap(),
            RequirementType::ArcaneBackground
        );
        assert_eq!(
            RequirementType::from_str("wild_card").unwrap(),
            RequirementType::WildCard
        );
        assert_eq!(
            RequirementType::from_str("description").unwrap(),
            RequirementType::Description
        );
    }

    #[test]
    fn from_str_case_insensitive() {
        assert_eq!(
            RequirementType::from_str("ATTRIBUTE").unwrap(),
            RequirementType::Attribute
        );
        assert_eq!(
            RequirementType::from_str("Arcane_Background").unwrap(),
            RequirementType::ArcaneBackground
        );
    }

    #[test]
    fn from_str_invalid() {
        assert!(RequirementType::from_str("invalid").is_err());
    }

    #[test]
    fn display() {
        assert_eq!(RequirementType::Attribute.to_string(), "attribute");
        assert_eq!(RequirementType::Rank.to_string(), "rank");
        assert_eq!(
            RequirementType::ArcaneBackground.to_string(),
            "arcane_background"
        );
    }

    #[test]
    fn serde_roundtrip() {
        let value = RequirementType::Skill;
        let json = serde_json::to_string(&value).unwrap();
        assert_eq!(json, "\"skill\"");
        let parsed: RequirementType = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, value);
    }
}
