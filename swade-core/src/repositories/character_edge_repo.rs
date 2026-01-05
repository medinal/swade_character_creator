use crate::error::Result;
use crate::models::CharacterEdge;
use rusqlite::{Connection, Row, params};

pub struct CharacterEdgeRepository;

impl CharacterEdgeRepository {
    /// Get a single character edge by ID
    pub fn get_by_id(conn: &Connection, id: i64) -> Result<Option<CharacterEdge>> {
        let mut stmt = conn.prepare(
            "SELECT id, character_id, edge_id, advance_taken, notes, source,
                    created_at, updated_at
             FROM character_edges WHERE id = ?",
        )?;

        let mut rows = stmt.query(params![id])?;

        match rows.next()? {
            Some(row) => Ok(Some(Self::row_to_model(row)?)),
            None => Ok(None),
        }
    }

    /// Get all character edges for a specific character
    pub fn get_by_character_id(conn: &Connection, character_id: i64) -> Result<Vec<CharacterEdge>> {
        let mut stmt = conn.prepare(
            "SELECT id, character_id, edge_id, advance_taken, notes, source,
                    created_at, updated_at
             FROM character_edges WHERE character_id = ? ORDER BY advance_taken, id",
        )?;

        let rows = stmt.query_map(params![character_id], Self::row_to_model)?;

        Ok(rows.filter_map(|r| r.ok()).collect())
    }

    /// Insert a new character edge, returns the generated ID
    pub fn insert(conn: &Connection, model: &CharacterEdge) -> Result<i64> {
        conn.execute(
            "INSERT INTO character_edges (character_id, edge_id, advance_taken, notes, source,
                                         created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?)",
            params![
                model.character_id,
                model.edge_id,
                model.advance_taken,
                model.notes,
                model.source,
                model.created_at,
                model.updated_at
            ],
        )?;

        Ok(conn.last_insert_rowid())
    }

    /// Update an existing character edge
    pub fn update(conn: &Connection, model: &CharacterEdge) -> Result<()> {
        conn.execute(
            "UPDATE character_edges
             SET character_id = ?, edge_id = ?, advance_taken = ?, notes = ?, source = ?,
                 updated_at = ?
             WHERE id = ?",
            params![
                model.character_id,
                model.edge_id,
                model.advance_taken,
                model.notes,
                model.source,
                model.updated_at,
                model.id
            ],
        )?;

        Ok(())
    }

    /// Delete a character edge by ID
    pub fn delete(conn: &Connection, id: i64) -> Result<()> {
        conn.execute("DELETE FROM character_edges WHERE id = ?", params![id])?;
        Ok(())
    }

    /// Delete all edges for a character with a specific source
    pub fn delete_by_character_id_and_source(
        conn: &Connection,
        character_id: i64,
        source: &str,
    ) -> Result<()> {
        conn.execute(
            "DELETE FROM character_edges WHERE character_id = ? AND source = ?",
            params![character_id, source],
        )?;
        Ok(())
    }

    /// Convert a database row to a CharacterEdge model
    fn row_to_model(row: &Row) -> rusqlite::Result<CharacterEdge> {
        Ok(CharacterEdge {
            id: row.get(0)?,
            character_id: row.get(1)?,
            edge_id: row.get(2)?,
            advance_taken: row.get(3)?,
            notes: row.get(4)?,
            source: row.get(5)?,
            created_at: row.get(6)?,
            updated_at: row.get(7)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;

    fn setup_test_db() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        let schema = include_str!("../../../db/schema.sql");
        conn.execute_batch(schema).unwrap();

        // Insert required rank for foreign key constraint
        conn.execute(
            "INSERT INTO ranks (id, name, min_advances, max_advances, description, created_at, updated_at)
             VALUES (1, 'Novice', 0, 3, 'Starting rank', '2024-01-01 00:00:00', '2024-01-01 00:00:00')",
            [],
        ).unwrap();

        conn
    }

    fn insert_test_character(conn: &Connection, id: i64, name: &str) {
        conn.execute(
            "INSERT INTO characters (id, name, is_wild_card, created_at, updated_at)
             VALUES (?, ?, 1, '2024-01-01 00:00:00', '2024-01-01 00:00:00')",
            params![id, name],
        )
        .unwrap();
    }

    fn insert_test_edge(conn: &Connection, id: i64, name: &str, can_take_multiple_times: bool) {
        conn.execute(
            "INSERT INTO edges (id, name, background, source, description,
                               can_take_multiple_times, created_at, updated_at)
             VALUES (?, ?, 'Background', 'core', 'Test description',
                     ?, '2024-01-01 00:00:00', '2024-01-01 00:00:00')",
            params![id, name, can_take_multiple_times],
        )
        .unwrap();
    }

    fn insert_test_character_edge(
        conn: &Connection,
        id: i64,
        character_id: i64,
        edge_id: i64,
        advance_taken: i64,
        notes: Option<&str>,
        source: &str,
    ) {
        conn.execute(
            "INSERT INTO character_edges (id, character_id, edge_id, advance_taken, notes, source,
                                         created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, '2024-01-01 00:00:00', '2024-01-01 00:00:00')",
            params![id, character_id, edge_id, advance_taken, notes, source],
        )
        .unwrap();
    }

    #[test]
    fn test_get_by_id_found() {
        let conn = setup_test_db();
        insert_test_character(&conn, 1, "Test Character");
        insert_test_edge(&conn, 1, "Alertness", false);
        insert_test_character_edge(&conn, 1, 1, 1, 0, None, "advancement");

        let result = CharacterEdgeRepository::get_by_id(&conn, 1).unwrap();

        assert!(result.is_some());
        let character_edge = result.unwrap();
        assert_eq!(character_edge.id, 1);
        assert_eq!(character_edge.character_id, 1);
        assert_eq!(character_edge.edge_id, 1);
        assert_eq!(character_edge.advance_taken, 0);
        assert_eq!(character_edge.notes, None);
        assert_eq!(character_edge.source, "advancement");
        assert_eq!(character_edge.created_at, "2024-01-01 00:00:00");
        assert_eq!(character_edge.updated_at, "2024-01-01 00:00:00");
    }

    #[test]
    fn test_get_by_id_not_found() {
        let conn = setup_test_db();

        let result = CharacterEdgeRepository::get_by_id(&conn, 999).unwrap();

        assert!(result.is_none());
    }

    #[test]
    fn test_get_by_id_with_notes() {
        let conn = setup_test_db();
        insert_test_character(&conn, 1, "Test Character");
        insert_test_edge(&conn, 1, "Scholar", true);
        insert_test_character_edge(&conn, 1, 1, 1, 2, Some("Athletics"), "advancement");

        let result = CharacterEdgeRepository::get_by_id(&conn, 1).unwrap();

        assert!(result.is_some());
        let character_edge = result.unwrap();
        assert_eq!(character_edge.notes, Some("Athletics".to_string()));
    }

    #[test]
    fn test_get_by_character_id_empty() {
        let conn = setup_test_db();
        insert_test_character(&conn, 1, "Test Character");

        let edges = CharacterEdgeRepository::get_by_character_id(&conn, 1).unwrap();

        assert_eq!(edges.len(), 0);
    }

    #[test]
    fn test_get_by_character_id_single_edge() {
        let conn = setup_test_db();
        insert_test_character(&conn, 1, "Test Character");
        insert_test_edge(&conn, 1, "Alertness", false);
        insert_test_character_edge(&conn, 1, 1, 1, 0, None, "advancement");

        let edges = CharacterEdgeRepository::get_by_character_id(&conn, 1).unwrap();

        assert_eq!(edges.len(), 1);
        assert_eq!(edges[0].character_id, 1);
        assert_eq!(edges[0].edge_id, 1);
    }

    #[test]
    fn test_get_by_character_id_multiple_edges() {
        let conn = setup_test_db();
        insert_test_character(&conn, 1, "Test Character");
        insert_test_edge(&conn, 1, "Alertness", false);
        insert_test_edge(&conn, 2, "Brawny", false);
        insert_test_edge(&conn, 3, "Combat Reflexes", false);
        insert_test_character_edge(&conn, 1, 1, 1, 0, None, "advancement");
        insert_test_character_edge(&conn, 2, 1, 2, 1, None, "advancement");
        insert_test_character_edge(&conn, 3, 1, 3, 2, None, "advancement");

        let edges = CharacterEdgeRepository::get_by_character_id(&conn, 1).unwrap();

        assert_eq!(edges.len(), 3);
        // Verify ordering by advance_taken
        assert_eq!(edges[0].advance_taken, 0);
        assert_eq!(edges[1].advance_taken, 1);
        assert_eq!(edges[2].advance_taken, 2);
    }

    #[test]
    fn test_get_by_character_id_different_sources() {
        let conn = setup_test_db();
        insert_test_character(&conn, 1, "Test Character");
        insert_test_edge(&conn, 1, "Alertness", false);
        insert_test_edge(&conn, 2, "Brawny", false);
        insert_test_edge(&conn, 3, "Quick", false);
        insert_test_character_edge(&conn, 1, 1, 1, 0, None, "ancestry");
        insert_test_character_edge(&conn, 2, 1, 2, 0, None, "hindrance_points");
        insert_test_character_edge(&conn, 3, 1, 3, 1, None, "advancement");

        let edges = CharacterEdgeRepository::get_by_character_id(&conn, 1).unwrap();

        assert_eq!(edges.len(), 3);
        let sources: Vec<String> = edges.iter().map(|e| e.source.clone()).collect();
        assert!(sources.contains(&"ancestry".to_string()));
        assert!(sources.contains(&"hindrance_points".to_string()));
        assert!(sources.contains(&"advancement".to_string()));
    }

    #[test]
    fn test_get_by_character_id_does_not_return_other_characters() {
        let conn = setup_test_db();
        insert_test_character(&conn, 1, "Character One");
        insert_test_character(&conn, 2, "Character Two");
        insert_test_edge(&conn, 1, "Alertness", false);
        insert_test_edge(&conn, 2, "Brawny", false);
        insert_test_character_edge(&conn, 1, 1, 1, 0, None, "advancement");
        insert_test_character_edge(&conn, 2, 2, 2, 0, None, "advancement");

        let edges = CharacterEdgeRepository::get_by_character_id(&conn, 1).unwrap();

        assert_eq!(edges.len(), 1);
        assert_eq!(edges[0].character_id, 1);
        assert_eq!(edges[0].edge_id, 1);
    }

    #[test]
    fn test_get_by_character_id_same_edge_multiple_times() {
        let conn = setup_test_db();
        insert_test_character(&conn, 1, "Test Character");
        insert_test_edge(&conn, 1, "Scholar", true); // Can take multiple times
        insert_test_character_edge(&conn, 1, 1, 1, 0, Some("Athletics"), "advancement");
        insert_test_character_edge(&conn, 2, 1, 1, 1, Some("Occult"), "advancement");

        let edges = CharacterEdgeRepository::get_by_character_id(&conn, 1).unwrap();

        assert_eq!(edges.len(), 2);
        assert_eq!(edges[0].edge_id, 1);
        assert_eq!(edges[1].edge_id, 1);
        assert_eq!(edges[0].notes, Some("Athletics".to_string()));
        assert_eq!(edges[1].notes, Some("Occult".to_string()));
    }

    #[test]
    fn test_insert_basic() {
        let conn = setup_test_db();
        insert_test_character(&conn, 1, "Test Character");
        insert_test_edge(&conn, 1, "Alertness", false);

        let character_edge = CharacterEdge {
            id: 0, // Will be set by database
            character_id: 1,
            edge_id: 1,
            advance_taken: 0,
            notes: None,
            source: "advancement".to_string(),
            created_at: "2024-01-01 00:00:00".to_string(),
            updated_at: "2024-01-01 00:00:00".to_string(),
        };

        let id = CharacterEdgeRepository::insert(&conn, &character_edge).unwrap();

        assert!(id > 0);
        let retrieved = CharacterEdgeRepository::get_by_id(&conn, id).unwrap();
        assert!(retrieved.is_some());
        let retrieved_edge = retrieved.unwrap();
        assert_eq!(retrieved_edge.character_id, 1);
        assert_eq!(retrieved_edge.edge_id, 1);
        assert_eq!(retrieved_edge.advance_taken, 0);
        assert_eq!(retrieved_edge.notes, None);
        assert_eq!(retrieved_edge.source, "advancement");
    }

    #[test]
    fn test_insert_with_notes() {
        let conn = setup_test_db();
        insert_test_character(&conn, 1, "Test Character");
        insert_test_edge(&conn, 1, "Scholar", true);

        let character_edge = CharacterEdge {
            id: 0,
            character_id: 1,
            edge_id: 1,
            advance_taken: 2,
            notes: Some("Athletics".to_string()),
            source: "advancement".to_string(),
            created_at: "2024-01-01 00:00:00".to_string(),
            updated_at: "2024-01-01 00:00:00".to_string(),
        };

        let id = CharacterEdgeRepository::insert(&conn, &character_edge).unwrap();

        let retrieved = CharacterEdgeRepository::get_by_id(&conn, id)
            .unwrap()
            .unwrap();
        assert_eq!(retrieved.notes, Some("Athletics".to_string()));
    }

    #[test]
    fn test_insert_different_sources() {
        let conn = setup_test_db();
        insert_test_character(&conn, 1, "Test Character");
        insert_test_edge(&conn, 1, "Alertness", false);
        insert_test_edge(&conn, 2, "Brawny", false);

        let edge1 = CharacterEdge {
            id: 0,
            character_id: 1,
            edge_id: 1,
            advance_taken: 0,
            notes: None,
            source: "ancestry".to_string(),
            created_at: "2024-01-01 00:00:00".to_string(),
            updated_at: "2024-01-01 00:00:00".to_string(),
        };

        let edge2 = CharacterEdge {
            id: 0,
            character_id: 1,
            edge_id: 2,
            advance_taken: 0,
            notes: None,
            source: "hindrance_points".to_string(),
            created_at: "2024-01-01 00:00:00".to_string(),
            updated_at: "2024-01-01 00:00:00".to_string(),
        };

        CharacterEdgeRepository::insert(&conn, &edge1).unwrap();
        CharacterEdgeRepository::insert(&conn, &edge2).unwrap();

        let edges = CharacterEdgeRepository::get_by_character_id(&conn, 1).unwrap();
        assert_eq!(edges.len(), 2);
    }

    #[test]
    fn test_update_basic_fields() {
        let conn = setup_test_db();
        insert_test_character(&conn, 1, "Test Character");
        insert_test_edge(&conn, 1, "Alertness", false);
        insert_test_edge(&conn, 2, "Brawny", false);
        insert_test_character_edge(&conn, 1, 1, 1, 0, None, "advancement");

        let mut character_edge = CharacterEdgeRepository::get_by_id(&conn, 1)
            .unwrap()
            .unwrap();
        character_edge.edge_id = 2;
        character_edge.advance_taken = 1;
        character_edge.source = "hindrance_points".to_string();
        character_edge.updated_at = "2024-01-02 00:00:00".to_string();

        CharacterEdgeRepository::update(&conn, &character_edge).unwrap();

        let updated = CharacterEdgeRepository::get_by_id(&conn, 1)
            .unwrap()
            .unwrap();
        assert_eq!(updated.edge_id, 2);
        assert_eq!(updated.advance_taken, 1);
        assert_eq!(updated.source, "hindrance_points");
        assert_eq!(updated.updated_at, "2024-01-02 00:00:00");
    }

    #[test]
    fn test_update_notes() {
        let conn = setup_test_db();
        insert_test_character(&conn, 1, "Test Character");
        insert_test_edge(&conn, 1, "Scholar", true);
        insert_test_character_edge(&conn, 1, 1, 1, 0, Some("Athletics"), "advancement");

        let mut character_edge = CharacterEdgeRepository::get_by_id(&conn, 1)
            .unwrap()
            .unwrap();
        character_edge.notes = Some("Occult".to_string());
        character_edge.updated_at = "2024-01-02 00:00:00".to_string();

        CharacterEdgeRepository::update(&conn, &character_edge).unwrap();

        let updated = CharacterEdgeRepository::get_by_id(&conn, 1)
            .unwrap()
            .unwrap();
        assert_eq!(updated.notes, Some("Occult".to_string()));
    }

    #[test]
    fn test_update_notes_to_none() {
        let conn = setup_test_db();
        insert_test_character(&conn, 1, "Test Character");
        insert_test_edge(&conn, 1, "Scholar", true);
        insert_test_character_edge(&conn, 1, 1, 1, 0, Some("Athletics"), "advancement");

        let mut character_edge = CharacterEdgeRepository::get_by_id(&conn, 1)
            .unwrap()
            .unwrap();
        character_edge.notes = None;
        character_edge.updated_at = "2024-01-02 00:00:00".to_string();

        CharacterEdgeRepository::update(&conn, &character_edge).unwrap();

        let updated = CharacterEdgeRepository::get_by_id(&conn, 1)
            .unwrap()
            .unwrap();
        assert_eq!(updated.notes, None);
    }

    #[test]
    fn test_delete_basic() {
        let conn = setup_test_db();
        insert_test_character(&conn, 1, "Test Character");
        insert_test_edge(&conn, 1, "Alertness", false);
        insert_test_character_edge(&conn, 1, 1, 1, 0, None, "advancement");

        CharacterEdgeRepository::delete(&conn, 1).unwrap();

        let result = CharacterEdgeRepository::get_by_id(&conn, 1).unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn test_delete_nonexistent() {
        let conn = setup_test_db();

        // Should not error when deleting nonexistent record
        let result = CharacterEdgeRepository::delete(&conn, 999);
        assert!(result.is_ok());
    }

    #[test]
    fn test_delete_does_not_affect_other_records() {
        let conn = setup_test_db();
        insert_test_character(&conn, 1, "Test Character");
        insert_test_edge(&conn, 1, "Alertness", false);
        insert_test_edge(&conn, 2, "Brawny", false);
        insert_test_character_edge(&conn, 1, 1, 1, 0, None, "advancement");
        insert_test_character_edge(&conn, 2, 1, 2, 1, None, "advancement");

        CharacterEdgeRepository::delete(&conn, 1).unwrap();

        let edges = CharacterEdgeRepository::get_by_character_id(&conn, 1).unwrap();
        assert_eq!(edges.len(), 1);
        assert_eq!(edges[0].id, 2);
    }

    #[test]
    fn test_row_to_model_all_fields() {
        let conn = setup_test_db();
        insert_test_character(&conn, 1, "Test Character");
        insert_test_edge(&conn, 1, "Test Edge", true);
        conn.execute(
            "INSERT INTO character_edges (id, character_id, edge_id, advance_taken, notes, source,
                                         created_at, updated_at)
             VALUES (42, 1, 1, 5, 'Test notes', 'ancestry',
                     '2024-12-25 10:30:00', '2024-12-26 15:45:00')",
            [],
        )
        .unwrap();

        let character_edge = CharacterEdgeRepository::get_by_id(&conn, 42)
            .unwrap()
            .unwrap();

        assert_eq!(character_edge.id, 42);
        assert_eq!(character_edge.character_id, 1);
        assert_eq!(character_edge.edge_id, 1);
        assert_eq!(character_edge.advance_taken, 5);
        assert_eq!(character_edge.notes, Some("Test notes".to_string()));
        assert_eq!(character_edge.source, "ancestry");
        assert_eq!(character_edge.created_at, "2024-12-25 10:30:00");
        assert_eq!(character_edge.updated_at, "2024-12-26 15:45:00");
    }
}
