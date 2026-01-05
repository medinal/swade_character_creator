# Services

Services are the orchestration layer between repositories and view models. They load data from the database and assemble it into view models for the presentation layer.

## Purpose

Services are responsible for:

- Loading base models from repositories
- Assembling view models from base models and related data
- Computing derived values during assembly (e.g., die value from base + steps)
- Coordinating multiple repository calls when needed

Most services are **thin** - they contain no business logic. However, `AdvancementService` is an exception: it contains advancement validation, requirement checking, and rule enforcement because these operations span multiple entities and must be coordinated at the service layer.

## Architecture Role

```
Database → Repository → Model → Service → View Model → Presentation
                                   ↑
                              You are here
```

Services receive a database connection, call repositories to load base models, and construct view models. The presentation layer calls services to get the view models it needs.

## Connection Lifecycle

The presentation layer (CLI or GUI) owns the database and connection lifecycle. It initializes the database at startup and passes connections to services:

```rust
// In main.rs or a CLI command handler
fn main() -> Result<()> {
    let db = Database::init()?;
    let conn = db.connection()?;

    // Call services with the connection
    let attributes = AttributeService::get_all(&conn)?;
    let edges = EdgeService::get_all(&conn)?;

    // Display to user...
}
```

Services never create their own connections. This keeps:
- Database initialization in one place (presentation layer startup)
- Services stateless and testable (just pass in a connection)
- Transaction control at the top level when needed

## Key Principles

### 1. Orchestration Only

Services coordinate data loading and view model construction. They don't make decisions about game rules.

```rust
// Good - just loading and assembling
impl AttributeService {
    pub fn get_all(conn: &Connection) -> Result<Vec<AttributeView>> {
        let attributes = AttributeRepository::get_all(conn)?;
        Ok(attributes.into_iter().map(AttributeView::new).collect())
    }
}

// Bad - business logic belongs in view models
impl AttributeService {
    pub fn can_increment(conn: &Connection, character_id: i64, attr_id: i64) -> bool {
        // Don't put this here - it belongs in CharacterView
    }
}
```

### 2. Connection Passing

Services receive `&Connection` as a parameter, not as a stored field. This allows the caller to manage transactions.

```rust
// Good - connection passed in
impl SkillService {
    pub fn get_by_id(conn: &Connection, id: i64) -> Result<Option<SkillView>> {
        // ...
    }
}

// Bad - don't store connections
pub struct SkillService {
    conn: Connection,  // Don't do this
}
```

### 3. Services Can Use Other Services

When loading shared concepts like modifiers or requirements, use dedicated services rather than duplicating repository logic:

```rust
impl EdgeService {
    pub fn get_by_id(conn: &Connection, id: i64) -> Result<Option<EdgeView>> {
        let Some(edge) = EdgeRepository::get_by_id(conn, id)? else {
            return Ok(None);
        };

        // Use other services for shared concepts
        let modifiers = ModifierService::get_for_edge(conn, id)?;
        let requirements = RequirementService::get_for_edge(conn, id)?;

        Ok(Some(EdgeView::new(edge, modifiers, requirements)))
    }
}
```

This keeps loading logic for complex types (like `RequirementTree`) in one place and ensures consistency across edges, hindrances, powers, and ancestries.

### 4. Assemble Related Data

When a view model needs data from multiple tables, the service coordinates those loads:

```rust
impl CharacterService {
    pub fn get_by_id(conn: &Connection, id: i64) -> Result<Option<CharacterView>> {
        let Some(character) = CharacterRepository::get_by_id(conn, id)? else {
            return Ok(None);
        };

        // Load related data using internal helper functions
        let attributes = character_load::load_attributes(conn, id)?;
        let skills = character_load::load_skills(conn, id)?;
        let edges = character_load::load_edges(conn, id)?;
        let hindrances = character_load::load_hindrances(conn, id)?;
        // ... assemble CharacterView
    }
}
```

### 5. Compute Derived Values

Services compute values that require combining data from multiple sources:

```rust
impl CharacterService {
    fn load_character_attributes(
        conn: &Connection,
        character_id: i64,
    ) -> Result<Vec<CharacterAttributeValue>> {
        let character_attributes = CharacterAttributeRepository::get_by_character_id(conn, character_id)?;
        let attributes = AttributeRepository::get_all(conn)?;

        let mut result = Vec::new();
        for ca in character_attributes {
            let attribute = attributes.iter().find(|a| a.id == ca.attribute_id).unwrap();
            let attribute_view = AttributeView::new(attribute.clone());

            // Compute die from base + steps
            let mut die = attribute_view.base_die;
            for _ in 0..ca.steps_incremented {
                die = die.increment();
            }

            result.push(CharacterAttributeValue::new(attribute_view, die));
        }
        Ok(result)
    }
}
```

### 6. Start Minimal

Like view models, add service methods as the presentation layer needs them. Don't build a complete CRUD interface upfront.

```rust
// Start with what you need
impl AttributeService {
    pub fn get_all(conn: &Connection) -> Result<Vec<AttributeView>> {
        // ...
    }
}

// Add more methods when needed
impl AttributeService {
    pub fn get_by_id(conn: &Connection, id: i64) -> Result<Option<AttributeView>> {
        // Added when the UI needed to load a single attribute
    }
}
```

## Implementation Guidelines

### Basic Structure

```rust
use rusqlite::Connection;
use crate::error::Result;
use crate::models::Attribute;
use crate::repositories::AttributeRepository;
use crate::views::AttributeView;

pub struct AttributeService;

impl AttributeService {
    pub fn get_all(conn: &Connection) -> Result<Vec<AttributeView>> {
        let attributes = AttributeRepository::get_all(conn)?;
        Ok(attributes.into_iter().map(AttributeView::new).collect())
    }

    pub fn get_by_id(conn: &Connection, id: i64) -> Result<Option<AttributeView>> {
        let attribute = AttributeRepository::get_by_id(conn, id)?;
        Ok(attribute.map(AttributeView::new))
    }
}
```

### Loading Related Data

Services use other services to load shared data types:

```rust
impl EdgeService {
    pub fn get_by_id(conn: &Connection, id: i64) -> Result<Option<EdgeView>> {
        let Some(edge) = EdgeRepository::get_by_id(conn, id)? else {
            return Ok(None);
        };

        let modifiers = ModifierService::get_for_edge(conn, id)?;
        let requirements = RequirementService::get_for_edge(conn, id)?;

        Ok(Some(EdgeView::new(edge, modifiers, requirements)))
    }

    pub fn get_all(conn: &Connection) -> Result<Vec<EdgeView>> {
        let edges = EdgeRepository::get_all(conn)?;

        edges
            .into_iter()
            .map(|edge| {
                let modifiers = ModifierService::get_for_edge(conn, edge.id)?;
                let requirements = RequirementService::get_for_edge(conn, edge.id)?;
                Ok(EdgeView::new(edge, modifiers, requirements))
            })
            .collect()
    }
}
```

### Naming Conventions

- File names: `{entity}_service.rs` (e.g., `attribute_service.rs`)
- Struct names: `{Entity}Service` (e.g., `AttributeService`)
- Methods: `get_by_id`, `get_all`, `get_by_{field}`

## What Belongs Here

- Loading base models from repositories
- Assembling view models from components
- Computing derived values (die from base + steps)
- Coordinating multiple repository calls

## What Does NOT Belong Here

- Business logic → view models (exception: `AdvancementService` for cross-entity coordination)
- Validation → view models (exception: `AdvancementService` for advancement rules)
- Game rules → view models
- Database queries → repositories
- HTTP/CLI handling → presentation layer

## Available Services

- **AttributeService** - Loads attributes into `AttributeView`
  - `get_all`, `get_by_id`, `get_by_name` (case-insensitive)
- **SkillService** - Loads skills into `SkillView`
  - `get_all`, `get_by_id`, `get_by_name` (case-insensitive)
- **EdgeService** - Loads edges with modifiers and requirements
  - `get_all`, `get_by_id`
- **HindranceService** - Loads hindrances with modifiers and requirements
  - `get_all`, `get_by_id`
- **PowerService** - Loads powers with modifiers and requirements
  - `get_all`, `get_by_id`
- **AncestryService** - Loads ancestries with choices, modifiers, and requirements
  - `get_all`, `get_by_id`
- **ArcaneBackgroundService** - Loads arcane backgrounds with skill names and requirements
  - `get_all`, `get_by_id`
- **GearService** - Loads gear items with categories, modifiers, requirements, and type-specific stats (weapon, armor, shield, ammunition)
  - `get_all`, `get_by_id`, `get_by_category_id`, `get_all_categories`
- **ModifierService** - Shared service for loading modifiers (used by edges, hindrances, ancestries, powers, gear)
  - `get_for_edge`, `get_for_hindrance`, `get_for_ancestry`, `get_for_power`, `get_for_gear`
- **RequirementService** - Shared service for loading requirement trees (used by edges, hindrances, ancestries, arcane backgrounds, powers, gear)
  - `get_for_edge`, `get_for_hindrance`, `get_for_ancestry`, `get_for_arcane_background`, `get_for_power`, `get_for_gear`
- **CharacterService** - Loads and saves complete characters; uses internal `character_load` and `character_sync` modules
  - `get_all`, `get_by_id` - load existing characters
  - `build_new` - create a new character in memory with defaults (does not persist)
  - `save` - persist a character (insert or update)
- **AdvancementService** - Handles character advancement (edges, attributes, skills, hindrances) with validation
  - `get_advancement_options` - get available advancement choices for a character
  - `apply_edge_advance`, `apply_attribute_advance`, `apply_expensive_skill_advance`, `apply_cheap_skill_advance`, `apply_hindrance_advance` - apply specific advancement types
  - `undo_advance` - revert the most recent advance
  - `get_advancement_history` - get list of all advances taken

## Testing

Service tests verify correct orchestration and assembly. Use the shared test utilities module:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::setup_test_db;

    fn insert_test_attribute(conn: &Connection, id: i64, name: &str, base_value: i64) {
        conn.execute(
            "INSERT INTO attributes (id, name, description, base_value, created_at, updated_at)
             VALUES (?, ?, 'Test', ?, '2024-01-01', '2024-01-01')",
            params![id, name, base_value],
        ).unwrap();
    }

    #[test]
    fn get_all_returns_attribute_views() {
        let conn = setup_test_db();
        insert_test_attribute(&conn, 1, "Agility", 4);
        insert_test_attribute(&conn, 2, "Smarts", 4);

        let views = AttributeService::get_all(&conn).unwrap();

        assert_eq!(views.len(), 2);
        assert_eq!(views[0].name, "Agility");
        assert_eq!(views[0].base_die, Die::d4());
    }

    #[test]
    fn get_by_id_returns_none_when_not_found() {
        let conn = setup_test_db();

        let result = AttributeService::get_by_id(&conn, 999).unwrap();

        assert!(result.is_none());
    }
}
```

### Test Categories

1. **Assembly tests** - Verify view models are built correctly from base models
2. **Related data tests** - Verify related data is loaded and included
3. **Computation tests** - Verify derived values are calculated correctly
4. **Not found tests** - Verify None/empty results for missing data
