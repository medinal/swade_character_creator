//! Game rule constants for Savage Worlds Adventure Edition (SWADE).
//!
//! These constants define the base values and costs used throughout the game system.

// ============================================================================
// Derived Stat Base Values
// ============================================================================

/// Base pace for all characters before modifiers.
pub const BASE_PACE: i32 = 6;

/// Base parry before Fighting skill bonus and modifiers.
pub const BASE_PARRY: i32 = 2;

/// Base toughness before Vigor bonus, size, and modifiers.
pub const BASE_TOUGHNESS: i32 = 2;

// ============================================================================
// Point Costs
// ============================================================================

/// Cost in hindrance points to purchase one edge.
pub const EDGE_HINDRANCE_POINT_COST: i64 = 2;

/// Cost in hindrance points to purchase one attribute point.
pub const ATTRIBUTE_HINDRANCE_POINT_COST: i64 = 2;

/// Skill points to hindrance points conversion ratio (1:1).
pub const SKILL_HINDRANCE_POINT_RATIO: i64 = 1;

// ============================================================================
// Source Type Identifiers
// ============================================================================
// These identify where a character element (edge, hindrance, etc.) came from.

/// Source for edges/hindrances chosen by the player during character creation.
pub const SOURCE_CHOSEN: &str = "chosen";

/// Source for edges/hindrances granted by the character's ancestry.
pub const SOURCE_ANCESTRY: &str = "ancestry";

/// Source for edges purchased with hindrance points.
pub const SOURCE_HINDRANCE_POINTS: &str = "hindrance_points";

/// Source for hindrances/powers from an arcane background.
pub const SOURCE_ARCANE_BACKGROUND: &str = "arcane_background";
