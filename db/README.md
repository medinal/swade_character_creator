# SWADE Database

This directory contains the SQLite database and tooling for the Savage Worlds Adventure Edition (SWADE) character management system.

## Overview

The database separates **seed data** (immutable game rules from the SWADE rulebook) from **character data** (player-created characters). Seed data tables are protected by triggers to prevent accidental modification.

## Files

| File | Description |
|------|-------------|
| `swade.db` | The SQLite database |
| `schema.sql` | Database schema definition |
| `seed_gear.sql` | Seed data for gear system (weapons, armor, equipment) |
| `table_dumps/` | Versioned SQL dumps of table data |

## Shell Scripts

All scripts operate in the `./db/` directory regardless of where they're invoked from.

| Script | Description |
|--------|-------------|
| `generate_table_dumps.sh` | Creates a new versioned dump of all tables |
| `reset_and_load.sh` | Rebuilds the database from schema and loads seed data |
| `remove_readonly_triggers.sh` | Removes protection triggers (for editing seed data) |

### Usage Examples

```bash
# Generate a new dump (creates v001, v002, etc.)
./db/generate_table_dumps.sh

# Reset database with latest dump
./db/reset_and_load.sh

# Reset with a specific version
./db/reset_and_load.sh v001

# Reset without triggers (for development)
./db/reset_and_load.sh --no-triggers
```

## Schema Diagram

```mermaid
erDiagram
    %% Core Game Data (Seed Tables)
    attributes {
        int id PK
        string name UK
        string description
        int base_value
    }

    skills {
        int id PK
        string name UK
        string description
        int linked_attribute_id FK
        bool is_core_skill
        int default_die_size
        int max_die_size
        int max_die_modifier
        string source
    }

    ranks {
        int id PK
        string name UK
        int min_advances
        int max_advances
        string description
    }

    edges {
        int id PK
        string name
        string background
        string source
        string description
        bool can_take_multiple_times
    }

    hindrances {
        int id PK
        string name
        string severity
        int point_value
        int companion_hindrance_id FK
        string source
        string description
    }

    ancestries {
        int id PK
        string name
        string source
        string description
    }

    arcane_backgrounds {
        int id PK
        string name UK
        int arcane_skill_id FK
        int starting_powers
        int starting_power_points
        string source
        string description
    }

    powers {
        int id PK
        string name
        int power_points
        string range
        string duration
        string source
        string description
    }

    %% Modifier System
    modifiers {
        int id PK
        string target_type
        string target_identifier
        string value_type
        int value
        string description
    }

    %% Requirements System
    requirements {
        int id PK
        string requirement_type
        int target_id
        int value
        string description
    }

    requirement_expressions {
        int id PK
        int parent_id FK
        string node_type
        int requirement_id FK
        int position
    }

    %% Ancestry Choices
    ancestry_choices {
        int id PK
        int ancestry_id FK
        string choice_type
        string choice_category
        int min_selections
        int max_selections
        string description
    }

    ancestry_choice_options {
        int id PK
        int choice_id FK
        string option_type
        int option_id
        string option_description
    }

    %% Character Data (Writable Tables)
    characters {
        int id PK
        bool is_wild_card
        string name
        int ancestry_id FK
        int attribute_points_spent
        int attribute_points_earned
        int skill_points_spent
        int skill_points_earned
        int hindrance_points_spent
        int hindrance_points_earned
        int hindrance_points_to_edges
        int hindrance_points_to_attributes
        int hindrance_points_to_skills
        int hindrance_points_to_wealth
        int power_points
        int power_points_used
        int wounds
        int fatigue
        int wealth
        string background
        string description
        blob portrait
        string portrait_mime_type
    }

    character_attributes {
        int id PK
        int character_id FK
        int attribute_id FK
        int steps_incremented
    }

    character_skills {
        int id PK
        int character_id FK
        int skill_id FK
        int current_die_size
        int current_die_modifier
    }

    character_edges {
        int id PK
        int character_id FK
        int edge_id FK
        int advance_taken
        string notes
        string source
    }

    character_hindrances {
        int id PK
        int character_id FK
        int hindrance_id FK
        string source
    }

    character_powers {
        int id PK
        int character_id FK
        int power_id FK
        int advance_taken
    }

    character_ancestry_choices {
        int id PK
        int character_id FK
        int choice_id FK
        int selected_option_id FK
    }

    character_arcane_backgrounds {
        int id PK
        int character_id FK
        int arcane_background_id FK
        int advance_taken
    }

    character_modifiers {
        int id PK
        int character_id FK
        int modifier_id FK
        int advance_taken
    }

    character_notes {
        int id PK
        int character_id FK
        string title
        string body
    }

    character_advances {
        int id PK
        int character_id FK
        int advance_number
        string advance_type
        int edge_id FK
        int attribute_id FK
        int skill_id_1 FK
        int skill_id_2 FK
        int hindrance_id FK
        string hindrance_action
        string notes
    }

    %% Relationships - Core
    skills ||--o{ attributes : "linked_attribute_id"
    arcane_backgrounds ||--o{ skills : "arcane_skill_id"
    hindrances ||--o| hindrances : "companion_hindrance_id"

    %% Relationships - Modifiers
    ancestry_modifiers ||--o{ ancestries : "ancestry_id"
    ancestry_modifiers ||--o{ modifiers : "modifier_id"
    edge_modifiers ||--o{ edges : "edge_id"
    edge_modifiers ||--o{ modifiers : "modifier_id"
    hindrance_modifiers ||--o{ hindrances : "hindrance_id"
    hindrance_modifiers ||--o{ modifiers : "modifier_id"
    power_modifiers ||--o{ powers : "power_id"
    power_modifiers ||--o{ modifiers : "modifier_id"

    %% Relationships - Requirements
    requirement_expressions ||--o| requirement_expressions : "parent_id"
    requirement_expressions ||--o| requirements : "requirement_id"
    edge_requirements ||--o{ edges : "edge_id"
    edge_requirements ||--o{ requirement_expressions : "requirement_expression_id"
    hindrance_requirements ||--o{ hindrances : "hindrance_id"
    hindrance_requirements ||--o{ requirement_expressions : "requirement_expression_id"
    ancestry_requirements ||--o{ ancestries : "ancestry_id"
    ancestry_requirements ||--o{ requirement_expressions : "requirement_expression_id"
    arcane_background_requirements ||--o{ arcane_backgrounds : "arcane_background_id"
    arcane_background_requirements ||--o{ requirement_expressions : "requirement_expression_id"
    power_requirements ||--o{ powers : "power_id"
    power_requirements ||--o{ requirement_expressions : "requirement_expression_id"

    %% Relationships - Ancestry Choices
    ancestry_choices ||--o{ ancestries : "ancestry_id"
    ancestry_choice_options ||--o{ ancestry_choices : "choice_id"

    %% Relationships - Characters
    characters ||--o| ancestries : "ancestry_id"
    character_attributes ||--o{ characters : "character_id"
    character_attributes ||--o{ attributes : "attribute_id"
    character_skills ||--o{ characters : "character_id"
    character_skills ||--o{ skills : "skill_id"
    character_edges ||--o{ characters : "character_id"
    character_edges ||--o{ edges : "edge_id"
    character_hindrances ||--o{ characters : "character_id"
    character_hindrances ||--o{ hindrances : "hindrance_id"
    character_powers ||--o{ characters : "character_id"
    character_powers ||--o{ powers : "power_id"
    character_ancestry_choices ||--o{ characters : "character_id"
    character_ancestry_choices ||--o{ ancestry_choices : "choice_id"
    character_ancestry_choices ||--o{ ancestry_choice_options : "selected_option_id"
    character_arcane_backgrounds ||--o{ characters : "character_id"
    character_arcane_backgrounds ||--o{ arcane_backgrounds : "arcane_background_id"
    character_modifiers ||--o{ characters : "character_id"
    character_modifiers ||--o{ modifiers : "modifier_id"
    character_notes ||--o{ characters : "character_id"
    character_advances ||--o{ characters : "character_id"
    character_advances ||--o| edges : "edge_id"
    character_advances ||--o| attributes : "attribute_id"
    character_advances ||--o| skills : "skill_id_1"
    character_advances ||--o| skills : "skill_id_2"
    character_advances ||--o| hindrances : "hindrance_id"

    %% Gear System
    gear_categories {
        int id PK
        string name UK
        string description
    }

    gear {
        int id PK
        string name
        int category_id FK
        string era
        int cost
        real weight
        string source
        string notes
        string description
    }

    weapon_stats {
        int id PK
        int gear_id FK UK
        string damage
        int ap
        int range_short
        int range_medium
        int range_long
        int rof
        int shots
        int min_strength
        bool is_two_handed
        int reach
        string notes
    }

    armor_stats {
        int id PK
        int gear_id FK UK
        int armor_value
        string coverage
        int min_strength
        bool is_heavy
    }

    shield_stats {
        int id PK
        int gear_id FK UK
        int parry_bonus
        int cover_penalty
        int min_strength
        string notes
    }

    ammunition_stats {
        int id PK
        int gear_id FK UK
        string ammo_type
        int quantity_per_unit
        string notes
    }

    gear_modifiers {
        int id PK
        int gear_id FK
        int modifier_id FK
    }

    gear_requirements {
        int id PK
        int gear_id FK
        int requirement_expression_id FK
    }

    character_gear {
        int id PK
        int character_id FK
        int gear_id FK
        int quantity
        bool is_equipped
        string custom_notes
    }

    %% Relationships - Gear System
    gear ||--o{ gear_categories : "category_id"
    weapon_stats ||--|| gear : "gear_id"
    armor_stats ||--|| gear : "gear_id"
    shield_stats ||--|| gear : "gear_id"
    ammunition_stats ||--|| gear : "gear_id"
    gear_modifiers ||--o{ gear : "gear_id"
    gear_modifiers ||--o{ modifiers : "modifier_id"
    gear_requirements ||--o{ gear : "gear_id"
    gear_requirements ||--o{ requirement_expressions : "requirement_expression_id"
    character_gear ||--o{ characters : "character_id"
    character_gear ||--o{ gear : "gear_id"
```

## Table Categories

### Seed Data (Protected)

Core game rules from the SWADE rulebook:

- **`attributes`** - The 5 attributes (Agility, Smarts, Spirit, Strength, Vigor)
- **`skills`** - Skills linked to attributes, with core skills marked
- **`ranks`** - Character progression tiers (Novice, Seasoned, Veteran, Heroic, Legendary)
- **`edges`** - Special abilities by category (Background, Combat, Leadership, etc.)
- **`hindrances`** - Character flaws (minor/major) with point values
- **`ancestries`** - Races/species with modifiers and choices
- **`arcane_backgrounds`** - Magic types (Magic, Miracles, Psionics, etc.)
- **`powers`** - Spells/abilities with PP cost, range, duration

### Modifier System

A flexible system for applying bonuses/penalties:

- **`modifiers`** - Generic effects (die increments, roll bonuses, flat bonuses, descriptions)
- **`*_modifiers`** - Junction tables linking modifiers to edges, hindrances, ancestries, powers

### Requirements System

Boolean expression trees for complex prerequisites:

- **`requirements`** - Atomic prereqs (attribute d8+, skill d6+, specific edge, rank)
- **`requirement_expressions`** - AND/OR/NOT operators for combining requirements
- **`*_requirements`** - Junction tables linking expressions to edges, hindrances, etc.

### Character Data (Writable)

Player-created character information:

- **`characters`** - Core character info, point tracking, wealth, wounds, fatigue
- **`character_attributes`** - Attribute improvements
- **`character_skills`** - Skill die sizes
- **`character_edges`** - Edges taken (with `source`: 'ancestry', 'advancement', 'hindrance_points')
- **`character_hindrances`** - Hindrances taken (with `source`: 'ancestry', 'chosen')
- **`character_powers`** - Powers learned
- **`character_ancestry_choices`** - Selected ancestry options (free edge, ancestral enemy, etc.)
- **`character_arcane_backgrounds`** - Arcane backgrounds acquired by character
- **`character_modifiers`** - Direct modifiers applied to character
- **`character_notes`** - Freeform notes attached to character
- **`character_advances`** - Advancement history tracking each advance taken
- **`character_gear`** - Equipment owned by character (with quantity and equipped status)

### Gear System

Equipment, weapons, armor, and items:

- **`gear_categories`** - Categories of gear (Melee Weapons, Ranged Weapons, Armor, etc.)
- **`gear`** - Base gear items with cost, weight, era, and source
- **`weapon_stats`** - Weapon-specific stats (damage, AP, range, ROF, shots, etc.)
- **`armor_stats`** - Armor-specific stats (armor value, coverage, heavy flag)
- **`shield_stats`** - Shield-specific stats (parry bonus, cover penalty)
- **`ammunition_stats`** - Ammunition-specific stats (ammo type, quantity per unit)
- **`gear_modifiers`** - Junction table linking gear to modifiers
- **`gear_requirements`** - Junction table linking gear to requirement expressions

## Source Tracking

The `character_edges` and `character_hindrances` tables include a `source` column to distinguish how the edge/hindrance was acquired:

| Table | Source Values | Description |
|-------|---------------|-------------|
| `character_edges` | `ancestry` | Free edge from ancestry (e.g., Human's free Novice edge) |
| | `advancement` | Taken during advancement |
| | `hindrance_points` | Purchased with hindrance points at character creation |
| `character_hindrances` | `ancestry` | Required by ancestry (e.g., Android's Pacifist) |
| | `chosen` | Selected during character creation |

This allows counting edges/hindrances for limits while excluding ancestry freebies:

```sql
-- Edges that count against limits
SELECT COUNT(*) FROM character_edges
WHERE character_id = ? AND source != 'ancestry';
```
