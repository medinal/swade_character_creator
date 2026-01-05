# Repositories

Repositories handle all database access. They provide CRUD operations for models and are the only layer that interacts directly with SQLite.

## Purpose

Repositories are the data access layer, responsible for:

- Loading models from the database
- Saving models to the database
- Querying by ID, name, or other criteria
- Managing database transactions

They contain **no business logic** - just SQL queries and model mapping.

## Architecture Role

```
Database → Repository → Model → Service → View Model → Presentation
              ↑
         You are here
```

Repositories receive a database connection, execute queries, and return models. Services call repositories to load data, then assemble view models with business logic.

## Base Repository Helpers

The `base_repository.rs` module provides helper functions to reduce boilerplate across repositories:

- `query_one_by_id` - Fetch a single record by ID
- `query_all_ordered` - Fetch all records with ordering
- `query_one_by_field` - Fetch a single record by a string field (case-insensitive)
- `query_where` - Query records with a WHERE clause
- `query_by_fk_id` - Query multiple records by a foreign key ID

Each repository still implements its own `row_to_model` function since that's specific to each model's fields.

## Implementation Guidelines

### Basic Structure

Repositories use helper functions from `base_repository.rs` to reduce boilerplate:

```rust
use crate::error::Result;
use crate::models::Edge;
use super::base_repository::{query_one_by_id, query_all_ordered};
use rusqlite::{Connection, Row};

pub struct EdgeRepository;

impl EdgeRepository {
    const TABLE: &'static str = "edges";
    const COLUMNS: &'static str = "id, name, background, source, description,
                                   can_take_multiple_times, created_at, updated_at";

    pub fn get_by_id(conn: &Connection, id: i64) -> Result<Option<Edge>> {
        query_one_by_id(conn, Self::TABLE, Self::COLUMNS, id, Self::row_to_model)
    }

    pub fn get_all(conn: &Connection) -> Result<Vec<Edge>> {
        query_all_ordered(conn, Self::TABLE, Self::COLUMNS, "name", Self::row_to_model)
    }

    fn row_to_model(row: &Row) -> rusqlite::Result<Edge> {
        Ok(Edge {
            id: row.get(0)?,
            name: row.get(1)?,
            background: row.get(2)?,
            source: row.get(3)?,
            description: row.get(4)?,
            can_take_multiple_times: row.get(5)?,
            created_at: row.get(6)?,
            updated_at: row.get(7)?,
        })
    }
}
```

### Standard Methods

Each repository should implement these methods as appropriate:

#### Read Operations (All Repositories)

```rust
// Get single record by ID
fn get_by_id(conn: &Connection, id: i64) -> Result<Option<Model>>

// Get all records
fn get_all(conn: &Connection) -> Result<Vec<Model>>
```

#### Write Operations (Character Tables Only)

Seed data tables are protected by triggers. Only character tables need write operations:

```rust
// Insert new record, return generated ID
fn insert(conn: &Connection, model: &Model) -> Result<i64>

// Update existing record
fn update(conn: &Connection, model: &Model) -> Result<()>

// Delete record
fn delete(conn: &Connection, id: i64) -> Result<()>
```

#### Junction Table Operations

Junction tables link entities together:

```rust
// Get all modifiers for an edge
fn get_by_edge_id(conn: &Connection, edge_id: i64) -> Result<Vec<EdgeModifier>>

// Get all edges for a character
fn get_by_character_id(conn: &Connection, character_id: i64) -> Result<Vec<CharacterEdge>>
```

### Row Mapping

Use a private `row_to_model` function to convert database rows to models:

```rust
fn row_to_model(row: &Row) -> rusqlite::Result<Model> {
    Ok(Model {
        id: row.get(0)?,
        field1: row.get(1)?,
        field2: row.get(2)?,
        // ... positional indexing matches SELECT column order
    })
}
```

### Handling Nullable Fields

For `Option<T>` fields in models, rusqlite handles NULL automatically:

```rust
fn row_to_model(row: &Row) -> rusqlite::Result<Hindrance> {
    Ok(Hindrance {
        id: row.get(0)?,
        name: row.get(1)?,
        companion_hindrance_id: row.get(4)?,  // Option<i64> - NULL becomes None
        // ...
    })
}
```

### Connection Passing

Repositories don't own connections - they receive `&Connection` as a parameter:

```rust
// Good - connection passed in
pub fn get_by_id(conn: &Connection, id: i64) -> Result<Option<Edge>>

// Bad - don't store connections in repositories
pub struct EdgeRepository {
    conn: Connection,  // Don't do this
}
```

This allows services to manage transactions across multiple repository calls.

## What Belongs Here

- SQL queries (SELECT, INSERT, UPDATE, DELETE)
- Row-to-model mapping
- Query parameter binding
- Basic query filtering and ordering

## What Does NOT Belong Here

- Business logic → view models
- Validation → view models
- Data transformation → services
- Complex joins assembling view models → services
- Transaction management → services (repositories participate in transactions)

## Naming Conventions

- File names: `{model}_repo.rs` (e.g., `edge_repo.rs`)
- Struct names: `{Model}Repository` (e.g., `EdgeRepository`)
- Methods: `get_by_{field}`, `get_all`, `insert`, `update`, `delete`

## Testing

Repository tests verify SQL correctness and row mapping. Use the shared `test_utils` module which provides:

- `setup_test_db()` - Creates an in-memory SQLite database with the real schema
- `setup_test_db_no_fk()` - Same but with foreign key constraints disabled
- `setup_test_db_with_fk()` - Explicitly enables foreign key constraints
- Helper functions for inserting common test data (e.g., `insert_test_edge`, `insert_test_character`)

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::setup_test_db;
    use rusqlite::params;

    // Local helper with custom signature for edge-specific tests
    fn insert_test_edge(
        conn: &Connection,
        id: i64,
        name: &str,
        background: &str,
        can_take_multiple_times: bool,
    ) {
        conn.execute(
            "INSERT INTO edges (id, name, background, source, description,
                               can_take_multiple_times, created_at, updated_at)
             VALUES (?, ?, ?, 'core', 'Test description',
                     ?, '2024-01-01 00:00:00', '2024-01-01 00:00:00')",
            params![id, name, background, can_take_multiple_times],
        )
        .unwrap();
    }

    #[test]
    fn test_get_by_id_found() {
        let conn = setup_test_db();
        insert_test_edge(&conn, 1, "Alertness", "Background", false);

        let result = EdgeRepository::get_by_id(&conn, 1).unwrap();

        assert!(result.is_some());
        let edge = result.unwrap();
        assert_eq!(edge.name, "Alertness");
        assert_eq!(edge.background, "Background");
    }

    #[test]
    fn test_get_by_id_not_found() {
        let conn = setup_test_db();

        let result = EdgeRepository::get_by_id(&conn, 999).unwrap();

        assert!(result.is_none());
    }

    #[test]
    fn test_get_all() {
        let conn = setup_test_db();
        insert_test_edge(&conn, 1, "Alertness", "Background", false);

        let edges = EdgeRepository::get_all(&conn).unwrap();

        assert_eq!(edges.len(), 1);
    }
}
```

### Test Categories

1. **Read tests** - Verify queries return correct data
2. **Not found tests** - Verify None/empty results for missing data
3. **Write tests** (character tables) - Verify insert/update/delete
4. **Junction tests** - Verify filtering by foreign key

## Repository Categories

### Seed Data Repositories (Read-Only)

These load immutable game rules data:

- `AttributeRepository`
- `SkillRepository`
- `RankRepository`
- `EdgeRepository`
- `HindranceRepository`
- `AncestryRepository`
- `ArcaneBackgroundRepository`
- `PowerRepository`
- `ModifierRepository`
- `RequirementRepository`
- `RequirementExpressionRepository`
- `AncestryChoiceRepository` (handles both AncestryChoice and AncestryChoiceOption)
- `GearRepository`
- `GearCategoryRepository`
- `WeaponStatsRepository`
- `ArmorStatsRepository`
- `ShieldStatsRepository`
- `AmmunitionStatsRepository`

### Junction Table Repositories (Read-Only)

These load relationships between seed data:

- `AncestryModifierRepository`
- `AncestryRequirementRepository`
- `EdgeModifierRepository`
- `EdgeRequirementRepository`
- `HindranceModifierRepository`
- `HindranceRequirementRepository`
- `PowerModifierRepository`
- `PowerRequirementRepository`
- `ArcaneBackgroundRequirementRepository`
- `GearModifierRepository`
- `GearRequirementRepository`

### Character Repositories (Read-Write)

These manage player character data:

- `CharacterRepository`
- `CharacterAdvanceRepository`
- `CharacterAncestryChoiceRepository`
- `CharacterArcaneBackgroundRepository`
- `CharacterAttributeRepository`
- `CharacterEdgeRepository`
- `CharacterGearRepository`
- `CharacterHindranceRepository`
- `CharacterModifierRepository`
- `CharacterNoteRepository`
- `CharacterPowerRepository`
- `CharacterSkillRepository`
