use crate::error::Result;
use crate::models::AmmunitionStats;
use super::base_repository::query_one_by_id;
use rusqlite::{Connection, Row, params};

pub struct AmmunitionStatsRepository;

impl AmmunitionStatsRepository {
    const TABLE: &'static str = "ammunition_stats";
    const COLUMNS: &'static str = "id, gear_id, ammo_type, quantity_per_unit, notes, created_at, updated_at";

    /// Get ammunition stats by ID
    pub fn get_by_id(conn: &Connection, id: i64) -> Result<Option<AmmunitionStats>> {
        query_one_by_id(conn, Self::TABLE, Self::COLUMNS, id, Self::row_to_model)
    }

    /// Get ammunition stats by gear ID
    pub fn get_by_gear_id(conn: &Connection, gear_id: i64) -> Result<Option<AmmunitionStats>> {
        let sql = format!("SELECT {} FROM {} WHERE gear_id = ?", Self::COLUMNS, Self::TABLE);
        let mut stmt = conn.prepare(&sql)?;
        let mut rows = stmt.query(params![gear_id])?;

        match rows.next()? {
            Some(row) => Ok(Some(Self::row_to_model(row)?)),
            None => Ok(None),
        }
    }

    /// Convert a database row to an AmmunitionStats model
    fn row_to_model(row: &Row) -> rusqlite::Result<AmmunitionStats> {
        Ok(AmmunitionStats {
            id: row.get(0)?,
            gear_id: row.get(1)?,
            ammo_type: row.get(2)?,
            quantity_per_unit: row.get(3)?,
            notes: row.get(4)?,
            created_at: row.get(5)?,
            updated_at: row.get(6)?,
        })
    }
}
