use rusqlite::{Connection, Row, params};

use crate::error::Result;
use crate::models::CharacterPower;

pub struct CharacterPowerRepository;

impl CharacterPowerRepository {
    /// Get a single character power by ID
    pub fn get_by_id(conn: &Connection, id: i64) -> Result<Option<CharacterPower>> {
        let mut stmt = conn.prepare(
            "SELECT id, character_id, power_id, advance_taken, created_at, updated_at
             FROM character_powers WHERE id = ?",
        )?;

        let mut rows = stmt.query(params![id])?;

        match rows.next()? {
            Some(row) => Ok(Some(Self::row_to_model(row)?)),
            None => Ok(None),
        }
    }

    /// Get all powers for a specific character
    pub fn get_by_character_id(
        conn: &Connection,
        character_id: i64,
    ) -> Result<Vec<CharacterPower>> {
        let mut stmt = conn.prepare(
            "SELECT id, character_id, power_id, advance_taken, created_at, updated_at
             FROM character_powers WHERE character_id = ?
             ORDER BY id",
        )?;

        let rows = stmt.query_map(params![character_id], Self::row_to_model)?;

        Ok(rows.filter_map(|r| r.ok()).collect())
    }

    /// Insert a new character power, return generated ID
    pub fn insert(conn: &Connection, character_power: &CharacterPower) -> Result<i64> {
        conn.execute(
            "INSERT INTO character_powers (
                character_id, power_id, advance_taken
             ) VALUES (?1, ?2, ?3)",
            params![
                character_power.character_id,
                character_power.power_id,
                character_power.advance_taken,
            ],
        )?;

        Ok(conn.last_insert_rowid())
    }

    /// Update an existing character power
    pub fn update(conn: &Connection, character_power: &CharacterPower) -> Result<()> {
        conn.execute(
            "UPDATE character_powers SET
                character_id = ?1,
                power_id = ?2,
                advance_taken = ?3,
                updated_at = CURRENT_TIMESTAMP
             WHERE id = ?4",
            params![
                character_power.character_id,
                character_power.power_id,
                character_power.advance_taken,
                character_power.id,
            ],
        )?;

        Ok(())
    }

    /// Delete a character power
    pub fn delete(conn: &Connection, id: i64) -> Result<()> {
        conn.execute("DELETE FROM character_powers WHERE id = ?", params![id])?;
        Ok(())
    }

    /// Delete all powers for a character
    pub fn delete_by_character_id(conn: &Connection, character_id: i64) -> Result<()> {
        conn.execute(
            "DELETE FROM character_powers WHERE character_id = ?",
            params![character_id],
        )?;
        Ok(())
    }

    /// Convert a database row to a CharacterPower model
    fn row_to_model(row: &Row) -> rusqlite::Result<CharacterPower> {
        Ok(CharacterPower {
            id: row.get(0)?,
            character_id: row.get(1)?,
            power_id: row.get(2)?,
            advance_taken: row.get(3)?,
            created_at: row.get(4)?,
            updated_at: row.get(5)?,
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
            "INSERT INTO characters (id, is_wild_card, name, ancestry_id,
                                    attribute_points_spent, attribute_points_earned,
                                    skill_points_spent, skill_points_earned,
                                    hindrance_points_spent, hindrance_points_earned,
                                    hindrance_points_to_edges, hindrance_points_to_attributes,
                                    hindrance_points_to_skills, hindrance_points_to_wealth,
                                    power_points, wealth, background, description,
                                    created_at, updated_at)
             VALUES (?, 1, ?, NULL, 0, 5, 0, 12, 0, 4, 0, 0, 0, 0, 0, 500,
                     'Test background', 'Test description', '2024-01-01 00:00:00', '2024-01-01 00:00:00')",
            params![id, name],
        ).unwrap();
    }

    fn insert_test_power(conn: &Connection, id: i64, name: &str) {
        conn.execute(
            "INSERT INTO powers (id, name, power_points, range, duration, source, description, created_at, updated_at)
             VALUES (?, ?, 1, 'Smarts', 'Instant', 'core', 'Test power description', '2024-01-01 00:00:00', '2024-01-01 00:00:00')",
            params![id, name],
        ).unwrap();
    }

    fn insert_test_character_power(
        conn: &Connection,
        id: i64,
        character_id: i64,
        power_id: i64,
        advance_taken: Option<i64>,
    ) {
        conn.execute(
            "INSERT INTO character_powers (id, character_id, power_id, advance_taken, created_at, updated_at)
             VALUES (?, ?, ?, ?, '2024-01-01 00:00:00', '2024-01-01 00:00:00')",
            params![id, character_id, power_id, advance_taken],
        )
        .unwrap();
    }

    #[test]
    fn test_get_by_id_found() {
        let conn = setup_test_db();
        insert_test_character(&conn, 1, "Test Character");
        insert_test_power(&conn, 1, "Bolt");
        insert_test_character_power(&conn, 1, 1, 1, None);

        let result = CharacterPowerRepository::get_by_id(&conn, 1).unwrap();

        assert!(result.is_some());
        let character_power = result.unwrap();
        assert_eq!(character_power.id, 1);
        assert_eq!(character_power.character_id, 1);
        assert_eq!(character_power.power_id, 1);
        assert_eq!(character_power.advance_taken, None);
        assert_eq!(character_power.created_at, "2024-01-01 00:00:00");
        assert_eq!(character_power.updated_at, "2024-01-01 00:00:00");
    }

    #[test]
    fn test_get_by_id_not_found() {
        let conn = setup_test_db();

        let result = CharacterPowerRepository::get_by_id(&conn, 999).unwrap();

        assert!(result.is_none());
    }

    #[test]
    fn test_get_by_id_with_advance_taken() {
        let conn = setup_test_db();
        insert_test_character(&conn, 1, "Test Character");
        insert_test_power(&conn, 1, "Healing");
        insert_test_character_power(&conn, 1, 1, 1, Some(3));

        let result = CharacterPowerRepository::get_by_id(&conn, 1).unwrap();

        assert!(result.is_some());
        let character_power = result.unwrap();
        assert_eq!(character_power.advance_taken, Some(3));
    }

    #[test]
    fn test_get_by_character_id_empty() {
        let conn = setup_test_db();
        insert_test_character(&conn, 1, "Test Character");

        let powers = CharacterPowerRepository::get_by_character_id(&conn, 1).unwrap();

        assert_eq!(powers.len(), 0);
    }

    #[test]
    fn test_get_by_character_id_single() {
        let conn = setup_test_db();
        insert_test_character(&conn, 1, "Test Character");
        insert_test_power(&conn, 1, "Bolt");
        insert_test_character_power(&conn, 1, 1, 1, None);

        let powers = CharacterPowerRepository::get_by_character_id(&conn, 1).unwrap();

        assert_eq!(powers.len(), 1);
        assert_eq!(powers[0].character_id, 1);
        assert_eq!(powers[0].power_id, 1);
    }

    #[test]
    fn test_get_by_character_id_multiple() {
        let conn = setup_test_db();
        insert_test_character(&conn, 1, "Test Character");
        insert_test_power(&conn, 1, "Bolt");
        insert_test_power(&conn, 2, "Healing");
        insert_test_power(&conn, 3, "Fly");
        insert_test_character_power(&conn, 1, 1, 1, None);
        insert_test_character_power(&conn, 2, 1, 2, Some(1));
        insert_test_character_power(&conn, 3, 1, 3, Some(3));

        let powers = CharacterPowerRepository::get_by_character_id(&conn, 1).unwrap();

        assert_eq!(powers.len(), 3);
        assert_eq!(powers[0].power_id, 1);
        assert_eq!(powers[0].advance_taken, None);
        assert_eq!(powers[1].power_id, 2);
        assert_eq!(powers[1].advance_taken, Some(1));
        assert_eq!(powers[2].power_id, 3);
        assert_eq!(powers[2].advance_taken, Some(3));
    }

    #[test]
    fn test_get_by_character_id_filters_correctly() {
        let conn = setup_test_db();
        insert_test_character(&conn, 1, "Character One");
        insert_test_character(&conn, 2, "Character Two");
        insert_test_power(&conn, 1, "Bolt");
        insert_test_power(&conn, 2, "Healing");
        insert_test_character_power(&conn, 1, 1, 1, None);
        insert_test_character_power(&conn, 2, 2, 2, None);

        let powers = CharacterPowerRepository::get_by_character_id(&conn, 1).unwrap();

        assert_eq!(powers.len(), 1);
        assert_eq!(powers[0].character_id, 1);
        assert_eq!(powers[0].power_id, 1);
    }

    #[test]
    fn test_get_by_character_id_not_found() {
        let conn = setup_test_db();
        insert_test_character(&conn, 1, "Test Character");
        insert_test_power(&conn, 1, "Bolt");
        insert_test_character_power(&conn, 1, 1, 1, None);

        let powers = CharacterPowerRepository::get_by_character_id(&conn, 999).unwrap();

        assert_eq!(powers.len(), 0);
    }

    #[test]
    fn test_get_by_character_id_ordered_by_id() {
        let conn = setup_test_db();
        insert_test_character(&conn, 1, "Test Character");
        insert_test_power(&conn, 1, "Power A");
        insert_test_power(&conn, 2, "Power B");
        insert_test_power(&conn, 3, "Power C");
        // Insert in reverse order to test ordering
        insert_test_character_power(&conn, 3, 1, 3, None);
        insert_test_character_power(&conn, 1, 1, 1, None);
        insert_test_character_power(&conn, 2, 1, 2, None);

        let powers = CharacterPowerRepository::get_by_character_id(&conn, 1).unwrap();

        assert_eq!(powers.len(), 3);
        // Should be ordered by id
        assert_eq!(powers[0].id, 1);
        assert_eq!(powers[1].id, 2);
        assert_eq!(powers[2].id, 3);
    }

    #[test]
    fn test_insert() {
        let conn = setup_test_db();
        insert_test_character(&conn, 1, "Test Character");
        insert_test_power(&conn, 1, "Bolt");

        let character_power = CharacterPower {
            id: 0, // ID will be auto-generated
            character_id: 1,
            power_id: 1,
            advance_taken: None,
            created_at: String::new(), // Will be set by database
            updated_at: String::new(), // Will be set by database
        };

        let id = CharacterPowerRepository::insert(&conn, &character_power).unwrap();

        assert!(id > 0);

        // Verify the inserted record
        let result = CharacterPowerRepository::get_by_id(&conn, id).unwrap();
        assert!(result.is_some());
        let inserted = result.unwrap();
        assert_eq!(inserted.character_id, 1);
        assert_eq!(inserted.power_id, 1);
        assert_eq!(inserted.advance_taken, None);
    }

    #[test]
    fn test_insert_with_advance_taken() {
        let conn = setup_test_db();
        insert_test_character(&conn, 1, "Test Character");
        insert_test_power(&conn, 1, "Healing");

        let character_power = CharacterPower {
            id: 0,
            character_id: 1,
            power_id: 1,
            advance_taken: Some(5),
            created_at: String::new(),
            updated_at: String::new(),
        };

        let id = CharacterPowerRepository::insert(&conn, &character_power).unwrap();

        let result = CharacterPowerRepository::get_by_id(&conn, id).unwrap();
        assert!(result.is_some());
        let inserted = result.unwrap();
        assert_eq!(inserted.advance_taken, Some(5));
    }

    #[test]
    fn test_update() {
        let conn = setup_test_db();
        insert_test_character(&conn, 1, "Test Character");
        insert_test_power(&conn, 1, "Bolt");
        insert_test_power(&conn, 2, "Healing");
        insert_test_character_power(&conn, 1, 1, 1, None);

        let mut character_power = CharacterPowerRepository::get_by_id(&conn, 1)
            .unwrap()
            .unwrap();
        character_power.power_id = 2;
        character_power.advance_taken = Some(2);

        CharacterPowerRepository::update(&conn, &character_power).unwrap();

        let result = CharacterPowerRepository::get_by_id(&conn, 1).unwrap();
        assert!(result.is_some());
        let updated = result.unwrap();
        assert_eq!(updated.power_id, 2);
        assert_eq!(updated.advance_taken, Some(2));
    }

    #[test]
    fn test_update_clears_advance_taken() {
        let conn = setup_test_db();
        insert_test_character(&conn, 1, "Test Character");
        insert_test_power(&conn, 1, "Bolt");
        insert_test_character_power(&conn, 1, 1, 1, Some(3));

        let mut character_power = CharacterPowerRepository::get_by_id(&conn, 1)
            .unwrap()
            .unwrap();
        character_power.advance_taken = None;

        CharacterPowerRepository::update(&conn, &character_power).unwrap();

        let result = CharacterPowerRepository::get_by_id(&conn, 1).unwrap();
        assert!(result.is_some());
        let updated = result.unwrap();
        assert_eq!(updated.advance_taken, None);
    }

    #[test]
    fn test_delete() {
        let conn = setup_test_db();
        insert_test_character(&conn, 1, "Test Character");
        insert_test_power(&conn, 1, "Bolt");
        insert_test_character_power(&conn, 1, 1, 1, None);

        // Verify it exists
        let result = CharacterPowerRepository::get_by_id(&conn, 1).unwrap();
        assert!(result.is_some());

        // Delete it
        CharacterPowerRepository::delete(&conn, 1).unwrap();

        // Verify it's gone
        let result = CharacterPowerRepository::get_by_id(&conn, 1).unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn test_delete_nonexistent() {
        let conn = setup_test_db();

        // Should not error when deleting non-existent record
        let result = CharacterPowerRepository::delete(&conn, 999);
        assert!(result.is_ok());
    }

    #[test]
    fn test_row_to_model_all_fields() {
        let conn = setup_test_db();
        insert_test_character(&conn, 42, "Test Character");
        insert_test_power(&conn, 99, "Test Power");
        conn.execute(
            "INSERT INTO character_powers (id, character_id, power_id, advance_taken, created_at, updated_at)
             VALUES (123, 42, 99, 7, '2024-12-25 10:30:00', '2024-12-26 15:45:00')",
            [],
        )
        .unwrap();

        let character_power = CharacterPowerRepository::get_by_id(&conn, 123)
            .unwrap()
            .unwrap();

        assert_eq!(character_power.id, 123);
        assert_eq!(character_power.character_id, 42);
        assert_eq!(character_power.power_id, 99);
        assert_eq!(character_power.advance_taken, Some(7));
        assert_eq!(character_power.created_at, "2024-12-25 10:30:00");
        assert_eq!(character_power.updated_at, "2024-12-26 15:45:00");
    }

    #[test]
    fn test_unique_constraint() {
        let conn = setup_test_db();
        insert_test_character(&conn, 1, "Test Character");
        insert_test_power(&conn, 1, "Bolt");
        insert_test_character_power(&conn, 1, 1, 1, None);

        // Try to insert duplicate character_id + power_id
        let result = conn.execute(
            "INSERT INTO character_powers (character_id, power_id, advance_taken)
             VALUES (1, 1, NULL)",
            [],
        );

        // Should fail due to UNIQUE constraint
        assert!(result.is_err());
    }
}
