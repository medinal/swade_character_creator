use rusqlite::Connection;

use crate::error::Result;
use crate::models::{
    AncestryRequirement, ArcaneBackgroundRequirement, EdgeRequirement, GearRequirement,
    HindranceRequirement, PowerRequirement, RequirementExpression,
};
use crate::repositories::{
    AncestryRequirementRepository, ArcaneBackgroundRequirementRepository,
    EdgeRequirementRepository, GearRequirementRepository, HindranceRequirementRepository,
    PowerRequirementRepository, RequirementExpressionRepository, RequirementRepository,
};
use crate::views::RequirementTree;

/// Trait for link models that have a requirement_expression_id field.
trait HasRequirementExpressionId {
    fn requirement_expression_id(&self) -> i64;
}

impl HasRequirementExpressionId for EdgeRequirement {
    fn requirement_expression_id(&self) -> i64 {
        self.requirement_expression_id
    }
}

impl HasRequirementExpressionId for HindranceRequirement {
    fn requirement_expression_id(&self) -> i64 {
        self.requirement_expression_id
    }
}

impl HasRequirementExpressionId for AncestryRequirement {
    fn requirement_expression_id(&self) -> i64 {
        self.requirement_expression_id
    }
}

impl HasRequirementExpressionId for ArcaneBackgroundRequirement {
    fn requirement_expression_id(&self) -> i64 {
        self.requirement_expression_id
    }
}

impl HasRequirementExpressionId for PowerRequirement {
    fn requirement_expression_id(&self) -> i64 {
        self.requirement_expression_id
    }
}

impl HasRequirementExpressionId for GearRequirement {
    fn requirement_expression_id(&self) -> i64 {
        self.requirement_expression_id
    }
}

pub struct RequirementService;

impl RequirementService {
    /// Get the requirement tree for an edge.
    pub fn get_for_edge(conn: &Connection, edge_id: i64) -> Result<RequirementTree> {
        let links = EdgeRequirementRepository::get_by_edge_id(conn, edge_id)?;
        Self::build_requirement_tree(conn, links)
    }

    /// Get the requirement tree for a hindrance.
    pub fn get_for_hindrance(conn: &Connection, hindrance_id: i64) -> Result<RequirementTree> {
        let links = HindranceRequirementRepository::get_by_hindrance_id(conn, hindrance_id)?;
        Self::build_requirement_tree(conn, links)
    }

    /// Get the requirement tree for an ancestry.
    pub fn get_for_ancestry(conn: &Connection, ancestry_id: i64) -> Result<RequirementTree> {
        let links = AncestryRequirementRepository::get_by_ancestry_id(conn, ancestry_id)?;
        Self::build_requirement_tree(conn, links)
    }

    /// Get the requirement tree for an arcane background.
    pub fn get_for_arcane_background(
        conn: &Connection,
        arcane_background_id: i64,
    ) -> Result<RequirementTree> {
        let links = ArcaneBackgroundRequirementRepository::get_by_arcane_background_id(
            conn,
            arcane_background_id,
        )?;
        Self::build_requirement_tree(conn, links)
    }

    /// Get the requirement tree for a power.
    pub fn get_for_power(conn: &Connection, power_id: i64) -> Result<RequirementTree> {
        let links = PowerRequirementRepository::get_by_power_id(conn, power_id)?;
        Self::build_requirement_tree(conn, links)
    }

    /// Get the requirement tree for gear.
    /// Note: Gear requirements are interpreted as graduated penalties rather than hard requirements.
    pub fn get_for_gear(conn: &Connection, gear_id: i64) -> Result<RequirementTree> {
        let links = GearRequirementRepository::get_by_gear_id(conn, gear_id)?;
        Self::build_requirement_tree(conn, links)
    }

    /// Build a combined requirement tree from a collection of requirement links.
    /// If multiple expressions exist, they are combined with AND.
    fn build_requirement_tree<T: HasRequirementExpressionId>(
        conn: &Connection,
        links: Vec<T>,
    ) -> Result<RequirementTree> {
        if links.is_empty() {
            return Ok(RequirementTree::none());
        }

        let all_expressions = RequirementExpressionRepository::get_all(conn)?;

        let mut trees = Vec::new();
        for link in links {
            if let Some(tree) =
                Self::build_tree(conn, link.requirement_expression_id(), &all_expressions)?
            {
                trees.push(tree);
            }
        }

        match trees.len() {
            0 => Ok(RequirementTree::none()),
            1 => Ok(trees.remove(0)),
            _ => Ok(RequirementTree::and(trees)),
        }
    }

    /// Build a requirement tree from a root expression ID
    fn build_tree(
        conn: &Connection,
        root_id: i64,
        all_expressions: &[RequirementExpression],
    ) -> Result<Option<RequirementTree>> {
        let root = all_expressions.iter().find(|e| e.id == root_id);

        let Some(root) = root else {
            return Ok(None);
        };

        Self::build_node(conn, root, all_expressions)
    }

    /// Recursively build a tree node
    fn build_node(
        conn: &Connection,
        expr: &RequirementExpression,
        all_expressions: &[RequirementExpression],
    ) -> Result<Option<RequirementTree>> {
        match expr.node_type.as_str() {
            "requirement" => {
                // Leaf node - load the actual requirement
                if let Some(req_id) = expr.requirement_id
                    && let Some(requirement) = RequirementRepository::get_by_id(conn, req_id)? {
                        return Ok(Some(RequirementTree::leaf(requirement)));
                    }
                Ok(None)
            }
            "and" => {
                let children = Self::build_children(conn, expr.id, all_expressions)?;
                if children.is_empty() {
                    Ok(Some(RequirementTree::none()))
                } else {
                    Ok(Some(RequirementTree::and(children)))
                }
            }
            "or" => {
                let children = Self::build_children(conn, expr.id, all_expressions)?;
                if children.is_empty() {
                    Ok(Some(RequirementTree::none()))
                } else {
                    Ok(Some(RequirementTree::or(children)))
                }
            }
            "not" => {
                let children = Self::build_children(conn, expr.id, all_expressions)?;
                if let Some(child) = children.into_iter().next() {
                    Ok(Some(RequirementTree::negate(child)))
                } else {
                    Ok(None)
                }
            }
            _ => Ok(None),
        }
    }

    /// Build child nodes for a parent expression
    fn build_children(
        conn: &Connection,
        parent_id: i64,
        all_expressions: &[RequirementExpression],
    ) -> Result<Vec<RequirementTree>> {
        // Find children and sort by position
        let mut children: Vec<_> = all_expressions
            .iter()
            .filter(|e| e.parent_id == Some(parent_id))
            .collect();
        children.sort_by_key(|e| e.position);

        let mut trees = Vec::new();
        for child in children {
            if let Some(tree) = Self::build_node(conn, child, all_expressions)? {
                trees.push(tree);
            }
        }

        Ok(trees)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::setup_test_db;
    use crate::views::RequirementNode;
    use rusqlite::params;

    fn insert_test_edge(conn: &Connection, id: i64) {
        conn.execute(
            "INSERT INTO edges (id, name, background, source, description,
                               can_take_multiple_times, created_at, updated_at)
             VALUES (?, 'Test Edge', 'Background', 'core', 'Test', 0, '2024-01-01', '2024-01-01')",
            params![id],
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
    fn get_for_edge_returns_none_when_no_requirements() {
        let conn = setup_test_db();
        insert_test_edge(&conn, 1);

        let tree = RequirementService::get_for_edge(&conn, 1).unwrap();

        assert!(tree.is_empty());
    }

    #[test]
    fn get_for_edge_returns_single_requirement() {
        let conn = setup_test_db();
        insert_test_edge(&conn, 1);
        insert_requirement(&conn, 1, "rank", "Novice");
        insert_requirement_expression(&conn, 1, None, "requirement", Some(1), 0);
        insert_edge_requirement(&conn, 1, 1);

        let tree = RequirementService::get_for_edge(&conn, 1).unwrap();

        assert!(!tree.is_empty());
        match tree.node {
            RequirementNode::Leaf(req) => {
                assert_eq!(req.description, "Novice");
            }
            _ => panic!("Expected leaf node"),
        }
    }

    #[test]
    fn get_for_edge_returns_and_tree() {
        let conn = setup_test_db();
        insert_test_edge(&conn, 1);
        insert_requirement(&conn, 1, "rank", "Novice");
        insert_requirement(&conn, 2, "attribute", "Agility d8");

        // AND node with two children
        insert_requirement_expression(&conn, 1, None, "and", None, 0);
        insert_requirement_expression(&conn, 2, Some(1), "requirement", Some(1), 0);
        insert_requirement_expression(&conn, 3, Some(1), "requirement", Some(2), 1);
        insert_edge_requirement(&conn, 1, 1);

        let tree = RequirementService::get_for_edge(&conn, 1).unwrap();

        match tree.node {
            RequirementNode::And(children) => {
                assert_eq!(children.len(), 2);
            }
            _ => panic!("Expected AND node"),
        }
    }

    #[test]
    fn get_for_edge_returns_or_tree() {
        let conn = setup_test_db();
        insert_test_edge(&conn, 1);
        insert_requirement(&conn, 1, "skill", "Fighting d8");
        insert_requirement(&conn, 2, "skill", "Shooting d8");

        // OR node with two children
        insert_requirement_expression(&conn, 1, None, "or", None, 0);
        insert_requirement_expression(&conn, 2, Some(1), "requirement", Some(1), 0);
        insert_requirement_expression(&conn, 3, Some(1), "requirement", Some(2), 1);
        insert_edge_requirement(&conn, 1, 1);

        let tree = RequirementService::get_for_edge(&conn, 1).unwrap();

        match tree.node {
            RequirementNode::Or(children) => {
                assert_eq!(children.len(), 2);
            }
            _ => panic!("Expected OR node"),
        }
    }

    #[test]
    fn get_for_edge_combines_multiple_expressions_with_and() {
        let conn = setup_test_db();
        insert_test_edge(&conn, 1);
        insert_requirement(&conn, 1, "rank", "Novice");
        insert_requirement(&conn, 2, "attribute", "Agility d8");

        // Two separate requirement expressions linked to the edge
        insert_requirement_expression(&conn, 1, None, "requirement", Some(1), 0);
        insert_requirement_expression(&conn, 2, None, "requirement", Some(2), 0);
        insert_edge_requirement(&conn, 1, 1);
        insert_edge_requirement(&conn, 1, 2);

        let tree = RequirementService::get_for_edge(&conn, 1).unwrap();

        // Should be combined with AND
        match tree.node {
            RequirementNode::And(children) => {
                assert_eq!(children.len(), 2);
            }
            _ => panic!("Expected AND node combining multiple expressions"),
        }
    }

    #[test]
    fn get_for_edge_returns_empty_for_nonexistent_edge() {
        let conn = setup_test_db();

        let tree = RequirementService::get_for_edge(&conn, 999).unwrap();

        assert!(tree.is_empty());
    }

    fn insert_test_hindrance(conn: &Connection, id: i64) {
        // severity must be lowercase per CHECK constraint
        conn.execute(
            "INSERT INTO hindrances (id, name, severity, point_value, source, description,
                                    created_at, updated_at)
             VALUES (?, 'Test Hindrance', 'major', 2, 'core', 'Test', '2024-01-01', '2024-01-01')",
            params![id],
        )
        .unwrap();
    }

    fn insert_hindrance_requirement(conn: &Connection, hindrance_id: i64, expression_id: i64) {
        conn.execute(
            "INSERT INTO hindrance_requirements (hindrance_id, requirement_expression_id, created_at, updated_at)
             VALUES (?, ?, '2024-01-01', '2024-01-01')",
            params![hindrance_id, expression_id],
        )
        .unwrap();
    }

    #[test]
    fn get_for_hindrance_returns_none_when_no_requirements() {
        let conn = setup_test_db();
        insert_test_hindrance(&conn, 1);

        let tree = RequirementService::get_for_hindrance(&conn, 1).unwrap();

        assert!(tree.is_empty());
    }

    #[test]
    fn get_for_hindrance_returns_single_requirement() {
        let conn = setup_test_db();
        insert_test_hindrance(&conn, 1);
        insert_requirement(&conn, 1, "rank", "Novice");
        insert_requirement_expression(&conn, 1, None, "requirement", Some(1), 0);
        insert_hindrance_requirement(&conn, 1, 1);

        let tree = RequirementService::get_for_hindrance(&conn, 1).unwrap();

        assert!(!tree.is_empty());
        match tree.node {
            RequirementNode::Leaf(req) => {
                assert_eq!(req.description, "Novice");
            }
            _ => panic!("Expected leaf node"),
        }
    }

    #[test]
    fn get_for_hindrance_returns_empty_for_nonexistent_hindrance() {
        let conn = setup_test_db();

        let tree = RequirementService::get_for_hindrance(&conn, 999).unwrap();

        assert!(tree.is_empty());
    }
}
