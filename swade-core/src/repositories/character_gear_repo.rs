use crate::error::Result;
use crate::models::CharacterGear;
use rusqlite::{Connection, Row, params};

pub struct CharacterGearRepository;

impl CharacterGearRepository {
    /// Get a single character gear entry by ID
    pub fn get_by_id(conn: &Connection, id: i64) -> Result<Option<CharacterGear>> {
        let mut stmt = conn.prepare(
            "SELECT id, character_id, gear_id, quantity, is_equipped, custom_notes,
                    created_at, updated_at
             FROM character_gear WHERE id = ?",
        )?;

        let mut rows = stmt.query(params![id])?;

        match rows.next()? {
            Some(row) => Ok(Some(Self::row_to_model(row)?)),
            None => Ok(None),
        }
    }

    /// Get all gear for a specific character
    pub fn get_by_character_id(conn: &Connection, character_id: i64) -> Result<Vec<CharacterGear>> {
        let mut stmt = conn.prepare(
            "SELECT id, character_id, gear_id, quantity, is_equipped, custom_notes,
                    created_at, updated_at
             FROM character_gear WHERE character_id = ? ORDER BY gear_id",
        )?;

        let rows = stmt.query_map(params![character_id], Self::row_to_model)?;

        Ok(rows.filter_map(|r| r.ok()).collect())
    }

    /// Insert a new character gear entry, returns the generated ID
    pub fn insert(conn: &Connection, model: &CharacterGear) -> Result<i64> {
        conn.execute(
            "INSERT INTO character_gear (character_id, gear_id, quantity, is_equipped, custom_notes,
                                        created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?)",
            params![
                model.character_id,
                model.gear_id,
                model.quantity,
                model.is_equipped,
                model.custom_notes,
                model.created_at,
                model.updated_at
            ],
        )?;

        Ok(conn.last_insert_rowid())
    }

    /// Update an existing character gear entry
    pub fn update(conn: &Connection, model: &CharacterGear) -> Result<()> {
        conn.execute(
            "UPDATE character_gear
             SET character_id = ?, gear_id = ?, quantity = ?, is_equipped = ?, custom_notes = ?,
                 updated_at = ?
             WHERE id = ?",
            params![
                model.character_id,
                model.gear_id,
                model.quantity,
                model.is_equipped,
                model.custom_notes,
                model.updated_at,
                model.id
            ],
        )?;

        Ok(())
    }

    /// Delete a character gear entry by ID
    pub fn delete(conn: &Connection, id: i64) -> Result<()> {
        conn.execute("DELETE FROM character_gear WHERE id = ?", params![id])?;
        Ok(())
    }

    /// Delete all gear for a character
    pub fn delete_by_character_id(conn: &Connection, character_id: i64) -> Result<()> {
        conn.execute(
            "DELETE FROM character_gear WHERE character_id = ?",
            params![character_id],
        )?;
        Ok(())
    }

    /// Convert a database row to a CharacterGear model
    fn row_to_model(row: &Row) -> rusqlite::Result<CharacterGear> {
        Ok(CharacterGear {
            id: row.get(0)?,
            character_id: row.get(1)?,
            gear_id: row.get(2)?,
            quantity: row.get(3)?,
            is_equipped: row.get(4)?,
            custom_notes: row.get(5)?,
            created_at: row.get(6)?,
            updated_at: row.get(7)?,
        })
    }
}
