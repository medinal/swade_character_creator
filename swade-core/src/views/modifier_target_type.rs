use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

/// Types of modifier targets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "snake_case")]
pub enum ModifierTargetType {
    Attribute,
    Skill,
    DerivedStat,
    EdgeChoice,
    HindranceChoice,
    HeritageChoice,
    SkillPoints,
    AttributePoints,
}

impl FromStr for ModifierTargetType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "attribute" => Ok(ModifierTargetType::Attribute),
            "skill" => Ok(ModifierTargetType::Skill),
            "derived_stat" => Ok(ModifierTargetType::DerivedStat),
            "edge_choice" => Ok(ModifierTargetType::EdgeChoice),
            "hindrance_choice" => Ok(ModifierTargetType::HindranceChoice),
            "heritage_choice" => Ok(ModifierTargetType::HeritageChoice),
            "skill_points" => Ok(ModifierTargetType::SkillPoints),
            "attribute_points" => Ok(ModifierTargetType::AttributePoints),
            _ => Err(format!("Invalid modifier target type: {}", s)),
        }
    }
}

impl fmt::Display for ModifierTargetType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ModifierTargetType::Attribute => write!(f, "attribute"),
            ModifierTargetType::Skill => write!(f, "skill"),
            ModifierTargetType::DerivedStat => write!(f, "derived_stat"),
            ModifierTargetType::EdgeChoice => write!(f, "edge_choice"),
            ModifierTargetType::HindranceChoice => write!(f, "hindrance_choice"),
            ModifierTargetType::HeritageChoice => write!(f, "heritage_choice"),
            ModifierTargetType::SkillPoints => write!(f, "skill_points"),
            ModifierTargetType::AttributePoints => write!(f, "attribute_points"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str_all_variants() {
        assert_eq!(
            ModifierTargetType::from_str("attribute").unwrap(),
            ModifierTargetType::Attribute
        );
        assert_eq!(
            ModifierTargetType::from_str("skill").unwrap(),
            ModifierTargetType::Skill
        );
        assert_eq!(
            ModifierTargetType::from_str("derived_stat").unwrap(),
            ModifierTargetType::DerivedStat
        );
        assert_eq!(
            ModifierTargetType::from_str("edge_choice").unwrap(),
            ModifierTargetType::EdgeChoice
        );
        assert_eq!(
            ModifierTargetType::from_str("hindrance_choice").unwrap(),
            ModifierTargetType::HindranceChoice
        );
        assert_eq!(
            ModifierTargetType::from_str("heritage_choice").unwrap(),
            ModifierTargetType::HeritageChoice
        );
        assert_eq!(
            ModifierTargetType::from_str("skill_points").unwrap(),
            ModifierTargetType::SkillPoints
        );
        assert_eq!(
            ModifierTargetType::from_str("attribute_points").unwrap(),
            ModifierTargetType::AttributePoints
        );
    }

    #[test]
    fn from_str_case_insensitive() {
        assert_eq!(
            ModifierTargetType::from_str("ATTRIBUTE").unwrap(),
            ModifierTargetType::Attribute
        );
        assert_eq!(
            ModifierTargetType::from_str("Skill_Points").unwrap(),
            ModifierTargetType::SkillPoints
        );
    }

    #[test]
    fn from_str_invalid() {
        assert!(ModifierTargetType::from_str("invalid").is_err());
    }

    #[test]
    fn display() {
        assert_eq!(ModifierTargetType::Attribute.to_string(), "attribute");
        assert_eq!(ModifierTargetType::DerivedStat.to_string(), "derived_stat");
        assert_eq!(ModifierTargetType::SkillPoints.to_string(), "skill_points");
    }

    #[test]
    fn serde_roundtrip() {
        let value = ModifierTargetType::Skill;
        let json = serde_json::to_string(&value).unwrap();
        assert_eq!(json, "\"skill\"");
        let parsed: ModifierTargetType = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, value);
    }
}
