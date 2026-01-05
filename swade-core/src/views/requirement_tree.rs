use serde::{Deserialize, Serialize};

use crate::models::Requirement;

/// A node in a requirement expression tree.
/// Represents boolean logic (AND, OR, NOT) or a leaf requirement.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub enum RequirementNode {
    /// All children must be satisfied
    And(Vec<RequirementTree>),
    /// At least one child must be satisfied
    Or(Vec<RequirementTree>),
    /// The child must NOT be satisfied
    Not(Box<RequirementTree>),
    /// A leaf requirement to evaluate
    Leaf(Requirement),
}

/// Status of a single requirement evaluation.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct RequirementStatus {
    /// Human-readable description of the requirement
    pub description: String,
    /// Whether this requirement is met
    pub is_met: bool,
}

/// Context for evaluating requirements against a character.
/// This struct holds the minimal data needed to check requirements,
/// avoiding circular dependencies with CharacterView.
#[derive(Debug, Clone)]
pub struct RequirementContext {
    /// Character's current rank ID
    pub rank_id: i64,
    /// Whether the character is a Wild Card
    pub is_wild_card: bool,
    /// Map of attribute_id -> effective die size (4, 6, 8, 10, 12)
    pub attribute_dies: std::collections::HashMap<i64, u8>,
    /// Map of skill_id -> effective die size (None if untrained)
    pub skill_dies: std::collections::HashMap<i64, Option<u8>>,
    /// Set of edge IDs the character has
    pub edge_ids: std::collections::HashSet<i64>,
    /// Set of arcane background IDs the character has
    pub arcane_background_ids: std::collections::HashSet<i64>,
    /// Map of skill_id -> effective die size for arcane skills
    pub arcane_skill_dies: std::collections::HashMap<i64, Option<u8>>,
}

/// A tree structure representing requirements with boolean logic.
/// Built from requirement_expressions and requirements tables.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct RequirementTree {
    pub node: RequirementNode,
}

impl RequirementTree {
    pub fn new(node: RequirementNode) -> Self {
        Self { node }
    }

    /// Create a tree with a single leaf requirement
    pub fn leaf(requirement: Requirement) -> Self {
        Self {
            node: RequirementNode::Leaf(requirement),
        }
    }

    /// Create an AND node with multiple children
    pub fn and(children: Vec<RequirementTree>) -> Self {
        Self {
            node: RequirementNode::And(children),
        }
    }

    /// Create an OR node with multiple children
    pub fn or(children: Vec<RequirementTree>) -> Self {
        Self {
            node: RequirementNode::Or(children),
        }
    }

    /// Create a NOT node wrapping a child
    pub fn negate(child: RequirementTree) -> Self {
        Self {
            node: RequirementNode::Not(Box::new(child)),
        }
    }

    /// Create an empty tree (no requirements - always satisfied)
    pub fn none() -> Self {
        Self {
            node: RequirementNode::And(vec![]),
        }
    }

    /// Check if this tree has no requirements
    pub fn is_empty(&self) -> bool {
        matches!(&self.node, RequirementNode::And(children) if children.is_empty())
    }

    /// Evaluate whether all requirements in this tree are met.
    pub fn evaluate(&self, ctx: &RequirementContext) -> bool {
        match &self.node {
            RequirementNode::And(children) => children.iter().all(|c| c.evaluate(ctx)),
            RequirementNode::Or(children) => {
                children.is_empty() || children.iter().any(|c| c.evaluate(ctx))
            }
            RequirementNode::Not(child) => !child.evaluate(ctx),
            RequirementNode::Leaf(req) => Self::evaluate_requirement(req, ctx),
        }
    }

    /// Evaluate and return detailed status of all leaf requirements.
    /// For AND nodes, returns all children's statuses.
    /// For OR nodes, returns all children's statuses.
    /// For NOT nodes, returns the negated child's status.
    /// For Leaf nodes, returns the single requirement's status.
    pub fn evaluate_detailed(&self, ctx: &RequirementContext) -> Vec<RequirementStatus> {
        let mut statuses = Vec::new();
        self.collect_leaf_statuses(ctx, &mut statuses, false);
        statuses
    }

    /// Recursively collect leaf requirement statuses.
    /// `negated` tracks whether we're inside a NOT node.
    fn collect_leaf_statuses(
        &self,
        ctx: &RequirementContext,
        statuses: &mut Vec<RequirementStatus>,
        negated: bool,
    ) {
        match &self.node {
            RequirementNode::And(children) | RequirementNode::Or(children) => {
                for child in children {
                    child.collect_leaf_statuses(ctx, statuses, negated);
                }
            }
            RequirementNode::Not(child) => {
                child.collect_leaf_statuses(ctx, statuses, !negated);
            }
            RequirementNode::Leaf(req) => {
                let mut is_met = Self::evaluate_requirement(req, ctx);
                if negated {
                    is_met = !is_met;
                }
                statuses.push(RequirementStatus {
                    description: req.description.clone(),
                    is_met,
                });
            }
        }
    }

    /// Evaluate a single requirement against the context.
    fn evaluate_requirement(req: &Requirement, ctx: &RequirementContext) -> bool {
        match req.requirement_type.as_str() {
            "rank" => {
                // target_id is the required rank_id
                // Character must be at or above this rank
                // Ranks: 1=Novice, 2=Seasoned, 3=Veteran, 4=Heroic, 5=Legendary
                if let Some(required_rank_id) = req.target_id {
                    ctx.rank_id >= required_rank_id
                } else {
                    true // No specific rank required
                }
            }
            "attribute" => {
                // target_id is attribute_id, value is required die size
                if let (Some(attr_id), Some(required_die)) = (req.target_id, req.value) {
                    ctx.attribute_dies
                        .get(&attr_id)
                        .map(|&die| die >= required_die as u8)
                        .unwrap_or(false)
                } else {
                    true
                }
            }
            "skill" => {
                // target_id is skill_id, value is required die size
                if let (Some(skill_id), Some(required_die)) = (req.target_id, req.value) {
                    ctx.skill_dies
                        .get(&skill_id)
                        .and_then(|&opt_die| opt_die)
                        .map(|die| die >= required_die as u8)
                        .unwrap_or(false)
                } else {
                    true
                }
            }
            "edge" => {
                // target_id is edge_id - character must have this edge
                if let Some(edge_id) = req.target_id {
                    ctx.edge_ids.contains(&edge_id)
                } else {
                    true
                }
            }
            "wild_card" => {
                // Character must be a Wild Card
                ctx.is_wild_card
            }
            "arcane_background" => {
                // Character must have at least one arcane background
                // If target_id is specified, must have that specific one
                if let Some(ab_id) = req.target_id {
                    ctx.arcane_background_ids.contains(&ab_id)
                } else {
                    !ctx.arcane_background_ids.is_empty()
                }
            }
            "arcane_skill" => {
                // target_id is skill_id, value is required die size
                // Check arcane skill specifically
                if let (Some(skill_id), Some(required_die)) = (req.target_id, req.value) {
                    ctx.arcane_skill_dies
                        .get(&skill_id)
                        .and_then(|&opt_die| opt_die)
                        .map(|die| die >= required_die as u8)
                        .unwrap_or(false)
                } else {
                    true
                }
            }
            "description" => {
                // Descriptive requirements (e.g., "GM approval") - cannot be automatically evaluated
                // Return true to not block, but these should be noted in UI
                true
            }
            _ => {
                // Unknown requirement type - be permissive
                true
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_requirement(description: &str) -> Requirement {
        Requirement {
            id: 1,
            requirement_type: "rank".to_string(),
            target_id: Some(1),
            value: Some(1),
            description: description.to_string(),
            created_at: "2024-01-01".to_string(),
            updated_at: "2024-01-01".to_string(),
        }
    }

    #[test]
    fn leaf_creates_single_requirement() {
        let req = create_test_requirement("Novice");
        let tree = RequirementTree::leaf(req);

        assert!(matches!(tree.node, RequirementNode::Leaf(_)));
    }

    #[test]
    fn and_creates_and_node() {
        let req1 = RequirementTree::leaf(create_test_requirement("Novice"));
        let req2 = RequirementTree::leaf(create_test_requirement("Agility d8"));
        let tree = RequirementTree::and(vec![req1, req2]);

        assert!(matches!(tree.node, RequirementNode::And(children) if children.len() == 2));
    }

    #[test]
    fn or_creates_or_node() {
        let req1 = RequirementTree::leaf(create_test_requirement("Fighting d8"));
        let req2 = RequirementTree::leaf(create_test_requirement("Shooting d8"));
        let tree = RequirementTree::or(vec![req1, req2]);

        assert!(matches!(tree.node, RequirementNode::Or(children) if children.len() == 2));
    }

    #[test]
    fn not_creates_not_node() {
        let req = RequirementTree::leaf(create_test_requirement("Novice"));
        let tree = RequirementTree::negate(req);

        assert!(matches!(tree.node, RequirementNode::Not(_)));
    }

    #[test]
    fn none_creates_empty_and() {
        let tree = RequirementTree::none();

        assert!(tree.is_empty());
    }

    #[test]
    fn is_empty_returns_false_for_non_empty() {
        let tree = RequirementTree::leaf(create_test_requirement("Novice"));

        assert!(!tree.is_empty());
    }
}
