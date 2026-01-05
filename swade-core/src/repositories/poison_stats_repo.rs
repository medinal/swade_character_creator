use crate::error::Result;
use crate::models::PoisonStats;
use super::base_repository::query_one_by_id;
use rusqlite::{Connection, Row, params};

pub struct PoisonStatsRepository;

impl PoisonStatsRepository {
    const TABLE: &'static str = "poison_stats";
    const COLUMNS: &'static str = "id, gear_id, poison_type, delivery_method, affected_attribute, notes, created_at, updated_at";

    /// Get poison stats by ID
    pub fn get_by_id(conn: &Connection, id: i64) -> Result<Option<PoisonStats>> {
        query_one_by_id(conn, Self::TABLE, Self::COLUMNS, id, Self::row_to_model)
    }

    /// Get poison stats by gear ID
    pub fn get_by_gear_id(conn: &Connection, gear_id: i64) -> Result<Option<PoisonStats>> {
        let sql = format!("SELECT {} FROM {} WHERE gear_id = ?", Self::COLUMNS, Self::TABLE);
        let mut stmt = conn.prepare(&sql)?;
        let mut rows = stmt.query(params![gear_id])?;

        match rows.next()? {
            Some(row) => Ok(Some(Self::row_to_model(row)?)),
            None => Ok(None),
        }
    }

    /// Convert a database row to a PoisonStats model
    fn row_to_model(row: &Row) -> rusqlite::Result<PoisonStats> {
        Ok(PoisonStats {
            id: row.get(0)?,
            gear_id: row.get(1)?,
            poison_type: row.get(2)?,
            delivery_method: row.get(3)?,
            affected_attribute: row.get(4)?,
            notes: row.get(5)?,
            created_at: row.get(6)?,
            updated_at: row.get(7)?,
        })
    }
}
