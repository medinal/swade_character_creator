use crate::error::Result;
use crate::models::Hindrance;
use super::base_repository::{query_one_by_id, query_all_ordered};
use rusqlite::{Connection, Row};

pub struct HindranceRepository;

impl HindranceRepository {
    const TABLE: &'static str = "hindrances";
    const COLUMNS: &'static str = "id, name, severity, point_value, companion_hindrance_id,
                                   source, description, created_at, updated_at";

    pub fn get_by_id(conn: &Connection, id: i64) -> Result<Option<Hindrance>> {
        query_one_by_id(conn, Self::TABLE, Self::COLUMNS, id, Self::row_to_model)
    }

    pub fn get_all(conn: &Connection) -> Result<Vec<Hindrance>> {
        query_all_ordered(conn, Self::TABLE, Self::COLUMNS, "name", Self::row_to_model)
    }

    fn row_to_model(row: &Row) -> rusqlite::Result<Hindrance> {
        Ok(Hindrance {
            id: row.get(0)?,
            name: row.get(1)?,
            severity: row.get(2)?,
            point_value: row.get(3)?,
            companion_hindrance_id: row.get(4)?,
            source: row.get(5)?,
            description: row.get(6)?,
            created_at: row.get(7)?,
            updated_at: row.get(8)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::setup_test_db;
    use rusqlite::params;

    // Local helper with custom signature for hindrance-specific tests
    fn insert_test_hindrance(
        conn: &Connection,
        id: i64,
        name: &str,
        severity: &str,
        point_value: i64,
    ) {
        conn.execute(
            "INSERT INTO hindrances (id, name, severity, point_value, companion_hindrance_id,
                                   source, description, created_at, updated_at)
             VALUES (?, ?, ?, ?, NULL, 'core', 'Test description', '2024-01-01', '2024-01-01')",
            params![id, name, severity, point_value],
        )
        .unwrap();
    }

    fn insert_test_hindrance_with_companion(
        conn: &Connection,
        id: i64,
        name: &str,
        severity: &str,
        point_value: i64,
        companion_id: i64,
    ) {
        conn.execute(
            "INSERT INTO hindrances (id, name, severity, point_value, companion_hindrance_id,
                                   source, description, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, 'core', 'Test description', '2024-01-01', '2024-01-01')",
            params![id, name, severity, point_value, companion_id],
        )
        .unwrap();
    }

    #[test]
    fn test_get_by_id_found() {
        let conn = setup_test_db();
        insert_test_hindrance(&conn, 1, "All Thumbs", "minor", 1);

        let result = HindranceRepository::get_by_id(&conn, 1).unwrap();

        assert!(result.is_some());
        let hindrance = result.unwrap();
        assert_eq!(hindrance.id, 1);
        assert_eq!(hindrance.name, "All Thumbs");
        assert_eq!(hindrance.severity, "minor");
        assert_eq!(hindrance.point_value, 1);
        assert_eq!(hindrance.companion_hindrance_id, None);
        assert_eq!(hindrance.source, "core");
        assert_eq!(hindrance.description, "Test description");
    }

    #[test]
    fn test_get_by_id_not_found() {
        let conn = setup_test_db();

        let result = HindranceRepository::get_by_id(&conn, 999).unwrap();

        assert!(result.is_none());
    }

    #[test]
    fn test_get_by_id_with_companion() {
        let conn = setup_test_db();
        insert_test_hindrance(&conn, 1, "Bad Eyes", "minor", 1);
        insert_test_hindrance_with_companion(&conn, 2, "Bad Eyes", "major", 2, 1);

        let result = HindranceRepository::get_by_id(&conn, 2).unwrap();

        assert!(result.is_some());
        let hindrance = result.unwrap();
        assert_eq!(hindrance.name, "Bad Eyes");
        assert_eq!(hindrance.severity, "major");
        assert_eq!(hindrance.point_value, 2);
        assert_eq!(hindrance.companion_hindrance_id, Some(1));
    }

    #[test]
    fn test_get_all_empty() {
        let conn = setup_test_db();

        let hindrances = HindranceRepository::get_all(&conn).unwrap();

        assert_eq!(hindrances.len(), 0);
    }

    #[test]
    fn test_get_all_multiple() {
        let conn = setup_test_db();
        insert_test_hindrance(&conn, 1, "All Thumbs", "minor", 1);
        insert_test_hindrance(&conn, 2, "Arrogant", "major", 2);
        insert_test_hindrance(&conn, 3, "Bad Luck", "major", 2);

        let hindrances = HindranceRepository::get_all(&conn).unwrap();

        assert_eq!(hindrances.len(), 3);
        // Verify ordering by name
        assert_eq!(hindrances[0].name, "All Thumbs");
        assert_eq!(hindrances[1].name, "Arrogant");
        assert_eq!(hindrances[2].name, "Bad Luck");
    }

    #[test]
    fn test_get_all_ordering() {
        let conn = setup_test_db();
        // Insert in non-alphabetical order
        insert_test_hindrance(&conn, 1, "Yellow", "minor", 1);
        insert_test_hindrance(&conn, 2, "Anemic", "minor", 1);
        insert_test_hindrance(&conn, 3, "Mild Mannered", "minor", 1);

        let hindrances = HindranceRepository::get_all(&conn).unwrap();

        assert_eq!(hindrances.len(), 3);
        // Should be ordered alphabetically
        assert_eq!(hindrances[0].name, "Anemic");
        assert_eq!(hindrances[1].name, "Mild Mannered");
        assert_eq!(hindrances[2].name, "Yellow");
    }

    #[test]
    fn test_row_to_model_all_fields() {
        let conn = setup_test_db();
        conn.execute(
            "INSERT INTO hindrances (id, name, severity, point_value, companion_hindrance_id,
                                   source, description, created_at, updated_at)
             VALUES (1, 'Test Hindrance', 'major', 2, NULL, 'test_source',
                    'Detailed test description', '2024-06-15 10:30:00', '2024-06-20 14:45:00')",
            [],
        )
        .unwrap();

        let result = HindranceRepository::get_by_id(&conn, 1).unwrap();

        assert!(result.is_some());
        let hindrance = result.unwrap();
        assert_eq!(hindrance.id, 1);
        assert_eq!(hindrance.name, "Test Hindrance");
        assert_eq!(hindrance.severity, "major");
        assert_eq!(hindrance.point_value, 2);
        assert_eq!(hindrance.companion_hindrance_id, None);
        assert_eq!(hindrance.source, "test_source");
        assert_eq!(hindrance.description, "Detailed test description");
        assert_eq!(hindrance.created_at, "2024-06-15 10:30:00");
        assert_eq!(hindrance.updated_at, "2024-06-20 14:45:00");
    }
}
