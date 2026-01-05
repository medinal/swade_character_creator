use crate::error::Result;
use crate::models::PackContents;
use super::base_repository::query_by_fk_id;
use rusqlite::{Connection, Row};

pub struct PackContentsRepository;

impl PackContentsRepository {
    const TABLE: &'static str = "pack_contents";
    const COLUMNS: &'static str = "id, pack_gear_id, item_gear_id, quantity, notes, created_at, updated_at";

    /// Get all contents for a specific pack
    pub fn get_by_pack_gear_id(conn: &Connection, pack_gear_id: i64) -> Result<Vec<PackContents>> {
        query_by_fk_id(
            conn,
            Self::TABLE,
            Self::COLUMNS,
            "pack_gear_id",
            pack_gear_id,
            "id",
            Self::row_to_model,
        )
    }

    /// Convert a database row to a PackContents model
    fn row_to_model(row: &Row) -> rusqlite::Result<PackContents> {
        Ok(PackContents {
            id: row.get(0)?,
            pack_gear_id: row.get(1)?,
            item_gear_id: row.get(2)?,
            quantity: row.get(3)?,
            notes: row.get(4)?,
            created_at: row.get(5)?,
            updated_at: row.get(6)?,
        })
    }
}
