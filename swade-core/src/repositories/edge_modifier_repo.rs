use crate::error::Result;
use crate::models::EdgeModifier;
use super::base_repository::{query_one_by_id, query_all_ordered, query_by_fk_id};
use rusqlite::{Connection, Row};

pub struct EdgeModifierRepository;

impl EdgeModifierRepository {
    const TABLE: &'static str = "edge_modifiers";
    const COLUMNS: &'static str = "id, edge_id, modifier_id, created_at, updated_at";

    /// Get a single edge modifier by ID
    pub fn get_by_id(conn: &Connection, id: i64) -> Result<Option<EdgeModifier>> {
        query_one_by_id(conn, Self::TABLE, Self::COLUMNS, id, Self::row_to_model)
    }

    /// Get all edge modifiers
    pub fn get_all(conn: &Connection) -> Result<Vec<EdgeModifier>> {
        query_all_ordered(conn, Self::TABLE, Self::COLUMNS, "edge_id, modifier_id", Self::row_to_model)
    }

    /// Get all edge modifiers for a specific edge
    pub fn get_by_edge_id(conn: &Connection, edge_id: i64) -> Result<Vec<EdgeModifier>> {
        query_by_fk_id(conn, Self::TABLE, Self::COLUMNS, "edge_id", edge_id, "id", Self::row_to_model)
    }

    /// Convert a database row to an EdgeModifier model
    fn row_to_model(row: &Row) -> rusqlite::Result<EdgeModifier> {
        Ok(EdgeModifier {
            id: row.get(0)?,
            edge_id: row.get(1)?,
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

    fn insert_test_edge(conn: &Connection, id: i64, name: &str) {
        conn.execute(
            "INSERT INTO edges (id, name, background, source, description,
                               can_take_multiple_times, created_at, updated_at)
             VALUES (?, ?, 'Background', 'core', 'Test description',
                     0, '2024-01-01 00:00:00', '2024-01-01 00:00:00')",
            params![id, name],
        )
        .unwrap();
    }

    fn insert_test_modifier(conn: &Connection, id: i64, description: &str) {
        conn.execute(
            "INSERT INTO modifiers (id, target_type, target_identifier, value_type,
                                   value, description, created_at, updated_at)
             VALUES (?, 'attribute', 'agility', 'die_increment',
                     1, ?, '2024-01-01 00:00:00', '2024-01-01 00:00:00')",
            params![id, description],
        )
        .unwrap();
    }

    fn insert_test_edge_modifier(conn: &Connection, id: i64, edge_id: i64, modifier_id: i64) {
        conn.execute(
            "INSERT INTO edge_modifiers (id, edge_id, modifier_id, created_at, updated_at)
             VALUES (?, ?, ?, '2024-01-01 00:00:00', '2024-01-01 00:00:00')",
            params![id, edge_id, modifier_id],
        )
        .unwrap();
    }

    #[test]
    fn test_get_by_id_found() {
        let conn = setup_test_db();
        insert_test_edge(&conn, 1, "Alertness");
        insert_test_modifier(&conn, 1, "Notice Bonus");
        insert_test_edge_modifier(&conn, 1, 1, 1);

        let result = EdgeModifierRepository::get_by_id(&conn, 1).unwrap();

        assert!(result.is_some());
        let edge_modifier = result.unwrap();
        assert_eq!(edge_modifier.id, 1);
        assert_eq!(edge_modifier.edge_id, 1);
        assert_eq!(edge_modifier.modifier_id, 1);
        assert_eq!(edge_modifier.created_at, "2024-01-01 00:00:00");
        assert_eq!(edge_modifier.updated_at, "2024-01-01 00:00:00");
    }

    #[test]
    fn test_get_by_id_not_found() {
        let conn = setup_test_db();

        let result = EdgeModifierRepository::get_by_id(&conn, 999).unwrap();

        assert!(result.is_none());
    }

    #[test]
    fn test_get_all() {
        let conn = setup_test_db();
        insert_test_edge(&conn, 1, "Alertness");
        insert_test_edge(&conn, 2, "Brawny");
        insert_test_modifier(&conn, 1, "Notice Bonus");
        insert_test_modifier(&conn, 2, "Toughness Bonus");

        insert_test_edge_modifier(&conn, 1, 1, 1);
        insert_test_edge_modifier(&conn, 2, 2, 2);

        let modifiers = EdgeModifierRepository::get_all(&conn).unwrap();

        assert_eq!(modifiers.len(), 2);
    }

    #[test]
    fn test_get_all_empty() {
        let conn = setup_test_db();

        let modifiers = EdgeModifierRepository::get_all(&conn).unwrap();

        assert_eq!(modifiers.len(), 0);
    }

    #[test]
    fn test_get_by_edge_id_multiple_modifiers() {
        let conn = setup_test_db();
        insert_test_edge(&conn, 1, "Brawny");
        insert_test_modifier(&conn, 1, "Toughness Bonus");
        insert_test_modifier(&conn, 2, "Load Limit Bonus");

        insert_test_edge_modifier(&conn, 1, 1, 1);
        insert_test_edge_modifier(&conn, 2, 1, 2);

        let modifiers = EdgeModifierRepository::get_by_edge_id(&conn, 1).unwrap();

        assert_eq!(modifiers.len(), 2);
        assert_eq!(modifiers[0].edge_id, 1);
        assert_eq!(modifiers[0].modifier_id, 1);
        assert_eq!(modifiers[1].edge_id, 1);
        assert_eq!(modifiers[1].modifier_id, 2);
    }

    #[test]
    fn test_get_by_edge_id_empty() {
        let conn = setup_test_db();

        let modifiers = EdgeModifierRepository::get_by_edge_id(&conn, 999).unwrap();

        assert_eq!(modifiers.len(), 0);
    }

    #[test]
    fn test_get_by_edge_id_filters_correctly() {
        let conn = setup_test_db();
        insert_test_edge(&conn, 1, "Alertness");
        insert_test_edge(&conn, 2, "Danger Sense");
        insert_test_modifier(&conn, 1, "Notice Bonus");

        insert_test_edge_modifier(&conn, 1, 1, 1);
        insert_test_edge_modifier(&conn, 2, 2, 1);

        let modifiers = EdgeModifierRepository::get_by_edge_id(&conn, 1).unwrap();

        assert_eq!(modifiers.len(), 1);
        assert_eq!(modifiers[0].edge_id, 1);
    }

    #[test]
    fn test_row_to_model_all_fields() {
        let conn = setup_test_db();
        insert_test_edge(&conn, 42, "Test Edge");
        insert_test_modifier(&conn, 99, "Test Modifier");

        conn.execute(
            "INSERT INTO edge_modifiers (id, edge_id, modifier_id, created_at, updated_at)
             VALUES (100, 42, 99, '2024-12-25 10:30:00', '2024-12-26 15:45:00')",
            [],
        )
        .unwrap();

        let edge_modifier = EdgeModifierRepository::get_by_id(&conn, 100)
            .unwrap()
            .unwrap();

        assert_eq!(edge_modifier.id, 100);
        assert_eq!(edge_modifier.edge_id, 42);
        assert_eq!(edge_modifier.modifier_id, 99);
        assert_eq!(edge_modifier.created_at, "2024-12-25 10:30:00");
        assert_eq!(edge_modifier.updated_at, "2024-12-26 15:45:00");
    }

    #[test]
    fn test_get_by_edge_id_with_single_modifier() {
        let conn = setup_test_db();
        insert_test_edge(&conn, 1, "Wizard");
        insert_test_modifier(&conn, 1, "Power Point Bonus");
        insert_test_edge_modifier(&conn, 1, 1, 1);

        let modifiers = EdgeModifierRepository::get_by_edge_id(&conn, 1).unwrap();

        assert_eq!(modifiers.len(), 1);
        assert_eq!(modifiers[0].edge_id, 1);
        assert_eq!(modifiers[0].modifier_id, 1);
    }

    #[test]
    fn test_unique_constraint_edge_modifier_pair() {
        let conn = setup_test_db();
        insert_test_edge(&conn, 1, "Alertness");
        insert_test_modifier(&conn, 1, "Notice Bonus");
        insert_test_edge_modifier(&conn, 1, 1, 1);

        // Attempting to insert duplicate edge_id/modifier_id pair should fail
        let result = conn.execute(
            "INSERT INTO edge_modifiers (edge_id, modifier_id, created_at, updated_at)
             VALUES (1, 1, '2024-01-01 00:00:00', '2024-01-01 00:00:00')",
            [],
        );

        assert!(result.is_err());
    }
}
