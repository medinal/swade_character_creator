use crate::error::Result;
use crate::models::WeaponStats;
use super::base_repository::query_one_by_id;
use rusqlite::{Connection, Row, params};

pub struct WeaponStatsRepository;

impl WeaponStatsRepository {
    const TABLE: &'static str = "weapon_stats";
    const COLUMNS: &'static str = "id, gear_id, damage, ap, range_short, range_medium, range_long, rof, shots, min_strength, is_two_handed, reach, blast_template, notes, created_at, updated_at";

    /// Get weapon stats by ID
    pub fn get_by_id(conn: &Connection, id: i64) -> Result<Option<WeaponStats>> {
        query_one_by_id(conn, Self::TABLE, Self::COLUMNS, id, Self::row_to_model)
    }

    /// Get weapon stats by gear ID
    pub fn get_by_gear_id(conn: &Connection, gear_id: i64) -> Result<Option<WeaponStats>> {
        let sql = format!("SELECT {} FROM {} WHERE gear_id = ?", Self::COLUMNS, Self::TABLE);
        let mut stmt = conn.prepare(&sql)?;
        let mut rows = stmt.query(params![gear_id])?;

        match rows.next()? {
            Some(row) => Ok(Some(Self::row_to_model(row)?)),
            None => Ok(None),
        }
    }

    /// Convert a database row to a WeaponStats model
    fn row_to_model(row: &Row) -> rusqlite::Result<WeaponStats> {
        Ok(WeaponStats {
            id: row.get(0)?,
            gear_id: row.get(1)?,
            damage: row.get(2)?,
            ap: row.get(3)?,
            range_short: row.get(4)?,
            range_medium: row.get(5)?,
            range_long: row.get(6)?,
            rof: row.get(7)?,
            shots: row.get(8)?,
            min_strength: row.get(9)?,
            is_two_handed: row.get(10)?,
            reach: row.get(11)?,
            blast_template: row.get(12)?,
            notes: row.get(13)?,
            created_at: row.get(14)?,
            updated_at: row.get(15)?,
        })
    }
}
