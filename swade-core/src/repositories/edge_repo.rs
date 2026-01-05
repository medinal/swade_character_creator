use crate::error::Result;
use crate::models::Edge;
use super::base_repository::{query_one_by_id, query_all_ordered};
use rusqlite::{Connection, Row};

pub struct EdgeRepository;

impl EdgeRepository {
    const TABLE: &'static str = "edges";
    const COLUMNS: &'static str = "id, name, background, source, description,
                                   can_take_multiple_times, created_at, updated_at";

    /// Get a single edge by ID
    pub fn get_by_id(conn: &Connection, id: i64) -> Result<Option<Edge>> {
        query_one_by_id(conn, Self::TABLE, Self::COLUMNS, id, Self::row_to_model)
    }

    /// Get all edges ordered by name
    pub fn get_all(conn: &Connection) -> Result<Vec<Edge>> {
        query_all_ordered(conn, Self::TABLE, Self::COLUMNS, "name", Self::row_to_model)
    }

    /// Convert a database row to an Edge model
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
        assert_eq!(edge.id, 1);
        assert_eq!(edge.name, "Alertness");
        assert_eq!(edge.background, "Background");
        assert_eq!(edge.source, "core");
        assert_eq!(edge.description, "Test description");
        assert_eq!(edge.can_take_multiple_times, false);
        assert_eq!(edge.created_at, "2024-01-01 00:00:00");
        assert_eq!(edge.updated_at, "2024-01-01 00:00:00");
    }

    #[test]
    fn test_get_by_id_not_found() {
        let conn = setup_test_db();

        let result = EdgeRepository::get_by_id(&conn, 999).unwrap();

        assert!(result.is_none());
    }

    #[test]
    fn test_get_by_id_with_can_take_multiple_times() {
        let conn = setup_test_db();
        insert_test_edge(&conn, 1, "Wizard", "Power", true);

        let result = EdgeRepository::get_by_id(&conn, 1).unwrap();

        assert!(result.is_some());
        let edge = result.unwrap();
        assert_eq!(edge.name, "Wizard");
        assert_eq!(edge.can_take_multiple_times, true);
    }

    #[test]
    fn test_get_all_empty() {
        let conn = setup_test_db();

        let edges = EdgeRepository::get_all(&conn).unwrap();

        assert_eq!(edges.len(), 0);
    }

    #[test]
    fn test_get_all_single_edge() {
        let conn = setup_test_db();
        insert_test_edge(&conn, 1, "Brawny", "Background", false);

        let edges = EdgeRepository::get_all(&conn).unwrap();

        assert_eq!(edges.len(), 1);
        assert_eq!(edges[0].name, "Brawny");
    }

    #[test]
    fn test_get_all_multiple_edges_ordered_by_name() {
        let conn = setup_test_db();
        insert_test_edge(&conn, 1, "Danger Sense", "Background", false);
        insert_test_edge(&conn, 2, "Alertness", "Background", false);
        insert_test_edge(&conn, 3, "Wizard", "Power", true);

        let edges = EdgeRepository::get_all(&conn).unwrap();

        assert_eq!(edges.len(), 3);
        // Should be ordered by name alphabetically
        assert_eq!(edges[0].name, "Alertness");
        assert_eq!(edges[1].name, "Danger Sense");
        assert_eq!(edges[2].name, "Wizard");
    }

    #[test]
    fn test_get_all_different_backgrounds() {
        let conn = setup_test_db();
        insert_test_edge(&conn, 1, "Alertness", "Background", false);
        insert_test_edge(&conn, 2, "Block", "Combat", false);
        insert_test_edge(&conn, 3, "Command", "Leadership", false);
        insert_test_edge(&conn, 4, "Wizard", "Power", true);

        let edges = EdgeRepository::get_all(&conn).unwrap();

        assert_eq!(edges.len(), 4);
        // Verify different background types are all retrieved
        let backgrounds: Vec<String> = edges.iter().map(|e| e.background.clone()).collect();
        assert!(backgrounds.contains(&"Background".to_string()));
        assert!(backgrounds.contains(&"Combat".to_string()));
        assert!(backgrounds.contains(&"Leadership".to_string()));
        assert!(backgrounds.contains(&"Power".to_string()));
    }

    #[test]
    fn test_row_to_model_all_fields() {
        let conn = setup_test_db();
        conn.execute(
            "INSERT INTO edges (id, name, background, source, description,
                               can_take_multiple_times, created_at, updated_at)
             VALUES (42, 'Test Edge', 'Social', 'supplement', 'Detailed description',
                     1, '2024-12-25 10:30:00', '2024-12-26 15:45:00')",
            [],
        )
        .unwrap();

        let edge = EdgeRepository::get_by_id(&conn, 42).unwrap().unwrap();

        assert_eq!(edge.id, 42);
        assert_eq!(edge.name, "Test Edge");
        assert_eq!(edge.background, "Social");
        assert_eq!(edge.source, "supplement");
        assert_eq!(edge.description, "Detailed description");
        assert_eq!(edge.can_take_multiple_times, true);
        assert_eq!(edge.created_at, "2024-12-25 10:30:00");
        assert_eq!(edge.updated_at, "2024-12-26 15:45:00");
    }
}
