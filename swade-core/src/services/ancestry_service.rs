use rusqlite::Connection;

use crate::error::Result;
use crate::repositories::{AncestryChoiceRepository, AncestryRepository};
use crate::services::{ModifierService, RequirementService};
use crate::views::{AncestryChoiceOptionView, AncestryChoiceView, AncestryView};

pub struct AncestryService;

impl AncestryService {
    pub fn get_all(conn: &Connection) -> Result<Vec<AncestryView>> {
        let ancestries = AncestryRepository::get_all(conn)?;

        let mut views = Vec::new();
        for ancestry in ancestries {
            let choices = Self::load_choices(conn, ancestry.id)?;
            let modifiers = ModifierService::get_for_ancestry(conn, ancestry.id)?;
            let requirements = RequirementService::get_for_ancestry(conn, ancestry.id)?;
            views.push(AncestryView::new(
                ancestry,
                choices,
                modifiers,
                requirements,
            ));
        }

        Ok(views)
    }

    pub fn get_by_id(conn: &Connection, id: i64) -> Result<Option<AncestryView>> {
        let ancestry = AncestryRepository::get_by_id(conn, id)?;

        match ancestry {
            Some(ancestry) => {
                let choices = Self::load_choices(conn, ancestry.id)?;
                let modifiers = ModifierService::get_for_ancestry(conn, ancestry.id)?;
                let requirements = RequirementService::get_for_ancestry(conn, ancestry.id)?;
                Ok(Some(AncestryView::new(
                    ancestry,
                    choices,
                    modifiers,
                    requirements,
                )))
            }
            None => Ok(None),
        }
    }

    fn load_choices(conn: &Connection, ancestry_id: i64) -> Result<Vec<AncestryChoiceView>> {
        let choices = AncestryChoiceRepository::get_choices_by_ancestry_id(conn, ancestry_id)?;

        let mut choice_views = Vec::new();
        for choice in choices {
            let options = AncestryChoiceRepository::get_options_by_choice_id(conn, choice.id)?;
            let option_views: Vec<AncestryChoiceOptionView> = options
                .into_iter()
                .map(AncestryChoiceOptionView::new)
                .collect();
            choice_views.push(AncestryChoiceView::new(choice, option_views));
        }

        Ok(choice_views)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::setup_test_db;
    use crate::views::{AncestryChoiceType, AncestryOptionType};
    use rusqlite::params;

    fn insert_test_ancestry(conn: &Connection, id: i64, name: &str, source: &str) {
        conn.execute(
            "INSERT INTO ancestries (id, name, source, description, created_at, updated_at)
             VALUES (?, ?, ?, 'Test description', '2024-01-01', '2024-01-01')",
            params![id, name, source],
        )
        .unwrap();
    }

    fn insert_test_choice(
        conn: &Connection,
        id: i64,
        ancestry_id: i64,
        choice_type: &str,
        description: &str,
    ) {
        conn.execute(
            "INSERT INTO ancestry_choices (id, ancestry_id, choice_type, min_selections,
                                          max_selections, description, created_at, updated_at)
             VALUES (?, ?, ?, 1, 1, ?, '2024-01-01', '2024-01-01')",
            params![id, ancestry_id, choice_type, description],
        )
        .unwrap();
    }

    fn insert_test_option(
        conn: &Connection,
        id: i64,
        choice_id: i64,
        option_type: &str,
        option_id: Option<i64>,
    ) {
        conn.execute(
            "INSERT INTO ancestry_choice_options (id, choice_id, option_type, option_id,
                                                  created_at, updated_at)
             VALUES (?, ?, ?, ?, '2024-01-01', '2024-01-01')",
            params![id, choice_id, option_type, option_id],
        )
        .unwrap();
    }

    fn insert_modifier(conn: &Connection, id: i64, description: &str) {
        conn.execute(
            "INSERT INTO modifiers (id, target_type, target_identifier, value_type,
                                   value, description, created_at, updated_at)
             VALUES (?, 'attribute', 'Strength', 'die_increment', 1, ?, '2024-01-01', '2024-01-01')",
            params![id, description],
        )
        .unwrap();
    }

    fn insert_ancestry_modifier(conn: &Connection, ancestry_id: i64, modifier_id: i64) {
        conn.execute(
            "INSERT INTO ancestry_modifiers (ancestry_id, modifier_id, created_at, updated_at)
             VALUES (?, ?, '2024-01-01', '2024-01-01')",
            params![ancestry_id, modifier_id],
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

    fn insert_ancestry_requirement(conn: &Connection, ancestry_id: i64, expression_id: i64) {
        conn.execute(
            "INSERT INTO ancestry_requirements (ancestry_id, requirement_expression_id, created_at, updated_at)
             VALUES (?, ?, '2024-01-01', '2024-01-01')",
            params![ancestry_id, expression_id],
        )
        .unwrap();
    }

    #[test]
    fn get_all_returns_empty_when_no_ancestries() {
        let conn = setup_test_db();

        let ancestries = AncestryService::get_all(&conn).unwrap();

        assert!(ancestries.is_empty());
    }

    #[test]
    fn get_all_returns_ancestries() {
        let conn = setup_test_db();
        insert_test_ancestry(&conn, 1, "Human", "core");
        insert_test_ancestry(&conn, 2, "Dwarf", "core");

        let ancestries = AncestryService::get_all(&conn).unwrap();

        assert_eq!(ancestries.len(), 2);
    }

    #[test]
    fn get_all_includes_choices() {
        let conn = setup_test_db();
        insert_test_ancestry(&conn, 1, "Human", "core");
        insert_test_choice(&conn, 1, 1, "free_edge", "Choose one free Edge");

        let ancestries = AncestryService::get_all(&conn).unwrap();

        assert_eq!(ancestries.len(), 1);
        assert_eq!(ancestries[0].choices.len(), 1);
        assert_eq!(
            ancestries[0].choices[0].choice_type,
            AncestryChoiceType::FreeEdge
        );
    }

    #[test]
    fn get_all_includes_choice_options() {
        let conn = setup_test_db();
        insert_test_ancestry(&conn, 1, "Human", "core");
        insert_test_choice(&conn, 1, 1, "free_edge", "Choose one free Edge");
        insert_test_option(&conn, 1, 1, "edge", Some(10));
        insert_test_option(&conn, 2, 1, "edge", Some(20));

        let ancestries = AncestryService::get_all(&conn).unwrap();

        assert_eq!(ancestries[0].choices[0].options.len(), 2);
        assert_eq!(
            ancestries[0].choices[0].options[0].option_type,
            AncestryOptionType::Edge
        );
    }

    #[test]
    fn get_all_includes_modifiers() {
        let conn = setup_test_db();
        insert_test_ancestry(&conn, 1, "Dwarf", "core");
        insert_modifier(&conn, 1, "+1 Vigor");
        insert_ancestry_modifier(&conn, 1, 1);

        let ancestries = AncestryService::get_all(&conn).unwrap();

        assert_eq!(ancestries.len(), 1);
        assert_eq!(ancestries[0].modifiers.len(), 1);
        assert_eq!(ancestries[0].modifiers[0].description, "+1 Vigor");
    }

    #[test]
    fn get_all_includes_requirements() {
        let conn = setup_test_db();
        insert_test_ancestry(&conn, 1, "Half-Elf", "core");
        insert_requirement(&conn, 1, "rank", "Seasoned");
        insert_requirement_expression(&conn, 1, None, "requirement", Some(1), 0);
        insert_ancestry_requirement(&conn, 1, 1);

        let ancestries = AncestryService::get_all(&conn).unwrap();

        assert_eq!(ancestries.len(), 1);
        assert!(!ancestries[0].requirements.is_empty());
    }

    #[test]
    fn get_by_id_returns_none_when_not_found() {
        let conn = setup_test_db();

        let ancestry = AncestryService::get_by_id(&conn, 999).unwrap();

        assert!(ancestry.is_none());
    }

    #[test]
    fn get_by_id_returns_ancestry_with_all_data() {
        let conn = setup_test_db();
        insert_test_ancestry(&conn, 1, "Elf", "core");
        insert_test_choice(&conn, 1, 1, "free_edge", "Choose one free Edge");
        insert_test_option(&conn, 1, 1, "edge", None);
        insert_modifier(&conn, 1, "+1 Agility");
        insert_ancestry_modifier(&conn, 1, 1);
        insert_requirement(&conn, 1, "rank", "Novice");
        insert_requirement_expression(&conn, 1, None, "requirement", Some(1), 0);
        insert_ancestry_requirement(&conn, 1, 1);

        let ancestry = AncestryService::get_by_id(&conn, 1).unwrap();

        assert!(ancestry.is_some());
        let ancestry = ancestry.unwrap();
        assert_eq!(ancestry.name, "Elf");
        assert_eq!(ancestry.choices.len(), 1);
        assert_eq!(ancestry.choices[0].options.len(), 1);
        assert_eq!(ancestry.modifiers.len(), 1);
        assert!(!ancestry.requirements.is_empty());
    }

    #[test]
    fn get_by_id_maps_fields_correctly() {
        let conn = setup_test_db();
        insert_test_ancestry(&conn, 1, "Human", "core");

        let ancestry = AncestryService::get_by_id(&conn, 1).unwrap().unwrap();

        assert_eq!(ancestry.id, 1);
        assert_eq!(ancestry.name, "Human");
        assert_eq!(ancestry.source, "core");
        assert_eq!(ancestry.description, "Test description");
    }
}
