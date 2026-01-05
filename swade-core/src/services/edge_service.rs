use rusqlite::Connection;

use crate::error::Result;
use crate::repositories::EdgeRepository;
use crate::services::{ModifierService, RequirementService};
use crate::views::EdgeView;

pub struct EdgeService;

impl EdgeService {
    pub fn get_all(conn: &Connection) -> Result<Vec<EdgeView>> {
        let edges = EdgeRepository::get_all(conn)?;

        let mut views = Vec::new();
        for edge in edges {
            let modifiers = ModifierService::get_for_edge(conn, edge.id)?;
            let requirements = RequirementService::get_for_edge(conn, edge.id)?;
            views.push(EdgeView::new(edge, modifiers, requirements));
        }

        Ok(views)
    }

    pub fn get_by_id(conn: &Connection, id: i64) -> Result<Option<EdgeView>> {
        let edge = EdgeRepository::get_by_id(conn, id)?;

        match edge {
            Some(edge) => {
                let modifiers = ModifierService::get_for_edge(conn, edge.id)?;
                let requirements = RequirementService::get_for_edge(conn, edge.id)?;
                Ok(Some(EdgeView::new(edge, modifiers, requirements)))
            }
            None => Ok(None),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::setup_test_db;
    use crate::views::RequirementNode;
    use rusqlite::params;

    fn insert_test_edge(conn: &Connection, id: i64, name: &str, category: &str) {
        conn.execute(
            "INSERT INTO edges (id, name, background, source, description,
                               can_take_multiple_times, created_at, updated_at)
             VALUES (?, ?, ?, 'core', 'Test description', 0, '2024-01-01', '2024-01-01')",
            params![id, name, category],
        )
        .unwrap();
    }

    fn insert_modifier(conn: &Connection, id: i64, description: &str) {
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

    fn insert_requirement(conn: &Connection, id: i64, req_type: &str, description: &str) {
        conn.execute(
            "INSERT INTO requirements (id, requirement_type, target_id, value, description,
                                       created_at, updated_at)
             VALUES (?, ?, 1, 1, ?, '2024-01-01', '2024-01-01')",
            params![id, req_type, description],
        )
        .unwrap();
    }

    fn insert_requirement_expression(
        conn: &Connection,
        id: i64,
        parent_id: Option<i64>,
        node_type: &str,
        requirement_id: Option<i64>,
        position: i64,
    ) {
        conn.execute(
            "INSERT INTO requirement_expressions (id, parent_id, node_type, requirement_id,
                                                  position, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, '2024-01-01', '2024-01-01')",
            params![id, parent_id, node_type, requirement_id, position],
        )
        .unwrap();
    }

    fn insert_edge_requirement(conn: &Connection, edge_id: i64, expression_id: i64) {
        conn.execute(
            "INSERT INTO edge_requirements (edge_id, requirement_expression_id, created_at, updated_at)
             VALUES (?, ?, '2024-01-01', '2024-01-01')",
            params![edge_id, expression_id],
        )
        .unwrap();
    }

    #[test]
    fn get_all_returns_empty_when_no_edges() {
        let conn = setup_test_db();

        let edges = EdgeService::get_all(&conn).unwrap();

        assert!(edges.is_empty());
    }

    #[test]
    fn get_all_returns_edges() {
        let conn = setup_test_db();
        insert_test_edge(&conn, 1, "Alertness", "Background");
        insert_test_edge(&conn, 2, "Brawny", "Background");

        let edges = EdgeService::get_all(&conn).unwrap();

        assert_eq!(edges.len(), 2);
    }

    #[test]
    fn get_all_includes_modifiers() {
        let conn = setup_test_db();
        insert_test_edge(&conn, 1, "Alertness", "Background");
        insert_modifier(&conn, 1, "+2 to Notice");
        insert_edge_modifier(&conn, 1, 1);

        let edges = EdgeService::get_all(&conn).unwrap();

        assert_eq!(edges.len(), 1);
        assert_eq!(edges[0].modifiers.len(), 1);
        assert_eq!(edges[0].modifiers[0].description, "+2 to Notice");
    }

    #[test]
    fn get_all_includes_requirements() {
        let conn = setup_test_db();
        insert_test_edge(&conn, 1, "Improved Alertness", "Background");
        insert_requirement(&conn, 1, "edge", "Alertness");
        insert_requirement_expression(&conn, 1, None, "requirement", Some(1), 0);
        insert_edge_requirement(&conn, 1, 1);

        let edges = EdgeService::get_all(&conn).unwrap();

        assert_eq!(edges.len(), 1);
        assert!(!edges[0].requirements.is_empty());
    }

    #[test]
    fn get_by_id_returns_none_when_not_found() {
        let conn = setup_test_db();

        let edge = EdgeService::get_by_id(&conn, 999).unwrap();

        assert!(edge.is_none());
    }

    #[test]
    fn get_by_id_returns_edge_with_modifiers_and_requirements() {
        let conn = setup_test_db();
        insert_test_edge(&conn, 1, "Alertness", "Background");
        insert_modifier(&conn, 1, "+2 to Notice");
        insert_edge_modifier(&conn, 1, 1);
        insert_requirement(&conn, 1, "rank", "Novice");
        insert_requirement_expression(&conn, 1, None, "requirement", Some(1), 0);
        insert_edge_requirement(&conn, 1, 1);

        let edge = EdgeService::get_by_id(&conn, 1).unwrap();

        assert!(edge.is_some());
        let edge = edge.unwrap();
        assert_eq!(edge.name, "Alertness");
        assert_eq!(edge.modifiers.len(), 1);
        assert!(!edge.requirements.is_empty());
    }

    #[test]
    fn get_by_id_maps_fields_correctly() {
        let conn = setup_test_db();
        insert_test_edge(&conn, 1, "Alertness", "Background");

        let edge = EdgeService::get_by_id(&conn, 1).unwrap().unwrap();

        assert_eq!(edge.id, 1);
        assert_eq!(edge.name, "Alertness");
        assert_eq!(edge.category, crate::views::EdgeCategory::Background);
        assert_eq!(edge.source, "core");
        assert_eq!(edge.description, "Test description");
        assert_eq!(edge.can_take_multiple_times, false);
    }

    #[test]
    fn get_all_with_complex_requirements() {
        let conn = setup_test_db();
        insert_test_edge(&conn, 1, "Complex Edge", "Combat");

        // Create an AND with two requirements
        insert_requirement(&conn, 1, "rank", "Seasoned");
        insert_requirement(&conn, 2, "attribute", "Agility d8");
        insert_requirement_expression(&conn, 1, None, "and", None, 0);
        insert_requirement_expression(&conn, 2, Some(1), "requirement", Some(1), 0);
        insert_requirement_expression(&conn, 3, Some(1), "requirement", Some(2), 1);
        insert_edge_requirement(&conn, 1, 1);

        let edges = EdgeService::get_all(&conn).unwrap();

        assert_eq!(edges.len(), 1);
        match &edges[0].requirements.node {
            RequirementNode::And(children) => {
                assert_eq!(children.len(), 2);
            }
            _ => panic!("Expected AND node"),
        }
    }
}
