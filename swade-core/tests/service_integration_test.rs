//! Integration tests for services using the actual seeded database.

use rusqlite::Connection;
use swade_core::services::{AttributeService, EdgeService, SkillService};
use swade_core::views::Die;

fn open_seeded_db() -> Connection {
    // Tests run from package directory (swade-core/), so path is relative to there
    Connection::open("../db/swade.db").expect("Failed to open seeded database")
}

#[test]
fn attribute_service_loads_all_five_attributes() {
    let conn = open_seeded_db();

    let attributes = AttributeService::get_all(&conn).unwrap();

    assert_eq!(attributes.len(), 5);

    let names: Vec<&str> = attributes.iter().map(|a| a.name.as_str()).collect();
    assert!(names.contains(&"Agility"));
    assert!(names.contains(&"Smarts"));
    assert!(names.contains(&"Spirit"));
    assert!(names.contains(&"Strength"));
    assert!(names.contains(&"Vigor"));
}

#[test]
fn attribute_service_all_attributes_have_d4_base() {
    let conn = open_seeded_db();

    let attributes = AttributeService::get_all(&conn).unwrap();

    for attr in attributes {
        assert_eq!(
            attr.base_die,
            Die::d4(),
            "Attribute {} should have d4 base",
            attr.name
        );
    }
}

#[test]
fn attribute_service_get_by_id_returns_agility() {
    let conn = open_seeded_db();

    let agility = AttributeService::get_by_id(&conn, 1).unwrap();

    assert!(agility.is_some());
    let agility = agility.unwrap();
    assert_eq!(agility.name, "Agility");
}

#[test]
fn skill_service_loads_skills() {
    let conn = open_seeded_db();

    let skills = SkillService::get_all(&conn).unwrap();

    assert!(skills.len() > 0, "Should have at least some skills");
    println!("Loaded {} skills", skills.len());
}

#[test]
fn skill_service_has_core_skills() {
    let conn = open_seeded_db();

    let skills = SkillService::get_all(&conn).unwrap();
    let core_skills: Vec<_> = skills.iter().filter(|s| s.is_core_skill).collect();

    assert!(core_skills.len() > 0, "Should have core skills");

    for skill in &core_skills {
        assert_eq!(
            skill.default_die,
            Some(Die::d4()),
            "Core skill {} should have d4 default",
            skill.name
        );
    }

    println!(
        "Core skills: {:?}",
        core_skills.iter().map(|s| &s.name).collect::<Vec<_>>()
    );
}

#[test]
fn skill_service_non_core_skills_have_no_default() {
    let conn = open_seeded_db();

    let skills = SkillService::get_all(&conn).unwrap();
    let non_core_skills: Vec<_> = skills.iter().filter(|s| !s.is_core_skill).collect();

    for skill in &non_core_skills {
        assert_eq!(
            skill.default_die, None,
            "Non-core skill {} should have no default die",
            skill.name
        );
    }
}

#[test]
fn skill_service_skills_have_valid_linked_attributes() {
    let conn = open_seeded_db();

    let skills = SkillService::get_all(&conn).unwrap();
    let attributes = AttributeService::get_all(&conn).unwrap();
    let attr_ids: Vec<i64> = attributes.iter().map(|a| a.id).collect();

    for skill in &skills {
        assert!(
            attr_ids.contains(&skill.linked_attribute_id),
            "Skill {} has invalid linked_attribute_id {}",
            skill.name,
            skill.linked_attribute_id
        );
    }
}

#[test]
fn edge_service_loads_edges() {
    let conn = open_seeded_db();

    let edges = EdgeService::get_all(&conn).unwrap();

    assert!(edges.len() > 0, "Should have at least some edges");
    println!("Loaded {} edges", edges.len());
}

#[test]
fn edge_service_edges_have_categories() {
    let conn = open_seeded_db();

    let edges = EdgeService::get_all(&conn).unwrap();

    // Collect unique categories
    let categories: std::collections::HashSet<String> =
        edges.iter().map(|e| e.category.to_string()).collect();

    println!("Edge categories: {:?}", categories);
    assert!(categories.len() > 0, "Should have at least one category");
}

#[test]
fn edge_service_get_by_id_returns_edge() {
    let conn = open_seeded_db();

    // First get all edges to find a valid ID
    let edges = EdgeService::get_all(&conn).unwrap();
    assert!(
        !edges.is_empty(),
        "Need at least one edge to test get_by_id"
    );

    let first_edge = &edges[0];
    let retrieved = EdgeService::get_by_id(&conn, first_edge.id).unwrap();

    assert!(retrieved.is_some());
    let retrieved = retrieved.unwrap();
    assert_eq!(retrieved.id, first_edge.id);
    assert_eq!(retrieved.name, first_edge.name);
}

#[test]
fn edge_service_get_by_id_returns_none_for_invalid_id() {
    let conn = open_seeded_db();

    let edge = EdgeService::get_by_id(&conn, 999999).unwrap();

    assert!(edge.is_none());
}

#[test]
fn edge_service_edges_have_valid_sources() {
    let conn = open_seeded_db();

    let edges = EdgeService::get_all(&conn).unwrap();

    for edge in &edges {
        assert!(
            !edge.source.is_empty(),
            "Edge {} should have a source",
            edge.name
        );
    }
}
