use rusqlite::Connection;

use crate::error::Result;
use crate::repositories::{ArcaneBackgroundChoiceRepository, ArcaneBackgroundRepository, SkillRepository};
use crate::services::RequirementService;
use crate::views::{ArcaneBackgroundChoiceOptionView, ArcaneBackgroundChoiceView, ArcaneBackgroundView};

pub struct ArcaneBackgroundService;

impl ArcaneBackgroundService {
    pub fn get_all(conn: &Connection) -> Result<Vec<ArcaneBackgroundView>> {
        let arcane_backgrounds = ArcaneBackgroundRepository::get_all(conn)?;

        let mut views = Vec::new();
        for ab in arcane_backgrounds {
            let skill_name = SkillRepository::get_by_id(conn, ab.arcane_skill_id)?.map(|s| s.name);
            let requirements = RequirementService::get_for_arcane_background(conn, ab.id)?;
            let choices = Self::load_choices(conn, ab.id)?;
            views.push(ArcaneBackgroundView::new(ab, skill_name, requirements, choices));
        }

        Ok(views)
    }

    pub fn get_by_id(conn: &Connection, id: i64) -> Result<Option<ArcaneBackgroundView>> {
        let arcane_background = ArcaneBackgroundRepository::get_by_id(conn, id)?;

        match arcane_background {
            Some(ab) => {
                let skill_name =
                    SkillRepository::get_by_id(conn, ab.arcane_skill_id)?.map(|s| s.name);
                let requirements = RequirementService::get_for_arcane_background(conn, ab.id)?;
                let choices = Self::load_choices(conn, ab.id)?;
                Ok(Some(ArcaneBackgroundView::new(
                    ab,
                    skill_name,
                    requirements,
                    choices,
                )))
            }
            None => Ok(None),
        }
    }

    fn load_choices(conn: &Connection, arcane_background_id: i64) -> Result<Vec<ArcaneBackgroundChoiceView>> {
        let choices = ArcaneBackgroundChoiceRepository::get_choices_by_arcane_background_id(conn, arcane_background_id)?;

        let mut choice_views = Vec::new();
        for choice in choices {
            let options = ArcaneBackgroundChoiceRepository::get_options_by_choice_id(conn, choice.id)?;
            let option_views: Vec<ArcaneBackgroundChoiceOptionView> = options
                .into_iter()
                .map(ArcaneBackgroundChoiceOptionView::new)
                .collect();
            choice_views.push(ArcaneBackgroundChoiceView::new(choice, option_views));
        }

        Ok(choice_views)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::setup_test_db;
    use rusqlite::params;

    fn insert_test_attribute(conn: &Connection, id: i64, name: &str) {
        conn.execute(
            "INSERT INTO attributes (id, name, description, base_value, created_at, updated_at)
             VALUES (?, ?, 'Test', 4, '2024-01-01', '2024-01-01')",
            params![id, name],
        )
        .unwrap();
    }

    fn insert_test_skill(conn: &Connection, id: i64, name: &str, attribute_id: i64) {
        conn.execute(
            "INSERT INTO skills (id, name, description, linked_attribute_id, is_core_skill,
                                 max_die_size, max_die_modifier, source, created_at, updated_at)
             VALUES (?, ?, 'Test skill', ?, 0, 12, 0, 'core', '2024-01-01', '2024-01-01')",
            params![id, name, attribute_id],
        )
        .unwrap();
    }

    fn insert_test_arcane_background(
        conn: &Connection,
        id: i64,
        name: &str,
        skill_id: i64,
        starting_powers: i64,
        power_points: i64,
    ) {
        conn.execute(
            "INSERT INTO arcane_backgrounds (id, name, arcane_skill_id, starting_powers,
                                             starting_power_points, source, description,
                                             created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, 'core', 'Test description', '2024-01-01', '2024-01-01')",
            params![id, name, skill_id, starting_powers, power_points],
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

    fn insert_arcane_background_requirement(conn: &Connection, ab_id: i64, expression_id: i64) {
        conn.execute(
            "INSERT INTO arcane_background_requirements (arcane_background_id, requirement_expression_id,
                                                         created_at, updated_at)
             VALUES (?, ?, '2024-01-01', '2024-01-01')",
            params![ab_id, expression_id],
        )
        .unwrap();
    }

    fn insert_arcane_background_choice(
        conn: &Connection,
        id: i64,
        ab_id: i64,
        choice_type: &str,
        description: &str,
    ) {
        conn.execute(
            "INSERT INTO arcane_background_choices (id, arcane_background_id, choice_type,
                                                    min_selections, max_selections, description,
                                                    created_at, updated_at)
             VALUES (?, ?, ?, 0, 55, ?, '2024-01-01', '2024-01-01')",
            params![id, ab_id, choice_type, description],
        )
        .unwrap();
    }

    fn insert_arcane_background_choice_option(
        conn: &Connection,
        id: i64,
        choice_id: i64,
        option_type: &str,
        option_id: Option<i64>,
        option_description: Option<&str>,
    ) {
        conn.execute(
            "INSERT INTO arcane_background_choice_options (id, choice_id, option_type, option_id,
                                                           option_description, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, '2024-01-01', '2024-01-01')",
            params![id, choice_id, option_type, option_id, option_description],
        )
        .unwrap();
    }

    #[test]
    fn get_all_returns_empty_when_no_arcane_backgrounds() {
        let conn = setup_test_db();

        let abs = ArcaneBackgroundService::get_all(&conn).unwrap();

        assert!(abs.is_empty());
    }

    #[test]
    fn get_all_returns_arcane_backgrounds() {
        let conn = setup_test_db();
        insert_test_attribute(&conn, 1, "Smarts");
        insert_test_skill(&conn, 1, "Spellcasting", 1);
        insert_test_skill(&conn, 2, "Faith", 1);
        insert_test_arcane_background(&conn, 1, "Magic", 1, 3, 10);
        insert_test_arcane_background(&conn, 2, "Miracles", 2, 2, 10);

        let abs = ArcaneBackgroundService::get_all(&conn).unwrap();

        assert_eq!(abs.len(), 2);
    }

    #[test]
    fn get_all_includes_skill_name() {
        let conn = setup_test_db();
        insert_test_attribute(&conn, 1, "Smarts");
        insert_test_skill(&conn, 1, "Spellcasting", 1);
        insert_test_arcane_background(&conn, 1, "Magic", 1, 3, 10);

        let abs = ArcaneBackgroundService::get_all(&conn).unwrap();

        assert_eq!(abs.len(), 1);
        assert_eq!(abs[0].arcane_skill_name, Some("Spellcasting".to_string()));
    }

    #[test]
    fn get_all_includes_requirements() {
        let conn = setup_test_db();
        insert_test_attribute(&conn, 1, "Smarts");
        insert_test_skill(&conn, 1, "Spellcasting", 1);
        insert_test_arcane_background(&conn, 1, "Magic", 1, 3, 10);
        insert_requirement(&conn, 1, "rank", "Novice");
        insert_requirement_expression(&conn, 1, None, "requirement", Some(1), 0);
        insert_arcane_background_requirement(&conn, 1, 1);

        let abs = ArcaneBackgroundService::get_all(&conn).unwrap();

        assert_eq!(abs.len(), 1);
        assert!(!abs[0].requirements.is_empty());
    }

    #[test]
    fn get_by_id_returns_none_when_not_found() {
        let conn = setup_test_db();

        let ab = ArcaneBackgroundService::get_by_id(&conn, 999).unwrap();

        assert!(ab.is_none());
    }

    #[test]
    fn get_by_id_returns_arcane_background_with_all_data() {
        let conn = setup_test_db();
        insert_test_attribute(&conn, 1, "Smarts");
        insert_test_skill(&conn, 1, "Spellcasting", 1);
        insert_test_arcane_background(&conn, 1, "Magic", 1, 3, 10);
        insert_requirement(&conn, 1, "rank", "Novice");
        insert_requirement_expression(&conn, 1, None, "requirement", Some(1), 0);
        insert_arcane_background_requirement(&conn, 1, 1);

        let ab = ArcaneBackgroundService::get_by_id(&conn, 1).unwrap();

        assert!(ab.is_some());
        let ab = ab.unwrap();
        assert_eq!(ab.name, "Magic");
        assert_eq!(ab.arcane_skill_name, Some("Spellcasting".to_string()));
        assert_eq!(ab.starting_powers, 3);
        assert_eq!(ab.starting_power_points, 10);
        assert!(!ab.requirements.is_empty());
    }

    #[test]
    fn get_by_id_maps_fields_correctly() {
        let conn = setup_test_db();
        insert_test_attribute(&conn, 1, "Spirit");
        insert_test_skill(&conn, 1, "Faith", 1);
        insert_test_arcane_background(&conn, 1, "Miracles", 1, 2, 10);

        let ab = ArcaneBackgroundService::get_by_id(&conn, 1)
            .unwrap()
            .unwrap();

        assert_eq!(ab.id, 1);
        assert_eq!(ab.name, "Miracles");
        assert_eq!(ab.arcane_skill_id, 1);
        assert_eq!(ab.arcane_skill_name, Some("Faith".to_string()));
        assert_eq!(ab.starting_powers, 2);
        assert_eq!(ab.starting_power_points, 10);
        assert_eq!(ab.source, "core");
        assert_eq!(ab.description, "Test description");
    }

    #[test]
    fn get_all_includes_choices() {
        let conn = setup_test_db();
        insert_test_attribute(&conn, 1, "Smarts");
        insert_test_skill(&conn, 1, "Spellcasting", 1);
        insert_test_arcane_background(&conn, 1, "Wizard", 1, 6, 15);
        insert_arcane_background_choice(&conn, 1, 1, "available_power", "Powers available to the Wizard");

        let abs = ArcaneBackgroundService::get_all(&conn).unwrap();

        assert_eq!(abs.len(), 1);
        assert_eq!(abs[0].choices.len(), 1);
        assert_eq!(abs[0].choices[0].description, "Powers available to the Wizard");
    }

    #[test]
    fn get_all_includes_choice_options() {
        let conn = setup_test_db();
        insert_test_attribute(&conn, 1, "Smarts");
        insert_test_skill(&conn, 1, "Spellcasting", 1);
        insert_test_arcane_background(&conn, 1, "Wizard", 1, 6, 15);
        insert_arcane_background_choice(&conn, 1, 1, "available_power", "Powers available");
        insert_arcane_background_choice_option(&conn, 1, 1, "power", Some(1), Some("Bolt"));
        insert_arcane_background_choice_option(&conn, 2, 1, "power", Some(2), Some("Blast"));

        let abs = ArcaneBackgroundService::get_all(&conn).unwrap();

        assert_eq!(abs[0].choices[0].options.len(), 2);
        assert_eq!(abs[0].choices[0].options[0].description, Some("Bolt".to_string()));
    }

    #[test]
    fn get_by_id_includes_choices() {
        let conn = setup_test_db();
        insert_test_attribute(&conn, 1, "Smarts");
        insert_test_skill(&conn, 1, "Spellcasting", 1);
        insert_test_arcane_background(&conn, 1, "Wizard", 1, 6, 15);
        insert_arcane_background_choice(&conn, 1, 1, "available_power", "Available powers");
        insert_arcane_background_choice(&conn, 2, 1, "built_in_hindrance", "Built-in hindrances");
        insert_arcane_background_choice_option(&conn, 1, 1, "power", Some(1), Some("Bolt"));
        insert_arcane_background_choice_option(&conn, 2, 2, "hindrance", Some(10), Some("Armor Interference"));

        let ab = ArcaneBackgroundService::get_by_id(&conn, 1).unwrap().unwrap();

        assert_eq!(ab.choices.len(), 2);
        assert_eq!(ab.choices[0].options.len(), 1);
        assert_eq!(ab.choices[1].options.len(), 1);
    }

    #[test]
    fn get_all_empty_choices_when_none_defined() {
        let conn = setup_test_db();
        insert_test_attribute(&conn, 1, "Smarts");
        insert_test_skill(&conn, 1, "Spellcasting", 1);
        insert_test_arcane_background(&conn, 1, "Magic", 1, 3, 10);

        let abs = ArcaneBackgroundService::get_all(&conn).unwrap();

        assert_eq!(abs[0].choices.len(), 0);
    }
}
