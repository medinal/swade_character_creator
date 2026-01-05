use serde::{Deserialize, Serialize};

/// Represents an advance that a character has taken, with resolved names for display.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct CharacterAdvanceValue {
    pub id: i64,
    pub advance_number: i64,
    pub advance_type: AdvanceType,
    pub description: String,
    pub created_at: String,
}

/// An attribute that can be increased via advancement.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct AttributeAdvanceOption {
    pub id: i64,
    pub name: String,
    /// Base die size (without modifiers)
    pub current_die: u8,
    /// Effective die size (with modifiers applied) - for display
    pub effective_die: u8,
    /// Base die size after advancement
    pub next_die: u8,
    /// Effective die size after advancement (with modifiers) - for display
    pub effective_next_die: u8,
    pub is_maxed: bool,
}

/// A skill that can be increased via advancement.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct SkillAdvanceOption {
    pub id: i64,
    pub name: String,
    /// Base die size (0 if untrained)
    pub current_die: u8,
    /// Effective die size (with modifiers applied) - for display
    pub effective_die: u8,
    /// Base die size after advancement
    pub next_die: u8,
    /// Effective die size after advancement (with modifiers) - for display
    pub effective_next_die: u8,
    pub is_maxed: bool,
}

/// The action that can be taken on a hindrance.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "snake_case")]
pub enum HindranceAction {
    /// Remove a minor hindrance entirely
    RemoveMinor,
    /// Reduce a major hindrance to its minor version
    ReduceMajor,
    /// Bank one advance toward removing a major (first of two)
    RemoveMajorHalf,
    /// Complete removal of a major hindrance (second of two)
    CompleteMajorRemoval,
}

impl HindranceAction {
    pub fn as_str(&self) -> &'static str {
        match self {
            HindranceAction::RemoveMinor => "remove_minor",
            HindranceAction::ReduceMajor => "reduce_major",
            HindranceAction::RemoveMajorHalf => "remove_major_half",
            HindranceAction::CompleteMajorRemoval => "remove_major_half", // Same DB value, different meaning
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            HindranceAction::RemoveMinor => "Remove",
            HindranceAction::ReduceMajor => "Reduce to Minor",
            HindranceAction::RemoveMajorHalf => "Begin Removal (requires 2 advances)",
            HindranceAction::CompleteMajorRemoval => "Complete Removal (2nd advance)",
        }
    }
}

/// A hindrance that can be modified via advancement.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct HindranceAdvanceOption {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub severity: String,
    /// The action available for this hindrance
    pub action: HindranceAction,
    /// Human-readable label for the action
    pub action_label: String,
    /// Whether this hindrance has a banked advance
    pub is_banked: bool,
}

/// The type of advancement taken.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AdvanceType {
    Edge,
    Attribute,
    SkillExpensive,
    SkillCheap,
    Hindrance,
}

impl std::str::FromStr for AdvanceType {
    type Err = ();

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "edge" => Ok(AdvanceType::Edge),
            "attribute" => Ok(AdvanceType::Attribute),
            "skill_expensive" => Ok(AdvanceType::SkillExpensive),
            "skill_cheap" => Ok(AdvanceType::SkillCheap),
            "hindrance" => Ok(AdvanceType::Hindrance),
            _ => Ok(AdvanceType::Edge), // Default fallback
        }
    }
}

impl AdvanceType {
    pub fn as_str(&self) -> &'static str {
        match self {
            AdvanceType::Edge => "edge",
            AdvanceType::Attribute => "attribute",
            AdvanceType::SkillExpensive => "skill_expensive",
            AdvanceType::SkillCheap => "skill_cheap",
            AdvanceType::Hindrance => "hindrance",
        }
    }
}

/// Options available for a character's next advancement.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct AdvancementOptions {
    /// Can take an edge (always true if requirements are met)
    pub can_take_edge: bool,
    /// Can increase an attribute (limited by rank rules)
    pub can_increase_attribute: bool,
    /// Reason why attribute increase is not available (if applicable)
    pub attribute_blocked_reason: Option<String>,
    /// Attributes available for advancement
    pub attribute_options: Vec<AttributeAdvanceOption>,
    /// Can increase one expensive skill (at/above linked attribute)
    pub can_increase_expensive_skill: bool,
    /// Skills at or above linked attribute (one per advance)
    pub expensive_skill_options: Vec<SkillAdvanceOption>,
    /// Can increase two cheap skills (below linked attribute)
    pub can_increase_cheap_skills: bool,
    /// Skills below linked attribute (pick two per advance)
    pub cheap_skill_options: Vec<SkillAdvanceOption>,
    /// Can remove/reduce a hindrance
    pub can_modify_hindrance: bool,
    /// Hindrances available for modification with their actions
    pub hindrance_options: Vec<HindranceAdvanceOption>,
    /// Current advance number (what number this advance would be)
    pub next_advance_number: i64,
    /// Current rank name
    pub current_rank: String,
    /// Rank after taking this advance
    pub rank_after_advance: String,
}
