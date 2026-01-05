# View Models

View models combine data from multiple base models into cohesive objects for the presentation layer. They are the home for business logic, but that logic should be added incrementally as needed.

## Purpose

View models serve as the domain logic layer, responsible for:

- Assembling related data from multiple base models
- Providing display-ready data to the presentation layer
- Implementing business logic **as concrete needs arise**

Business logic (validation, calculations, rule enforcement) belongs here, but should be added only when a consumer actually needs it.

## Architecture Role

```
Database → Repository → Model → Service → View Model → Presentation
                                              ↑
                                         You are here
```

Services load base models from repositories and assemble them into view models. The presentation layer (CLI/GUI) consumes view models directly. View models are self-contained and do not depend on services or repositories.

## File Organization

All view models live in single files. The views directory is organized as follows:

```
src/views/
├── mod.rs
│
├── # Reference Data Views (SWADE game elements)
├── edge_view.rs                       # Edge with modifiers and requirements
├── hindrance_view.rs                  # Hindrance with modifiers and point values
├── ancestry_view.rs                   # Ancestry with modifiers and choices
├── ancestry_choice_view.rs            # Ancestry choice definitions
├── ancestry_choice_option_view.rs     # Options within ancestry choices
├── power_view.rs                      # Power with modifiers and PP cost
├── arcane_background_view.rs          # Arcane background with skill link
├── attribute_view.rs                  # Attribute definitions
├── skill_view.rs                      # Skill definitions
│
├── # Gear Views (equipment and items)
├── gear_view.rs                       # Gear item with category and type-specific stats
├── gear_category_view.rs              # Gear category (weapons, armor, etc.)
├── weapon_stats_view.rs               # Weapon statistics (damage, AP, range, RoF, reach)
├── armor_stats_view.rs                # Armor statistics (armor value, coverage)
├── shield_stats_view.rs               # Shield statistics (parry bonus, cover)
├── ammunition_stats_view.rs           # Ammunition statistics (type, quantity)
│
├── # Character Value Views (character-specific instances)
├── character_view.rs                  # Complete character with all related data
├── character_attribute_value.rs       # Character's attribute die value
├── character_skill_value.rs           # Character's skill die value
├── character_edge_value.rs            # Edge assigned to a character
├── character_hindrance_value.rs       # Hindrance assigned to a character
├── character_power_value.rs           # Power assigned to a character
├── character_arcane_background_value.rs # Arcane background for a character
├── character_ancestry_choice_value.rs # Ancestry choice selection for a character
├── character_advance_value.rs         # Advancement types, options, and records
├── character_note_value.rs            # Notes attached to a character
├── character_gear_value.rs            # Gear item owned by a character
│
├── # Supporting Types
├── die.rs                             # Type-safe SWADE die value (d4-d12, with modifiers)
├── derived_stats_view.rs              # Pace, Parry, Toughness calculations
├── encumbrance_info.rs                # Encumbrance status and load limits
├── requirement_tree.rs                # Boolean expression tree for prerequisites
├── game_config.rs                     # Game configuration settings
│
├── # Enum Types (serializable with string representations)
├── ancestry_choice_type.rs            # Types of ancestry choices
├── ancestry_option_type.rs            # Types of ancestry options
├── edge_category.rs                   # Edge category classification
├── severity.rs                        # Hindrance severity (Minor/Major)
├── modifier_target_type.rs            # What a modifier targets
├── modifier_value_type.rs             # Type of modifier value
├── node_type.rs                       # Requirement tree node types
└── requirement_type.rs                # Types of requirements
```

## Key Principles

### 1. Start Minimal, Add Logic as Needed

Begin with just the struct definition and constructor. Add methods only when a concrete consumer (presentation layer, another view model, etc.) needs them.

```rust
// Start with this - data and construction only
pub struct EdgeView {
    pub id: i64,
    pub name: String,
    pub category: EdgeCategory,
    pub description: String,
    pub modifiers: Vec<Modifier>,
    pub requirements: RequirementTree,
}

impl EdgeView {
    pub fn new(edge: Edge, modifiers: Vec<Modifier>, requirements: RequirementTree) -> Self {
        let category = EdgeCategory::from_str(&edge.background).unwrap_or(EdgeCategory::Background);
        Self {
            id: edge.id,
            name: edge.name,
            category,
            description: edge.description,
            modifiers,
            requirements,
        }
    }
}

// Later, when the character creation wizard needs to check eligibility,
// add the method then - not before
impl EdgeView {
    pub fn is_available_to(&self, character: &CharacterView) -> bool {
        self.requirements.evaluate(character)
    }
}
```

**Don't** add methods speculatively. If you're thinking "we might need this later," wait until later.

### 2. Mutability Depends on Purpose

View models fall into two categories with different mutability rules:

**Reference Data Views** (immutable) - Represent rulebook data for display and querying:

- `EdgeView`, `HindranceView`, `AncestryView`, `PowerView`, `ArcaneBackgroundView`
- Constructed once by services, never modified
- Used to browse available options and check requirements

```rust
// Reference views are immutable after construction
let edge_view = EdgeView::new(edge, modifiers, requirements);

// Don't mutate reference views
edge_view.name = "New Name".to_string();  // Don't do this
```

**Working Document Views** (mutable) - Represent entities being actively edited:

- `CharacterView` during character creation/advancement
- May expose mutation methods that enforce business rules
- Add mutation methods as the presentation layer needs them

```rust
// Start with data and construction
pub struct CharacterView {
    pub id: i64,
    pub name: String,
    pub attributes: HashMap<i64, Die>,
    pub edges: Vec<CharacterEdge>,
    // ...
}

// Add mutation methods when the wizard/UI actually needs them
impl CharacterView {
    pub fn add_edge(&mut self, edge: &EdgeView) -> Result<(), ValidationError> {
        // validation and mutation logic
    }
}
```

### 3. Self-Contained

View models contain all data needed for their use case. No additional queries required:

```rust
pub struct EdgeView {
    // Base data
    pub id: i64,
    pub name: String,
    pub category: EdgeCategory,
    pub description: String,

    // Related data (pre-loaded)
    pub modifiers: Vec<Modifier>,
    pub requirements: RequirementTree,
}
```

### 4. No Database Access

View models never touch the database. All data comes through construction:

```rust
// Good - data passed in during construction
impl EdgeView {
    pub fn new(edge: Edge, modifiers: Vec<Modifier>, req_tree: RequirementTree) -> Self;
}

// Bad - don't query inside view models
impl EdgeView {
    pub fn load(conn: &Connection, id: i64) -> Self;  // Don't do this
}
```

## View Model Categories

### Reference Data Views (Immutable)

Representations of SWADE game elements for display:

- `EdgeView` - Edge with modifiers and requirements
- `HindranceView` - Hindrance with modifiers and point values
- `AncestryView` - Ancestry with modifiers and choices
- `AncestryChoiceView` - Ancestry choice definitions
- `AncestryChoiceOptionView` - Options within ancestry choices
- `PowerView` - Power with modifiers and PP cost
- `ArcaneBackgroundView` - Arcane background with skill link
- `AttributeView` - Attribute definitions
- `SkillView` - Skill definitions

### Gear Views (Immutable)

Equipment and item representations:

- `GearView` - Gear item with category and type-specific stats (weapon/armor/shield/ammo)
- `GearCategoryView` - Gear category classification
- `WeaponStatsView` - Weapon statistics (damage, AP, range, RoF, shots, min strength, reach, two-handed)
- `ArmorStatsView` - Armor statistics (armor value, coverage, min strength, heavy flag)
- `ShieldStatsView` - Shield statistics (parry bonus, cover penalty, min strength)
- `AmmunitionStatsView` - Ammunition statistics (ammo type, quantity per unit)

### Working Document Views (Mutable)

Entities being actively created or modified:

- `CharacterView` - Character being built or advanced

### Character Value Views

Character-specific instances of game elements:

- `CharacterAttributeValue` - Character's attribute with die value
- `CharacterSkillValue` - Character's skill with die value and effective value
- `CharacterEdgeValue` - Edge assigned to a character
- `CharacterHindranceValue` - Hindrance assigned to a character
- `CharacterPowerValue` - Power assigned to a character
- `CharacterArcaneBackgroundValue` - Arcane background for a character
- `CharacterAncestryChoiceValue` - Ancestry choice selection for a character
- `CharacterAdvanceValue` - Advancement record for a character
- `AdvanceType` - Type of advancement taken (edge, attribute, skill, hindrance)
- `AdvancementOptions` - Options available for a character's next advancement
- `AttributeAdvanceOption` - An attribute that can be increased via advancement
- `SkillAdvanceOption` - A skill that can be increased via advancement
- `HindranceAdvanceOption` - A hindrance that can be modified via advancement
- `HindranceAction` - The action that can be taken on a hindrance
- `CharacterNoteValue` - Notes attached to a character
- `CharacterGearValue` - Gear item owned by a character (with quantity, equipped status)

### Supporting Types

Utility types used within view models:

- `Die` - Type-safe SWADE die value (d4 through d12, with d12+ modifiers for values beyond d12)
- `DerivedStatsView` - Computed Pace, Parry, Toughness, and Size values
- `EncumbranceInfo` - Encumbrance status (current weight, load limit, penalty)
- `RequirementTree` - Boolean expression tree for evaluating prerequisites
- `RequirementContext` - Context for evaluating requirements against a character
- `GameConfig` - Game configuration settings

## Implementation Guidelines

### Start with Struct and Constructor

Every view model starts with just the data structure and a way to build it:

```rust
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use crate::models::{Edge, Modifier};
use crate::views::{EdgeCategory, RequirementTree};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgeView {
    pub id: i64,
    pub name: String,
    pub category: EdgeCategory,
    pub source: String,
    pub description: String,
    pub can_take_multiple_times: bool,
    pub modifiers: Vec<Modifier>,
    pub requirements: RequirementTree,
}

impl EdgeView {
    pub fn new(
        edge: Edge,
        modifiers: Vec<Modifier>,
        requirements: RequirementTree,
    ) -> Self {
        let category = EdgeCategory::from_str(&edge.background).unwrap_or(EdgeCategory::Background);
        Self {
            id: edge.id,
            name: edge.name,
            category,
            source: edge.source,
            description: edge.description,
            can_take_multiple_times: edge.can_take_multiple_times,
            modifiers,
            requirements,
        }
    }
}
```

This is often sufficient. The presentation layer can access fields directly for display.

### Adding Methods

Add methods when you have a concrete use case. Examples of when to add methods:

- The UI needs to display "Requires: Agility d8, Fighting d6" → add `requirements_display(&self) -> String`
- The character wizard needs to filter available edges → add `is_available_to(&self, character: &CharacterView) -> bool`
- The character sheet needs to show why an edge can't be taken → add `unmet_requirements(&self, character: &CharacterView) -> Vec<String>`

Examples of methods that might be added later (don't add these upfront):

```rust
// Add when the character wizard needs eligibility checking
impl EdgeView {
    pub fn is_available_to(&self, character: &CharacterView) -> bool {
        self.requirements.evaluate(character)
    }
}

// Add when the UI needs to explain why an edge is unavailable
impl EdgeView {
    pub fn unmet_requirements(&self, character: &CharacterView) -> Vec<String> {
        self.requirements.unmet_requirements(character)
    }
}

// Add when calculating a character's total bonuses
impl EdgeView {
    pub fn bonus_to(&self, stat: &str) -> i32 {
        self.modifiers
            .iter()
            .filter(|m| m.target_identifier == stat)
            .map(|m| m.value)
            .sum()
    }
}
```

### RequirementTree

The `RequirementTree` type is an exception to "start minimal" - it exists specifically to evaluate prerequisites, so evaluation logic is core to its purpose:

```rust
impl RequirementTree {
    pub fn evaluate(&self, ctx: &RequirementContext) -> bool {
        match &self.node {
            RequirementNode::And(children) => children.iter().all(|c| c.evaluate(ctx)),
            RequirementNode::Or(children) => children.iter().any(|c| c.evaluate(ctx)),
            RequirementNode::Not(child) => !child.evaluate(ctx),
            RequirementNode::Leaf(req) => Self::evaluate_requirement(req, ctx),
        }
    }
}
```

### Die Type

The `Die` type represents a SWADE die value. It supports sizes d4 through d12, and values beyond d12 are represented as d12+N (e.g., d12+1, d12+2):

```rust
use serde::{Deserialize, Serialize};

/// A SWADE die value.
/// Represents die sizes from d4 to d12, and beyond d12 with modifiers (d12+1, d12+2, etc.).
/// The progression is: d4 -> d6 -> d8 -> d10 -> d12 -> d12+1 -> d12+2 -> ...
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Die {
    size: u8,
    modifier: u8,
}

impl Die {
    const VALID_SIZES: [u8; 5] = [4, 6, 8, 10, 12];

    /// Create a die from a size, returning None if the size is invalid.
    pub fn new(size: u8) -> Option<Self> {
        if Self::VALID_SIZES.contains(&size) {
            Some(Self { size, modifier: 0 })
        } else {
            None
        }
    }

    /// Create a die with a modifier (e.g., d12+2).
    /// Only d12 can have a modifier; other sizes return None if modifier > 0.
    pub fn with_modifier(size: u8, modifier: u8) -> Option<Self>;

    /// Convenience constructors for each die type
    pub fn d4() -> Self;
    pub fn d6() -> Self;
    pub fn d8() -> Self;
    pub fn d10() -> Self;
    pub fn d12() -> Self;

    /// Get the die size (4, 6, 8, 10, or 12).
    pub fn size(&self) -> u8;

    /// Get the modifier (0 for normal dice, 1+ for d12+N).
    pub fn modifier(&self) -> u8;

    /// Increment to the next die value.
    /// d4 -> d6 -> d8 -> d10 -> d12 -> d12+1 -> d12+2 -> ...
    pub fn increment(&self) -> Self;

    /// Decrement to the previous die value, returning None if already at d4.
    /// d12+2 -> d12+1 -> d12 -> d10 -> d8 -> d6 -> d4 -> None
    pub fn decrement(&self) -> Option<Self>;

    /// Calculate steps between two dice values.
    pub fn steps_from(&self, from: Die) -> u8;
}

impl fmt::Display for Die {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.modifier == 0 {
            write!(f, "d{}", self.size)
        } else {
            write!(f, "d{}+{}", self.size, self.modifier)
        }
    }
}
```

### CharacterSkillValue

Represents a character's skill with both purchased and effective die values:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterSkillValue {
    pub skill: SkillView,
    /// The purchased die value (None for untrained).
    pub die: Option<Die>,
    /// The effective die value (purchased + modifiers). None for untrained.
    pub effective_die: Option<Die>,
    /// Whether the current die is above the linked attribute's die.
    pub is_above_attribute: bool,
    /// The cost to increment this skill (1 if at/below attribute, 2 if above).
    pub increment_cost: i64,
    /// Whether this skill can be incremented.
    pub can_increment: bool,
    /// Whether this skill can be decremented.
    pub can_decrement: bool,
}
```

## What Belongs Here

- Struct definitions combining base models with related data
- Constructors that assemble view models from components
- Business logic methods **when a consumer needs them**
- Display formatting methods **when the UI needs them**

## What Does NOT Belong Here

- Database queries → repositories
- Data loading orchestration → services
- HTTP/CLI handling → presentation layer
- Base model definitions → models layer
- Persistence logic → repositories
- Speculative methods "we might need later"

## Testing

Test the logic you have, not the logic you might add. Tests live in the same file as the code they test, within a `#[cfg(test)]` module.

### Basic Pattern

```rust
// edge_view.rs

pub struct EdgeView {
    pub id: i64,
    pub name: String,
    // ...
}

impl EdgeView {
    pub fn new(edge: Edge, modifiers: Vec<Modifier>, requirements: RequirementTree) -> Self {
        // ...
    }
}

// If you've added a method, test it
impl EdgeView {
    pub fn bonus_to(&self, stat: &str) -> i32 {
        self.modifiers
            .iter()
            .filter(|m| m.target_identifier == stat)
            .map(|m| m.value)
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_edge_with_modifier(stat: &str, value: i32) -> EdgeView {
        // helper to build test data
    }

    #[test]
    fn test_bonus_to_matching_stat() {
        let edge = create_test_edge_with_modifier("Notice", 2);
        assert_eq!(edge.bonus_to("Notice"), 2);
    }

    #[test]
    fn test_bonus_to_unrelated_stat() {
        let edge = create_test_edge_with_modifier("Notice", 2);
        assert_eq!(edge.bonus_to("Fighting"), 0);
    }
}
```

### Test What Exists

Don't write tests for methods you haven't implemented yet. When you add a method, add its tests at the same time.
