use crate::error::Result;
use crate::models::AncestryModifier;
use super::base_repository::{query_one_by_id, query_by_fk_id};
use rusqlite::{Connection, Row};

pub struct AncestryModifierRepository;

impl AncestryModifierRepository {
    const TABLE: &'static str = "ancestry_modifiers";
    const COLUMNS: &'static str = "id, ancestry_id, modifier_id, created_at, updated_at";

    /// Get a single ancestry modifier by ID
    pub fn get_by_id(conn: &Connection, id: i64) -> Result<Option<AncestryModifier>> {
        query_one_by_id(conn, Self::TABLE, Self::COLUMNS, id, Self::row_to_model)
    }

    /// Get all ancestry modifiers for a specific ancestry
    pub fn get_by_ancestry_id(conn: &Connection, ancestry_id: i64) -> Result<Vec<AncestryModifier>> {
        query_by_fk_id(conn, Self::TABLE, Self::COLUMNS, "ancestry_id", ancestry_id, "id", Self::row_to_model)
    }

    /// Convert a database row to an AncestryModifier model
    fn row_to_model(row: &Row) -> rusqlite::Result<AncestryModifier> {
        Ok(AncestryModifier {
            id: row.get(0)?,
            ancestry_id: row.get(1)?,
            modifier_id: row.get(2)?,
            created_at: row.get(3)?,
            updated_at: row.get(4)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::{Connection, params};

    fn setup_test_db() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        let schema = include_str!("../../../db/schema.sql");
        conn.execute_batch(schema).unwrap();
        conn
    }

    fn insert_test_ancestry(conn: &Connection, id: i64, name: &str) {
        conn.execute(
            "INSERT INTO ancestries (id, name, source, description, created_at, updated_at)
             VALUES (?, ?, 'core', 'Test ancestry', '2024-01-01 00:00:00', '2024-01-01 00:00:00')",
            params![id, name],
        )
        .unwrap();
    }

    fn insert_test_modifier(conn: &Connection, id: i64, description: &str) {
        conn.execute(
            "INSERT INTO modifiers (id, target_type, target_identifier, value_type, value,
                                   description, created_at, updated_at)
             VALUES (?, 'attribute', 'strength', 'flat_bonus', 1,
                     ?, '2024-01-01 00:00:00', '2024-01-01 00:00:00')",
            params![id, description],
        )
        .unwrap();
    }

    fn insert_test_ancestry_modifier(
        conn: &Connection,
        id: i64,
        ancestry_id: i64,
        modifier_id: i64,
    ) {
        conn.execute(
            "INSERT INTO ancestry_modifiers (id, ancestry_id, modifier_id, created_at, updated_at)
             VALUES (?, ?, ?, '2024-01-01 00:00:00', '2024-01-01 00:00:00')",
            params![id, ancestry_id, modifier_id],
        )
        .unwrap();
    }

    #[test]
    fn test_get_by_id_found() {
        let conn = setup_test_db();
        insert_test_ancestry(&conn, 1, "Human");
        insert_test_modifier(&conn, 1, "Bonus Edge");
        insert_test_ancestry_modifier(&conn, 1, 1, 1);

        let result = AncestryModifierRepository::get_by_id(&conn, 1).unwrap();

        assert!(result.is_some());
        let ancestry_modifier = result.unwrap();
        assert_eq!(ancestry_modifier.id, 1);
        assert_eq!(ancestry_modifier.ancestry_id, 1);
        assert_eq!(ancestry_modifier.modifier_id, 1);
        assert_eq!(ancestry_modifier.created_at, "2024-01-01 00:00:00");
        assert_eq!(ancestry_modifier.updated_at, "2024-01-01 00:00:00");
    }

    #[test]
    fn test_get_by_id_not_found() {
        let conn = setup_test_db();

        let result = AncestryModifierRepository::get_by_id(&conn, 999).unwrap();

        assert!(result.is_none());
    }

    #[test]
    fn test_get_by_ancestry_id_empty() {
        let conn = setup_test_db();
        insert_test_ancestry(&conn, 1, "Human");

        let modifiers = AncestryModifierRepository::get_by_ancestry_id(&conn, 1).unwrap();

        assert_eq!(modifiers.len(), 0);
    }

    #[test]
    fn test_get_by_ancestry_id_single() {
        let conn = setup_test_db();
        insert_test_ancestry(&conn, 1, "Human");
        insert_test_modifier(&conn, 1, "Bonus Edge");
        insert_test_ancestry_modifier(&conn, 1, 1, 1);

        let modifiers = AncestryModifierRepository::get_by_ancestry_id(&conn, 1).unwrap();

        assert_eq!(modifiers.len(), 1);
        assert_eq!(modifiers[0].ancestry_id, 1);
        assert_eq!(modifiers[0].modifier_id, 1);
    }

    #[test]
    fn test_get_by_ancestry_id_multiple() {
        let conn = setup_test_db();
        insert_test_ancestry(&conn, 1, "Human");
        insert_test_modifier(&conn, 1, "Bonus Edge");
        insert_test_modifier(&conn, 2, "Adaptable");
        insert_test_modifier(&conn, 3, "Extra Skills");
        insert_test_ancestry_modifier(&conn, 1, 1, 1);
        insert_test_ancestry_modifier(&conn, 2, 1, 2);
        insert_test_ancestry_modifier(&conn, 3, 1, 3);

        let modifiers = AncestryModifierRepository::get_by_ancestry_id(&conn, 1).unwrap();

        assert_eq!(modifiers.len(), 3);
        assert_eq!(modifiers[0].modifier_id, 1);
        assert_eq!(modifiers[1].modifier_id, 2);
        assert_eq!(modifiers[2].modifier_id, 3);
    }

    #[test]
    fn test_get_by_ancestry_id_filters_correctly() {
        let conn = setup_test_db();
        insert_test_ancestry(&conn, 1, "Human");
        insert_test_ancestry(&conn, 2, "Dwarf");
        insert_test_modifier(&conn, 1, "Bonus Edge");
        insert_test_modifier(&conn, 2, "Hardy");
        insert_test_ancestry_modifier(&conn, 1, 1, 1);
        insert_test_ancestry_modifier(&conn, 2, 2, 2);

        let modifiers = AncestryModifierRepository::get_by_ancestry_id(&conn, 1).unwrap();

        assert_eq!(modifiers.len(), 1);
        assert_eq!(modifiers[0].ancestry_id, 1);
        assert_eq!(modifiers[0].modifier_id, 1);
    }

    #[test]
    fn test_get_by_ancestry_id_not_found() {
        let conn = setup_test_db();
        insert_test_ancestry(&conn, 1, "Human");
        insert_test_modifier(&conn, 1, "Bonus Edge");
        insert_test_ancestry_modifier(&conn, 1, 1, 1);

        let modifiers = AncestryModifierRepository::get_by_ancestry_id(&conn, 999).unwrap();

        assert_eq!(modifiers.len(), 0);
    }

    #[test]
    fn test_row_to_model_all_fields() {
        let conn = setup_test_db();
        insert_test_ancestry(&conn, 42, "Test Ancestry");
        insert_test_modifier(&conn, 99, "Test Modifier");
        conn.execute(
            "INSERT INTO ancestry_modifiers (id, ancestry_id, modifier_id, created_at, updated_at)
             VALUES (123, 42, 99, '2024-12-25 10:30:00', '2024-12-26 15:45:00')",
            [],
        )
        .unwrap();

        let ancestry_modifier = AncestryModifierRepository::get_by_id(&conn, 123)
            .unwrap()
            .unwrap();

        assert_eq!(ancestry_modifier.id, 123);
        assert_eq!(ancestry_modifier.ancestry_id, 42);
        assert_eq!(ancestry_modifier.modifier_id, 99);
        assert_eq!(ancestry_modifier.created_at, "2024-12-25 10:30:00");
        assert_eq!(ancestry_modifier.updated_at, "2024-12-26 15:45:00");
    }

    #[test]
    fn test_get_by_ancestry_id_ordered_by_id() {
        let conn = setup_test_db();
        insert_test_ancestry(&conn, 1, "Human");
        insert_test_modifier(&conn, 1, "Modifier A");
        insert_test_modifier(&conn, 2, "Modifier B");
        insert_test_modifier(&conn, 3, "Modifier C");
        // Insert in reverse order to test ordering
        insert_test_ancestry_modifier(&conn, 3, 1, 3);
        insert_test_ancestry_modifier(&conn, 1, 1, 1);
        insert_test_ancestry_modifier(&conn, 2, 1, 2);

        let modifiers = AncestryModifierRepository::get_by_ancestry_id(&conn, 1).unwrap();

        assert_eq!(modifiers.len(), 3);
        // Should be ordered by id
        assert_eq!(modifiers[0].id, 1);
        assert_eq!(modifiers[1].id, 2);
        assert_eq!(modifiers[2].id, 3);
    }
}
