//! Export character to various formats.

use std::sync::Mutex;
use swade_core::services::CharacterService;
use swade_core::views::CharacterView;
use tauri::State;
use tauri_plugin_dialog::{DialogExt, FilePath};

use crate::error::{CommandError, CommandResult};
use crate::state::{lock_state, AppState};

/// Export a character as a markdown file.
/// Shows a save dialog and writes the file to the chosen location.
/// Returns true if the file was saved, false if the user cancelled.
#[tauri::command]
#[specta::specta]
pub async fn export_character_markdown(
    id: i64,
    app: tauri::AppHandle,
    state: State<'_, Mutex<AppState>>,
) -> CommandResult<bool> {
    // Get the character data
    let (markdown, default_filename) = {
        let state = lock_state(&state)?;
        let conn = state.connection()?;

        let character = CharacterService::get_by_id(&conn, id)?
            .ok_or_else(|| CommandError::NotFound("Character not found".to_string()))?;

        let markdown = generate_markdown(&character);
        // Sanitize filename: replace invalid characters
        let filename = character.name.replace(['<', '>', ':', '"', '/', '\\', '|', '?', '*'], "_") + ".md";

        (markdown, filename)
    };

    // Show save dialog
    let file_path = app
        .dialog()
        .file()
        .set_file_name(&default_filename)
        .add_filter("Markdown", &["md"])
        .blocking_save_file();

    match file_path {
        Some(path) => {
            // Write the file
            let path_str = match &path {
                FilePath::Path(p) => p.to_string_lossy().to_string(),
                FilePath::Url(u) => u.path().to_string(),
            };

            std::fs::write(&path_str, &markdown)
                .map_err(|e| CommandError::State(format!("Failed to write file: {}", e)))?;

            Ok(true)
        }
        None => {
            // User cancelled
            Ok(false)
        }
    }
}

/// Generate a markdown string from a character view.
fn generate_markdown(character: &CharacterView) -> String {
    let mut md = String::new();

    // Header
    md.push_str(&format!("# {}\n\n", character.name));
    md.push_str(&format!(
        "**Rank:** {} | **Advances:** {} | **Wild Card:** {}\n\n",
        character.rank.name,
        character.current_advances,
        if character.is_wild_card { "Yes" } else { "No" }
    ));
    md.push_str("---\n\n");

    // Basic Information
    md.push_str("## Basic Information\n\n");

    if let Some(ref ancestry) = character.ancestry {
        md.push_str(&format!("**Ancestry:** {}\n", ancestry.name));
        md.push_str(&format!("> {}\n\n", ancestry.description));
    }

    if let Some(ref background) = character.background {
        md.push_str(&format!("**Background:** {}\n\n", background));
    }

    if let Some(ref description) = character.description {
        md.push_str(&format!("**Description:** {}\n\n", description));
    }

    md.push_str("---\n\n");

    // Attributes
    md.push_str("## Attributes\n\n");
    md.push_str("| Attribute | Die |\n");
    md.push_str("|-----------|-----|\n");
    for attr in &character.attributes {
        let die_str = format!("{}", attr.effective_die);
        md.push_str(&format!("| {} | {} |\n", attr.attribute.name, die_str));
    }
    md.push_str("\n---\n\n");

    // Derived Stats
    md.push_str("## Derived Stats\n\n");
    md.push_str("| Stat | Value |\n");
    md.push_str("|------|-------|\n");
    md.push_str(&format!("| Pace | {} |\n", character.derived_stats.pace));
    md.push_str(&format!("| Parry | {} |\n", character.derived_stats.parry));
    md.push_str(&format!(
        "| Toughness | {} |\n",
        character.derived_stats.toughness
    ));
    md.push_str(&format!("| Size | {} |\n", character.derived_stats.size));
    md.push_str("\n---\n\n");

    // Skills - only trained skills and untrained core skills
    md.push_str("## Skills\n\n");
    md.push_str("| Skill | Die | Attribute |\n");
    md.push_str("|-------|-----|----------|\n");

    // Build a map of attribute IDs to names
    let attr_map: std::collections::HashMap<i64, &str> = character
        .attributes
        .iter()
        .map(|a| (a.attribute.id, a.attribute.name.as_str()))
        .collect();

    // Filter to trained skills OR untrained core skills, then sort alphabetically
    let mut skills_to_show: Vec<_> = character
        .skills
        .iter()
        .filter(|s| s.die.is_some() || s.skill.is_core_skill)
        .collect();
    skills_to_show.sort_by(|a, b| a.skill.name.cmp(&b.skill.name));

    for skill in skills_to_show {
        let die_str = match &skill.effective_die {
            Some(d) => format!("{}", d),
            None => "d4-2".to_string(), // Untrained core skill
        };
        let attr_name = attr_map
            .get(&skill.skill.linked_attribute_id)
            .unwrap_or(&"?");
        md.push_str(&format!(
            "| {} | {} | {} |\n",
            skill.skill.name, die_str, attr_name
        ));
    }
    md.push_str("\n*Only trained skills and untrained core skills shown*\n\n");
    md.push_str("---\n\n");

    // Hindrances
    md.push_str("## Hindrances\n\n");
    if character.hindrances.is_empty() {
        md.push_str("*None*\n\n");
    } else {
        for h in &character.hindrances {
            // Capitalize severity for display
            let severity = match h.hindrance.severity {
                swade_core::views::Severity::Major => "Major",
                swade_core::views::Severity::Minor => "Minor",
            };
            md.push_str(&format!(
                "- **{}** ({}): {}\n",
                h.hindrance.name, severity, h.hindrance.description
            ));
        }
        md.push_str("\n");
    }
    md.push_str("---\n\n");

    // Edges
    md.push_str("## Edges\n\n");
    if character.edges.is_empty() {
        md.push_str("*None*\n\n");
    } else {
        for e in &character.edges {
            md.push_str(&format!(
                "- **{}** ({}): {}\n",
                e.edge.name, e.edge.category, e.edge.description
            ));
        }
        md.push_str("\n");
    }
    md.push_str("---\n\n");

    // Powers (only if character has arcane backgrounds)
    if !character.arcane_backgrounds.is_empty() {
        md.push_str("## Powers\n\n");

        for ab in &character.arcane_backgrounds {
            md.push_str(&format!(
                "**Arcane Background:** {}\n",
                ab.arcane_background.name
            ));
            if let Some(ref skill_name) = ab.arcane_background.arcane_skill_name {
                md.push_str(&format!("**Arcane Skill:** {}\n", skill_name));
            }
        }

        md.push_str(&format!(
            "**Power Points:** {} / {}\n\n",
            character.power_points - character.power_points_used,
            character.power_points
        ));

        if !character.powers.is_empty() {
            md.push_str("| Power | PP | Range | Duration |\n");
            md.push_str("|-------|-----|-------|----------|\n");
            for p in &character.powers {
                md.push_str(&format!(
                    "| {} | {} | {} | {} |\n",
                    p.power.name, p.power.power_points, p.power.range, p.power.duration
                ));
            }
            md.push_str("\n");
        }

        md.push_str("---\n\n");
    }

    // Gear
    md.push_str("## Gear\n\n");
    md.push_str(&format!("**Wealth:** ${}\n\n", character.wealth));

    // Separate gear into categories
    let weapons: Vec<_> = character
        .gear
        .iter()
        .filter(|g| g.gear.is_weapon())
        .collect();
    let armor: Vec<_> = character
        .gear
        .iter()
        .filter(|g| g.gear.is_armor() || g.gear.is_shield())
        .collect();
    let other: Vec<_> = character
        .gear
        .iter()
        .filter(|g| !g.gear.is_weapon() && !g.gear.is_armor() && !g.gear.is_shield())
        .collect();

    // Weapons
    if !weapons.is_empty() {
        md.push_str("### Weapons\n\n");
        md.push_str("| Item | Damage | Range | Notes |\n");
        md.push_str("|------|--------|-------|-------|\n");
        for w in weapons {
            if let Some(ref stats) = w.gear.weapon_stats {
                let range = stats.range_string().unwrap_or_else(|| "Melee".to_string());
                let mut notes_parts = Vec::new();
                if stats.ap > 0 {
                    notes_parts.push(format!("AP {}", stats.ap));
                }
                if stats.is_two_handed {
                    notes_parts.push("2H".to_string());
                }
                if let Some(ref n) = stats.notes {
                    notes_parts.push(n.clone());
                }
                let notes = if notes_parts.is_empty() {
                    "-".to_string()
                } else {
                    notes_parts.join(", ")
                };

                let name = if w.quantity > 1 {
                    format!("{} x{}", w.gear.name, w.quantity)
                } else {
                    w.gear.name.clone()
                };

                md.push_str(&format!("| {} | {} | {} | {} |\n", name, stats.damage, range, notes));
            }
        }
        md.push_str("\n");
    }

    // Armor
    if !armor.is_empty() {
        md.push_str("### Armor\n\n");
        md.push_str("| Item | Armor | Coverage | Notes |\n");
        md.push_str("|------|-------|----------|-------|\n");
        for a in armor {
            if let Some(ref stats) = a.gear.armor_stats {
                let mut notes_parts = Vec::new();
                if stats.is_heavy {
                    notes_parts.push("Heavy".to_string());
                }
                let notes = if notes_parts.is_empty() {
                    "-".to_string()
                } else {
                    notes_parts.join(", ")
                };
                md.push_str(&format!(
                    "| {} | +{} | {} | {} |\n",
                    a.gear.name, stats.armor_value, stats.coverage, notes
                ));
            } else if let Some(ref stats) = a.gear.shield_stats {
                md.push_str(&format!(
                    "| {} | +{} Parry | Shield | - |\n",
                    a.gear.name, stats.parry_bonus
                ));
            }
        }
        md.push_str("\n");
    }

    // Other Gear
    if !other.is_empty() {
        md.push_str("### Other Gear\n\n");
        for g in other {
            let qty_str = if g.quantity > 1 {
                format!(" x{}", g.quantity)
            } else {
                String::new()
            };
            md.push_str(&format!(
                "- {}{} - {:.1} lbs\n",
                g.gear.name, qty_str, g.total_weight
            ));
        }
        md.push_str("\n");
    }

    if character.gear.is_empty() {
        md.push_str("*No gear*\n\n");
    }

    md
}
