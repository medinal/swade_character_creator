use crate::error::Result;
use crate::models::HindranceModifier;
use super::base_repository::{query_one_by_id, query_by_fk_id};
use rusqlite::{Connection, Row};

pub struct HindranceModifierRepository;

impl HindranceModifierRepository {
    const TABLE: &'static str = "hindrance_modifiers";
    const COLUMNS: &'static str = "id, hindrance_id, modifier_id, created_at, updated_at";

    /// Get a single hindrance modifier by ID
    pub fn get_by_id(conn: &Connection, id: i64) -> Result<Option<HindranceModifier>> {
        query_one_by_id(conn, Self::TABLE, Self::COLUMNS, id, Self::row_to_model)
    }

    /// Get all modifiers for a specific hindrance
    pub fn get_by_hindrance_id(conn: &Connection, hindrance_id: i64) -> Result<Vec<HindranceModifier>> {
        query_by_fk_id(conn, Self::TABLE, Self::COLUMNS, "hindrance_id", hindrance_id, "id", Self::row_to_model)
    }

    /// Convert a database row to a HindranceModifier model
    fn row_to_model(row: &Row) -> rusqlite::Result<HindranceModifier> {
        Ok(HindranceModifier {
            id: row.get(0)?,
            hindrance_id: row.get(1)?,
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

    fn insert_test_hindrance(
        conn: &Connection,
        id: i64,
        name: &str,
        severity: &str,
        point_value: i64,
    ) {
        conn.execute(
            "INSERT INTO hindrances (id, name, severity, point_value, source, description,
                                    created_at, updated_at)
             VALUES (?, ?, ?, ?, 'core', 'Test description',
                     '2024-01-01 00:00:00', '2024-01-01 00:00:00')",
            params![id, name, severity, point_value],
        )
        .unwrap();
    }

    fn insert_test_modifier(conn: &Connection, id: i64, target_type: &str, value_type: &str) {
        conn.execute(
            "INSERT INTO modifiers (id, target_type, value_type, description,
                                   created_at, updated_at)
             VALUES (?, ?, ?, 'Test modifier description',
                     '2024-01-01 00:00:00', '2024-01-01 00:00:00')",
            params![id, target_type, value_type],
        )
        .unwrap();
    }

    fn insert_test_hindrance_modifier(
        conn: &Connection,
        id: i64,
        hindrance_id: i64,
        modifier_id: i64,
    ) {
        conn.execute(
            "INSERT INTO hindrance_modifiers (id, hindrance_id, modifier_id,
                                             created_at, updated_at)
             VALUES (?, ?, ?, '2024-01-01 00:00:00', '2024-01-01 00:00:00')",
            params![id, hindrance_id, modifier_id],
        )
        .unwrap();
    }

    #[test]
    fn test_get_by_id_found() {
        let conn = setup_test_db();
        insert_test_hindrance(&conn, 1, "Blind", "major", 2);
        insert_test_modifier(&conn, 10, "skill", "roll_bonus");
        insert_test_hindrance_modifier(&conn, 100, 1, 10);

        let result = HindranceModifierRepository::get_by_id(&conn, 100).unwrap();

        assert!(result.is_some());
        let hm = result.unwrap();
        assert_eq!(hm.id, 100);
        assert_eq!(hm.hindrance_id, 1);
        assert_eq!(hm.modifier_id, 10);
        assert_eq!(hm.created_at, "2024-01-01 00:00:00");
        assert_eq!(hm.updated_at, "2024-01-01 00:00:00");
    }

    #[test]
    fn test_get_by_id_not_found() {
        let conn = setup_test_db();

        let result = HindranceModifierRepository::get_by_id(&conn, 999).unwrap();

        assert!(result.is_none());
    }

    #[test]
    fn test_get_by_hindrance_id_empty() {
        let conn = setup_test_db();
        insert_test_hindrance(&conn, 1, "Blind", "major", 2);

        let modifiers = HindranceModifierRepository::get_by_hindrance_id(&conn, 1).unwrap();

        assert_eq!(modifiers.len(), 0);
    }

    #[test]
    fn test_get_by_hindrance_id_single_modifier() {
        let conn = setup_test_db();
        insert_test_hindrance(&conn, 1, "Blind", "major", 2);
        insert_test_modifier(&conn, 10, "skill", "roll_bonus");
        insert_test_hindrance_modifier(&conn, 100, 1, 10);

        let modifiers = HindranceModifierRepository::get_by_hindrance_id(&conn, 1).unwrap();

        assert_eq!(modifiers.len(), 1);
        assert_eq!(modifiers[0].hindrance_id, 1);
        assert_eq!(modifiers[0].modifier_id, 10);
    }

    #[test]
    fn test_get_by_hindrance_id_multiple_modifiers() {
        let conn = setup_test_db();
        insert_test_hindrance(&conn, 1, "Blind", "major", 2);
        insert_test_modifier(&conn, 10, "skill", "roll_bonus");
        insert_test_modifier(&conn, 11, "attribute", "flat_bonus");
        insert_test_modifier(&conn, 12, "derived_stat", "description");
        insert_test_hindrance_modifier(&conn, 100, 1, 10);
        insert_test_hindrance_modifier(&conn, 101, 1, 11);
        insert_test_hindrance_modifier(&conn, 102, 1, 12);

        let modifiers = HindranceModifierRepository::get_by_hindrance_id(&conn, 1).unwrap();

        assert_eq!(modifiers.len(), 3);
        assert_eq!(modifiers[0].id, 100);
        assert_eq!(modifiers[0].modifier_id, 10);
        assert_eq!(modifiers[1].id, 101);
        assert_eq!(modifiers[1].modifier_id, 11);
        assert_eq!(modifiers[2].id, 102);
        assert_eq!(modifiers[2].modifier_id, 12);
    }

    #[test]
    fn test_get_by_hindrance_id_filters_correctly() {
        let conn = setup_test_db();
        insert_test_hindrance(&conn, 1, "Blind", "major", 2);
        insert_test_hindrance(&conn, 2, "Deaf", "major", 2);
        insert_test_modifier(&conn, 10, "skill", "roll_bonus");
        insert_test_modifier(&conn, 11, "attribute", "flat_bonus");
        insert_test_hindrance_modifier(&conn, 100, 1, 10);
        insert_test_hindrance_modifier(&conn, 101, 2, 11);

        let modifiers = HindranceModifierRepository::get_by_hindrance_id(&conn, 1).unwrap();

        assert_eq!(modifiers.len(), 1);
        assert_eq!(modifiers[0].hindrance_id, 1);
        assert_eq!(modifiers[0].modifier_id, 10);
    }

    #[test]
    fn test_get_by_hindrance_id_not_found() {
        let conn = setup_test_db();

        let modifiers = HindranceModifierRepository::get_by_hindrance_id(&conn, 999).unwrap();

        assert_eq!(modifiers.len(), 0);
    }

    #[test]
    fn test_row_to_model_all_fields() {
        let conn = setup_test_db();
        insert_test_hindrance(&conn, 5, "Test Hindrance", "minor", 1);
        insert_test_modifier(&conn, 20, "skill", "die_increment");
        conn.execute(
            "INSERT INTO hindrance_modifiers (id, hindrance_id, modifier_id,
                                             created_at, updated_at)
             VALUES (200, 5, 20, '2024-12-25 10:30:00', '2024-12-26 15:45:00')",
            [],
        )
        .unwrap();

        let hm = HindranceModifierRepository::get_by_id(&conn, 200)
            .unwrap()
            .unwrap();

        assert_eq!(hm.id, 200);
        assert_eq!(hm.hindrance_id, 5);
        assert_eq!(hm.modifier_id, 20);
        assert_eq!(hm.created_at, "2024-12-25 10:30:00");
        assert_eq!(hm.updated_at, "2024-12-26 15:45:00");
    }
}
