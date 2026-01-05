use crate::error::Result;
use crate::models::ShieldStats;
use super::base_repository::query_one_by_id;
use rusqlite::{Connection, Row, params};

pub struct ShieldStatsRepository;

impl ShieldStatsRepository {
    const TABLE: &'static str = "shield_stats";
    const COLUMNS: &'static str = "id, gear_id, parry_bonus, cover_penalty, min_strength, notes, created_at, updated_at";

    /// Get shield stats by ID
    pub fn get_by_id(conn: &Connection, id: i64) -> Result<Option<ShieldStats>> {
        query_one_by_id(conn, Self::TABLE, Self::COLUMNS, id, Self::row_to_model)
    }

    /// Get shield stats by gear ID
    pub fn get_by_gear_id(conn: &Connection, gear_id: i64) -> Result<Option<ShieldStats>> {
        let sql = format!("SELECT {} FROM {} WHERE gear_id = ?", Self::COLUMNS, Self::TABLE);
        let mut stmt = conn.prepare(&sql)?;
        let mut rows = stmt.query(params![gear_id])?;

        match rows.next()? {
            Some(row) => Ok(Some(Self::row_to_model(row)?)),
            None => Ok(None),
        }
    }

    /// Convert a database row to a ShieldStats model
    fn row_to_model(row: &Row) -> rusqlite::Result<ShieldStats> {
        Ok(ShieldStats {
            id: row.get(0)?,
            gear_id: row.get(1)?,
            parry_bonus: row.get(2)?,
            cover_penalty: row.get(3)?,
            min_strength: row.get(4)?,
            notes: row.get(5)?,
            created_at: row.get(6)?,
            updated_at: row.get(7)?,
        })
    }
}
