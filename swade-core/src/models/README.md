# Models

Base models are pure data structures that map directly to database tables. They contain no business logic.

## Purpose

Models serve as the data transfer layer between the database and the rest of the application. They are:

- **Pure data** - Only fields, no methods beyond basic construction
- **1:1 with tables** - Each model maps to exactly one database table
- **Serializable** - Derive `Serialize`/`Deserialize` for JSON export
- **Cloneable** - Derive `Clone` for easy copying when building view models

## Architecture Role

```
Database → Repository → Model → Service → View Model → Presentation
                         ↑
                    You are here
```

Models are loaded by repositories and passed to services, which assemble them into view models. Business logic (validation, calculations, requirement evaluation) belongs in view models, not here.

## Implementation Guidelines

### Struct Definition

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Edge {
    pub id: i64,
    pub name: String,
    pub background: String,
    pub source: String,
    pub description: String,
    pub can_take_multiple_times: bool,
    pub created_at: String,
    pub updated_at: String,
}
```

### Required Derives

All models must derive:
- `Debug` - For logging and error messages
- `Clone` - For building view models
- `Serialize`, `Deserialize` - For JSON import/export

Some models also derive `specta::Type` for TypeScript type generation (e.g., `Rank`, `Modifier`, `Requirement`).

### Field Types

| SQLite Type | Rust Type |
|-------------|-----------|
| `INTEGER` (id) | `i64` |
| `INTEGER` (nullable) | `Option<i64>` |
| `INTEGER` (boolean) | `bool` |
| `REAL` | `f64` |
| `VARCHAR`/`TEXT` | `String` |
| `VARCHAR` (nullable) | `Option<String>` |
| `BLOB` (nullable) | `Option<Vec<u8>>` |
| `TIMESTAMP` | `String` |

### Naming Conventions

- Struct names: `PascalCase`, singular (e.g., `Edge`, not `Edges`)
- Field names: `snake_case`, matching database column names exactly
- File names: `snake_case.rs` (e.g., `edge.rs`, `arcane_background.rs`)

### Junction Tables

Junction tables (many-to-many relationships) get their own models:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgeModifier {
    pub id: i64,
    pub edge_id: i64,
    pub modifier_id: i64,
    pub created_at: String,
    pub updated_at: String,
}
```

## What Belongs Here

- Struct definitions with public fields
- Optional `new()` constructor for convenience
- Optional `Default` implementation if sensible defaults exist

## What Does NOT Belong Here

- Validation logic → view models
- Business rules → view models
- Database queries → repositories
- Data transformation → services
- Calculated fields → view models

## Testing

Models are pure data and generally don't need unit tests. Test coverage comes from:

1. **Repository tests** - Verify models load correctly from database
2. **View model tests** - Verify business logic using model data
3. **Integration tests** - Verify end-to-end data flow

If you add a `new()` constructor or `Default` impl, add a simple test:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_edge_new() {
        let edge = Edge::new(1, "Alertness".to_string());
        assert_eq!(edge.id, 1);
        assert_eq!(edge.name, "Alertness");
    }
}
```

## Model Categories

### Core Game Data (Seed Tables)

These models represent immutable SWADE rules:

- `Attribute` - Agility, Smarts, Spirit, Strength, Vigor
- `Skill` - Skills linked to attributes
- `Rank` - Novice, Seasoned, Veteran, Heroic, Legendary
- `Edge` - Character advantages
- `Hindrance` - Character disadvantages
- `Ancestry` - Races/species
- `ArcaneBackground` - Magic types
- `Power` - Spells and abilities

### Modifier System

- `Modifier` - Generic bonus/penalty that can be applied to various targets

### Requirement System

- `Requirement` - Atomic prerequisite (e.g., "Agility d8")
- `RequirementExpression` - Boolean tree node for complex prerequisites

### Gear System

Equipment and inventory:

- `Gear` - Base gear item with name, category, era, cost, weight
- `GearCategory` - Categories for organizing gear
- `WeaponStats` - Weapon-specific stats (damage, AP, range, ROF, etc.)
- `ArmorStats` - Armor-specific stats (armor value, coverage, heavy flag)
- `ShieldStats` - Shield-specific stats (parry bonus, cover penalty)
- `AmmunitionStats` - Ammunition-specific stats (ammo type, quantity per unit)

### Junction Tables

Link modifiers and requirements to game elements:

- `AncestryModifier`, `AncestryRequirement`
- `EdgeModifier`, `EdgeRequirement`
- `HindranceModifier`, `HindranceRequirement`
- `PowerModifier`, `PowerRequirement`
- `ArcaneBackgroundRequirement`
- `GearModifier`, `GearRequirement`

### Ancestry Choices

- `AncestryChoice` - A choice an ancestry offers (e.g., "free edge")
- `AncestryChoiceOption` - An option within that choice (e.g., "Alertness")

### Character Data

Player-created character information:

- `Character` - Core character record
- `CharacterAttribute`, `CharacterSkill` - Attribute/skill values
- `CharacterEdge`, `CharacterHindrance` - Taken edges/hindrances
- `CharacterPower`, `CharacterArcaneBackground` - Arcane selections
- `CharacterModifier` - Applied modifiers
- `CharacterAncestryChoice` - Selected ancestry options
- `CharacterAdvance` - Advancement records (level-up choices)
- `CharacterNote` - Player notes attached to characters
- `CharacterGear` - Character inventory (quantity, equipped status, custom notes)
