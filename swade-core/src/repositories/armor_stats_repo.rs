use crate::error::Result;
use crate::models::ArmorStats;
use super::base_repository::query_one_by_id;
use rusqlite::{Connection, Row, params};

pub struct ArmorStatsRepository;

impl ArmorStatsRepository {
    const TABLE: &'static str = "armor_stats";
    const COLUMNS: &'static str = "id, gear_id, armor_value, coverage, min_strength, is_heavy, created_at, updated_at";

    /// Get armor stats by ID
    pub fn get_by_id(conn: &Connection, id: i64) -> Result<Option<ArmorStats>> {
        query_one_by_id(conn, Self::TABLE, Self::COLUMNS, id, Self::row_to_model)
    }

    /// Get armor stats by gear ID
    pub fn get_by_gear_id(conn: &Connection, gear_id: i64) -> Result<Option<ArmorStats>> {
        let sql = format!("SELECT {} FROM {} WHERE gear_id = ?", Self::COLUMNS, Self::TABLE);
        let mut stmt = conn.prepare(&sql)?;
        let mut rows = stmt.query(params![gear_id])?;

        match rows.next()? {
            Some(row) => Ok(Some(Self::row_to_model(row)?)),
            None => Ok(None),
        }
    }

    /// Convert a database row to an ArmorStats model
    fn row_to_model(row: &Row) -> rusqlite::Result<ArmorStats> {
        Ok(ArmorStats {
            id: row.get(0)?,
            gear_id: row.get(1)?,
            armor_value: row.get(2)?,
            coverage: row.get(3)?,
            min_strength: row.get(4)?,
            is_heavy: row.get(5)?,
            created_at: row.get(6)?,
            updated_at: row.get(7)?,
        })
    }
}
