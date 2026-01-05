use rusqlite::Connection;

use crate::error::Result;
use crate::repositories::PowerRepository;
use crate::services::{ModifierService, RequirementService};
use crate::views::PowerView;

pub struct PowerService;

impl PowerService {
    pub fn get_all(conn: &Connection) -> Result<Vec<PowerView>> {
        let powers = PowerRepository::get_all(conn)?;

        let mut views = Vec::new();
        for power in powers {
            let modifiers = ModifierService::get_for_power(conn, power.id)?;
            let requirements = RequirementService::get_for_power(conn, power.id)?;

            views.push(PowerView {
                id: power.id,
                name: power.name,
                power_points: power.power_points,
                range: power.range,
                duration: power.duration,
                source: power.source,
                description: power.description,
                modifiers,
                requirements,
            });
        }

        Ok(views)
    }

    pub fn get_by_id(conn: &Connection, id: i64) -> Result<Option<PowerView>> {
        let power = match PowerRepository::get_by_id(conn, id)? {
            Some(p) => p,
            None => return Ok(None),
        };

        let modifiers = ModifierService::get_for_power(conn, power.id)?;
        let requirements = RequirementService::get_for_power(conn, power.id)?;

        Ok(Some(PowerView {
            id: power.id,
            name: power.name,
            power_points: power.power_points,
            range: power.range,
            duration: power.duration,
            source: power.source,
            description: power.description,
            modifiers,
            requirements,
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::setup_test_db;
    use rusqlite::params;

    fn insert_test_power(conn: &Connection, id: i64, name: &str) {
        conn.execute(
            "INSERT INTO powers (id, name, power_points, range, duration, source, description,
                                created_at, updated_at)
             VALUES (?, ?, 2, 'Smarts', 'Instant', 'core', 'Test power', '2024-01-01', '2024-01-01')",
            params![id, name],
        )
        .unwrap();
    }

    fn insert_test_modifier(conn: &Connection, id: i64, description: &str) {
        conn.execute(
            "INSERT INTO modifiers (id, target_type, target_identifier, value_type,
                                   value, description, created_at, updated_at)
             VALUES (?, 'skill', 'Spellcasting', 'roll_bonus', 2, ?, '2024-01-01', '2024-01-01')",
            params![id, description],
        )
        .unwrap();
    }

    fn insert_power_modifier(conn: &Connection, power_id: i64, modifier_id: i64) {
        conn.execute(
            "INSERT INTO power_modifiers (power_id, modifier_id, created_at, updated_at)
             VALUES (?, ?, '2024-01-01', '2024-01-01')",
            params![power_id, modifier_id],
        )
        .unwrap();
    }

    #[test]
    fn get_all_returns_empty_when_no_powers() {
        let conn = setup_test_db();

        let powers = PowerService::get_all(&conn).unwrap();

        assert_eq!(powers.len(), 0);
    }

    #[test]
    fn get_all_returns_powers_with_modifiers() {
        let conn = setup_test_db();
        insert_test_power(&conn, 1, "Bolt");
        insert_test_modifier(&conn, 1, "+2 to cast");
        insert_power_modifier(&conn, 1, 1);

        let powers = PowerService::get_all(&conn).unwrap();

        assert_eq!(powers.len(), 1);
        assert_eq!(powers[0].name, "Bolt");
        assert_eq!(powers[0].modifiers.len(), 1);
        assert_eq!(powers[0].modifiers[0].description, "+2 to cast");
    }

    #[test]
    fn get_all_returns_powers_sorted_by_name() {
        let conn = setup_test_db();
        insert_test_power(&conn, 1, "Healing");
        insert_test_power(&conn, 2, "Barrier");
        insert_test_power(&conn, 3, "Bolt");

        let powers = PowerService::get_all(&conn).unwrap();

        assert_eq!(powers.len(), 3);
        assert_eq!(powers[0].name, "Barrier");
        assert_eq!(powers[1].name, "Bolt");
        assert_eq!(powers[2].name, "Healing");
    }

    #[test]
    fn get_by_id_returns_power_when_found() {
        let conn = setup_test_db();
        insert_test_power(&conn, 1, "Bolt");

        let power = PowerService::get_by_id(&conn, 1).unwrap();

        assert!(power.is_some());
        let power = power.unwrap();
        assert_eq!(power.id, 1);
        assert_eq!(power.name, "Bolt");
        assert!(power.requirements.is_empty());
    }

    #[test]
    fn get_by_id_returns_none_when_not_found() {
        let conn = setup_test_db();

        let power = PowerService::get_by_id(&conn, 999).unwrap();

        assert!(power.is_none());
    }

    #[test]
    fn get_by_id_includes_modifiers() {
        let conn = setup_test_db();
        insert_test_power(&conn, 1, "Healing");
        insert_test_modifier(&conn, 1, "Heals 1 wound");
        insert_power_modifier(&conn, 1, 1);

        let power = PowerService::get_by_id(&conn, 1).unwrap().unwrap();

        assert_eq!(power.modifiers.len(), 1);
        assert_eq!(power.modifiers[0].description, "Heals 1 wound");
    }
}
