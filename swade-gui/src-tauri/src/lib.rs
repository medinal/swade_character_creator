//! SWADE Character Creator - Tauri backend.

mod commands;
mod error;
mod state;

use std::sync::Mutex;
use swade_core::db::Database;
use tauri_specta::{collect_commands, Builder};

use commands::{
    // Character
    get_characters,
    get_character,
    create_character,
    delete_character,
    get_draft_character,
    save_character,
    discard_draft,
    load_character_into_draft,
    update_draft_basic_info,
    update_character_status,
    update_character_portrait,
    clear_character_portrait,
    // Ancestry
    get_ancestries,
    update_draft_ancestry,
    update_draft_ancestry_choice,
    // Hindrances
    get_hindrances,
    add_draft_hindrance,
    remove_draft_hindrance,
    // Edges
    get_edges,
    add_draft_edge,
    remove_draft_edge,
    allocate_hindrance_points_to_edges,
    // Powers
    get_arcane_backgrounds,
    add_draft_arcane_background,
    remove_draft_arcane_background,
    update_draft_arcane_background_choice,
    get_powers,
    add_draft_power,
    remove_draft_power,
    // Skills
    get_skills,
    get_game_config,
    update_draft_skill,
    allocate_hindrance_points_to_skills,
    check_skill_decrement_impact,
    // Attributes
    update_draft_attribute,
    allocate_hindrance_points_to_attributes,
    check_attribute_decrement_impact,
    // Notes
    get_character_notes,
    create_character_note,
    update_character_note,
    delete_character_note,
    // Gear
    get_all_gear,
    get_gear_categories,
    get_gear_by_category,
    get_character_gear,
    add_gear,
    purchase_gear,
    sell_gear,
    remove_gear,
    toggle_gear_equipped,
    update_gear_notes,
    update_character_wealth,
    // Advancement
    get_advancement_options,
    take_edge_advance,
    take_attribute_advance,
    take_expensive_skill_advance,
    take_cheap_skill_advance,
    take_hindrance_advance,
    undo_last_advance,
    get_advancement_history,
};
use state::AppState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let builder = Builder::<tauri::Wry>::new()
        .commands(collect_commands![
            // Character
            get_characters,
            get_character,
            create_character,
            delete_character,
            get_draft_character,
            save_character,
            discard_draft,
            load_character_into_draft,
            update_draft_basic_info,
            update_character_status,
            update_character_portrait,
            clear_character_portrait,
            // Ancestry
            get_ancestries,
            update_draft_ancestry,
            update_draft_ancestry_choice,
            // Hindrances
            get_hindrances,
            add_draft_hindrance,
            remove_draft_hindrance,
            // Edges
            get_edges,
            add_draft_edge,
            remove_draft_edge,
            allocate_hindrance_points_to_edges,
            // Powers
            get_arcane_backgrounds,
            add_draft_arcane_background,
            remove_draft_arcane_background,
            update_draft_arcane_background_choice,
            get_powers,
            add_draft_power,
            remove_draft_power,
            // Skills
            get_skills,
            get_game_config,
            update_draft_skill,
            allocate_hindrance_points_to_skills,
            check_skill_decrement_impact,
            // Attributes
            update_draft_attribute,
            allocate_hindrance_points_to_attributes,
            check_attribute_decrement_impact,
            // Notes
            get_character_notes,
            create_character_note,
            update_character_note,
            delete_character_note,
            // Gear
            get_all_gear,
            get_gear_categories,
            get_gear_by_category,
            get_character_gear,
            add_gear,
            purchase_gear,
            sell_gear,
            remove_gear,
            toggle_gear_equipped,
            update_gear_notes,
    update_character_wealth,
            // Advancement
            get_advancement_options,
            take_edge_advance,
            take_attribute_advance,
            take_expensive_skill_advance,
            take_cheap_skill_advance,
            take_hindrance_advance,
            undo_last_advance,
            get_advancement_history
        ]);

    #[cfg(debug_assertions)]
    builder
        .export(
            specta_typescript::Typescript::default()
                .bigint(specta_typescript::BigIntExportBehavior::Number),
            "../src/lib/bindings.ts",
        )
        .expect("Failed to export TypeScript bindings");

    let db = Database::init().expect("Failed to initialize database");
    let app_state = Mutex::new(AppState {
        db,
        draft_character: None,
    });

    tauri::Builder::default()
        .invoke_handler(builder.invoke_handler())
        .manage(app_state)
        .setup(move |app| {
            builder.mount_events(app);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
