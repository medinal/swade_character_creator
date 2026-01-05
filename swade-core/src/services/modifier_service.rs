use rusqlite::Connection;

use crate::error::Result;
use crate::models::{
    AncestryModifier, EdgeModifier, GearModifier, HindranceModifier, Modifier, PowerModifier,
};
use crate::repositories::{
    AncestryModifierRepository, EdgeModifierRepository, GearModifierRepository,
    HindranceModifierRepository, ModifierRepository, PowerModifierRepository,
};

/// Trait for link models that have a modifier_id field.
trait HasModifierId {
    fn modifier_id(&self) -> i64;
}

impl HasModifierId for EdgeModifier {
    fn modifier_id(&self) -> i64 {
        self.modifier_id
    }
}

impl HasModifierId for HindranceModifier {
    fn modifier_id(&self) -> i64 {
        self.modifier_id
    }
}

impl HasModifierId for AncestryModifier {
    fn modifier_id(&self) -> i64 {
        self.modifier_id
    }
}

impl HasModifierId for PowerModifier {
    fn modifier_id(&self) -> i64 {
        self.modifier_id
    }
}

impl HasModifierId for GearModifier {
    fn modifier_id(&self) -> i64 {
        self.modifier_id
    }
}

/// Fetches modifiers from a collection of link records.
fn collect_modifiers<T: HasModifierId>(conn: &Connection, links: Vec<T>) -> Result<Vec<Modifier>> {
    let mut modifiers = Vec::new();
    for link in links {
        if let Some(modifier) = ModifierRepository::get_by_id(conn, link.modifier_id())? {
            modifiers.push(modifier);
        }
    }
    Ok(modifiers)
}

pub struct ModifierService;

impl ModifierService {
    pub fn get_for_edge(conn: &Connection, edge_id: i64) -> Result<Vec<Modifier>> {
        let links = EdgeModifierRepository::get_by_edge_id(conn, edge_id)?;
        collect_modifiers(conn, links)
    }

    pub fn get_for_hindrance(conn: &Connection, hindrance_id: i64) -> Result<Vec<Modifier>> {
        let links = HindranceModifierRepository::get_by_hindrance_id(conn, hindrance_id)?;
        collect_modifiers(conn, links)
    }

    pub fn get_for_ancestry(conn: &Connection, ancestry_id: i64) -> Result<Vec<Modifier>> {
        let links = AncestryModifierRepository::get_by_ancestry_id(conn, ancestry_id)?;
        collect_modifiers(conn, links)
    }

    pub fn get_for_power(conn: &Connection, power_id: i64) -> Result<Vec<Modifier>> {
        let links = PowerModifierRepository::get_by_power_id(conn, power_id)?;
        collect_modifiers(conn, links)
    }

    pub fn get_for_gear(conn: &Connection, gear_id: i64) -> Result<Vec<Modifier>> {
        let links = GearModifierRepository::get_by_gear_id(conn, gear_id)?;
        collect_modifiers(conn, links)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::setup_test_db;
    use rusqlite::params;

    fn insert_test_edge(conn: &Connection, id: i64, name: &str) {
        conn.execute(
            "INSERT INTO edges (id, name, background, source, description,
                               can_take_multiple_times, created_at, updated_at)
             VALUES (?, ?, 'Background', 'core', 'Test', 0, '2024-01-01', '2024-01-01')",
            params![id, name],
        )
        .unwrap();
    }

    fn insert_test_modifier(conn: &Connection, id: i64, description: &str) {
        conn.execute(
            "INSERT INTO modifiers (id, target_type, target_identifier, value_type,
                                   value, description, created_at, updated_at)
             VALUES (?, 'skill', 'Notice', 'roll_bonus', 2, ?, '2024-01-01', '2024-01-01')",
            params![id, description],
        )
        .unwrap();
    }

    fn insert_edge_modifier(conn: &Connection, edge_id: i64, modifier_id: i64) {
        conn.execute(
            "INSERT INTO edge_modifiers (edge_id, modifier_id, created_at, updated_at)
             VALUES (?, ?, '2024-01-01', '2024-01-01')",
            params![edge_id, modifier_id],
        )
        .unwrap();
    }

    #[test]
    fn get_for_edge_returns_modifiers() {
        let conn = setup_test_db();
        insert_test_edge(&conn, 1, "Alertness");
        insert_test_modifier(&conn, 1, "+2 to Notice");
        insert_edge_modifier(&conn, 1, 1);

        let modifiers = ModifierService::get_for_edge(&conn, 1).unwrap();

        assert_eq!(modifiers.len(), 1);
        assert_eq!(modifiers[0].description, "+2 to Notice");
    }

    #[test]
    fn get_for_edge_returns_multiple_modifiers() {
        let conn = setup_test_db();
        insert_test_edge(&conn, 1, "Brawny");
        insert_test_modifier(&conn, 1, "+1 Toughness");
        insert_test_modifier(&conn, 2, "+1 Load Limit");
        insert_edge_modifier(&conn, 1, 1);
        insert_edge_modifier(&conn, 1, 2);

        let modifiers = ModifierService::get_for_edge(&conn, 1).unwrap();

        assert_eq!(modifiers.len(), 2);
    }

    #[test]
    fn get_for_edge_returns_empty_when_no_modifiers() {
        let conn = setup_test_db();
        insert_test_edge(&conn, 1, "Luck");

        let modifiers = ModifierService::get_for_edge(&conn, 1).unwrap();

        assert_eq!(modifiers.len(), 0);
    }

    #[test]
    fn get_for_edge_returns_empty_for_nonexistent_edge() {
        let conn = setup_test_db();

        let modifiers = ModifierService::get_for_edge(&conn, 999).unwrap();

        assert_eq!(modifiers.len(), 0);
    }

    #[test]
    fn get_for_edge_only_returns_modifiers_for_specified_edge() {
        let conn = setup_test_db();
        insert_test_edge(&conn, 1, "Alertness");
        insert_test_edge(&conn, 2, "Danger Sense");
        insert_test_modifier(&conn, 1, "+2 to Notice");
        insert_test_modifier(&conn, 2, "Danger Sense bonus");
        insert_edge_modifier(&conn, 1, 1);
        insert_edge_modifier(&conn, 2, 2);

        let modifiers = ModifierService::get_for_edge(&conn, 1).unwrap();

        assert_eq!(modifiers.len(), 1);
        assert_eq!(modifiers[0].description, "+2 to Notice");
    }

    fn insert_test_hindrance(conn: &Connection, id: i64, name: &str) {
        // severity must be lowercase per CHECK constraint
        conn.execute(
            "INSERT INTO hindrances (id, name, severity, point_value, source, description,
                                    created_at, updated_at)
             VALUES (?, ?, 'major', 2, 'core', 'Test', '2024-01-01', '2024-01-01')",
            params![id, name],
        )
        .unwrap();
    }

    fn insert_hindrance_modifier(conn: &Connection, hindrance_id: i64, modifier_id: i64) {
        conn.execute(
            "INSERT INTO hindrance_modifiers (hindrance_id, modifier_id, created_at, updated_at)
             VALUES (?, ?, '2024-01-01', '2024-01-01')",
            params![hindrance_id, modifier_id],
        )
        .unwrap();
    }

    #[test]
    fn get_for_hindrance_returns_modifiers() {
        let conn = setup_test_db();
        insert_test_hindrance(&conn, 1, "Bad Eyes");
        insert_test_modifier(&conn, 1, "-2 to Notice (sight)");
        insert_hindrance_modifier(&conn, 1, 1);

        let modifiers = ModifierService::get_for_hindrance(&conn, 1).unwrap();

        assert_eq!(modifiers.len(), 1);
        assert_eq!(modifiers[0].description, "-2 to Notice (sight)");
    }

    #[test]
    fn get_for_hindrance_returns_empty_when_no_modifiers() {
        let conn = setup_test_db();
        insert_test_hindrance(&conn, 1, "Curious");

        let modifiers = ModifierService::get_for_hindrance(&conn, 1).unwrap();

        assert_eq!(modifiers.len(), 0);
    }

    #[test]
    fn get_for_hindrance_returns_empty_for_nonexistent_hindrance() {
        let conn = setup_test_db();

        let modifiers = ModifierService::get_for_hindrance(&conn, 999).unwrap();

        assert_eq!(modifiers.len(), 0);
    }
}
