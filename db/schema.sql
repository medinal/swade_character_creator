-- ============================================================================
-- SWADE Character Creator Database Schema
-- Tables ordered by dependency (base tables first, then dependent tables)
-- ============================================================================

CREATE TABLE IF NOT EXISTS "attributes" (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name VARCHAR(20) NOT NULL UNIQUE,
    description TEXT NOT NULL,
    base_value INTEGER NOT NULL DEFAULT 4,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE ranks (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name VARCHAR(20) NOT NULL UNIQUE,
    min_advances INTEGER NOT NULL,
    max_advances INTEGER NULL, -- NULL for Legendary (16+)
    description TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE modifiers (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    target_type VARCHAR(20) CHECK (target_type IN ('attribute', 'skill', 'derived_stat', 'edge_choice', 'hindrance_choice', 'heritage_choice', 'skill_points', 'attribute_points', 'power_slots', 'power_points', 'wealth')),
    target_identifier VARCHAR(30), -- nullable for general abilities
    value_type VARCHAR(20) NOT NULL CHECK (value_type IN ('die_increment', 'roll_bonus', 'flat_bonus', 'description', 'bonus_selection', 'mandatory_selection')),
    value INTEGER, -- can be NULL, especially for description type
    description TEXT NOT NULL, -- always present, concise human-readable text
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE requirements (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    requirement_type VARCHAR(20) NOT NULL CHECK (requirement_type IN (
        'attribute', 'skill', 'rank', 'arcane_skill',
        'edge', 'arcane_background', 'wild_card', 'description'
    )),
    target_id INTEGER, -- nullable for types like rank_min, power_points_min
    value INTEGER, -- can be NULL for description type
    description TEXT NOT NULL, -- always present, human-readable requirement text
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE requirement_expressions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    parent_id INTEGER, -- NULL for root nodes
    node_type VARCHAR(15) NOT NULL CHECK (node_type IN ('requirement', 'and', 'or', 'not')),
    requirement_id INTEGER, -- FK to requirements table, NULL for operators
    position INTEGER NOT NULL, -- order within parent (0, 1, 2, etc.)
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (parent_id) REFERENCES requirement_expressions(id),
    FOREIGN KEY (requirement_id) REFERENCES requirements(id),
    -- Constraints to ensure data integrity
    CHECK (
        (node_type = 'requirement' AND requirement_id IS NOT NULL) OR
        (node_type != 'requirement' AND requirement_id IS NULL)
    )
);

CREATE TABLE gear_categories (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name VARCHAR(30) NOT NULL UNIQUE,
    description TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE skills (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name VARCHAR(30) NOT NULL UNIQUE,
    description TEXT NOT NULL,
    linked_attribute_id INTEGER NOT NULL,
    is_core_skill BOOLEAN NOT NULL DEFAULT 0,
    default_die_size INTEGER NULL, -- NULL for non-core skills, 4 for core skills
    max_die_size INTEGER NOT NULL DEFAULT 12,
    max_die_modifier INTEGER NOT NULL DEFAULT 0,
    source VARCHAR(20) NOT NULL DEFAULT 'core',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (linked_attribute_id) REFERENCES attributes(id)
);

CREATE TABLE ancestries (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name VARCHAR(50) NOT NULL,
    source VARCHAR(20) NOT NULL DEFAULT 'core',
    description TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(name, source)
);

CREATE TABLE edges (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name VARCHAR(50) NOT NULL,
    background VARCHAR(20) NOT NULL, -- Edge category: Background, Combat, Leadership, Power, Professional, Social, Weird
    source VARCHAR(20) NOT NULL DEFAULT 'core',
    description TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    can_take_multiple_times BOOLEAN NOT NULL DEFAULT 0, -- Whether this edge can be taken more than once
    UNIQUE(name, source)
);

CREATE TABLE hindrances (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name VARCHAR(50) NOT NULL,
    severity VARCHAR(10) NOT NULL CHECK (severity IN ('minor', 'major')),
    point_value INTEGER NOT NULL, -- 1 for minor, 2 for major
    companion_hindrance_id INTEGER, -- FK to the other variant, NULL if no companion
    source VARCHAR(20) NOT NULL DEFAULT 'core',
    description TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (companion_hindrance_id) REFERENCES hindrances(id)
);

CREATE TABLE powers (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name VARCHAR(50) NOT NULL,
    power_points INTEGER NOT NULL,
    range VARCHAR(30) NOT NULL,
    duration VARCHAR(30) NOT NULL,
    source VARCHAR(20) NOT NULL DEFAULT 'core',
    description TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(name, source)
);

CREATE TABLE gear (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name VARCHAR(100) NOT NULL,
    category_id INTEGER NOT NULL,
    era VARCHAR(20) NOT NULL DEFAULT 'any' CHECK (era IN ('medieval', 'modern', 'futuristic', 'any')),
    cost INTEGER NOT NULL DEFAULT 0, -- Cost in dollars
    weight REAL NOT NULL DEFAULT 0, -- Weight in pounds
    source VARCHAR(20) NOT NULL DEFAULT 'core',
    notes TEXT, -- Special notes about the item
    description TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (category_id) REFERENCES gear_categories(id),
    UNIQUE(name, source)
);

CREATE TABLE arcane_backgrounds (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name VARCHAR(50) NOT NULL UNIQUE,
    arcane_skill_id INTEGER NOT NULL,
    starting_powers INTEGER NOT NULL,
    starting_power_points INTEGER NOT NULL,
    has_power_list BOOLEAN NOT NULL DEFAULT 0, -- FC arcane backgrounds have limited power lists
    source VARCHAR(20) NOT NULL DEFAULT 'core',
    description TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (arcane_skill_id) REFERENCES skills(id)
);

CREATE TABLE arcane_background_choices (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    arcane_background_id INTEGER NOT NULL,
    choice_type VARCHAR(50) NOT NULL, -- 'available_power', 'required_starting_power', 'choosable_starting_power', 'built_in_hindrance', 'special_ability', 'edge_category'
    choice_category VARCHAR(50), -- For filtering/grouping (e.g., 'domain' for cleric powers)
    min_selections INTEGER NOT NULL DEFAULT 0,
    max_selections INTEGER NOT NULL DEFAULT 1,
    description TEXT NOT NULL,
    position INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (arcane_background_id) REFERENCES arcane_backgrounds(id) ON DELETE CASCADE
);

CREATE TABLE arcane_background_choice_options (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    choice_id INTEGER NOT NULL,
    option_type VARCHAR(50) NOT NULL, -- 'power', 'hindrance', 'ability', 'edge_category'
    option_id INTEGER, -- FK to powers/hindrances (NULL for abilities/edge_categories)
    option_description TEXT, -- For abilities or power limitations (e.g., "Summoned creatures only")
    position INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (choice_id) REFERENCES arcane_background_choices(id) ON DELETE CASCADE
);

CREATE TABLE arcane_background_requirements (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    arcane_background_id INTEGER NOT NULL,
    requirement_expression_id INTEGER NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (arcane_background_id) REFERENCES arcane_backgrounds(id),
    FOREIGN KEY (requirement_expression_id) REFERENCES requirement_expressions(id),
    UNIQUE(arcane_background_id, requirement_expression_id)
);

CREATE TABLE ancestry_choices (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    ancestry_id INTEGER NOT NULL,
    choice_type VARCHAR(50) NOT NULL, -- 'free_edge', 'mandatory_hindrance', 'heritage_trait', etc.
    choice_category VARCHAR(50), -- 'background_edge', 'combat_edge', etc. (for filtering)
    min_selections INTEGER NOT NULL DEFAULT 1,
    max_selections INTEGER NOT NULL DEFAULT 1,
    description TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (ancestry_id) REFERENCES ancestries(id) ON DELETE CASCADE
);

CREATE TABLE ancestry_choice_options (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    choice_id INTEGER NOT NULL,
    option_type VARCHAR(50) NOT NULL, -- 'edge', 'hindrance', 'modifier'
    option_id INTEGER, -- specific ID or NULL for "any"
    option_description TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (choice_id) REFERENCES ancestry_choices(id) ON DELETE CASCADE
);

CREATE TABLE ancestry_modifiers (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    ancestry_id INTEGER NOT NULL,
    modifier_id INTEGER NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (ancestry_id) REFERENCES ancestries(id),
    FOREIGN KEY (modifier_id) REFERENCES modifiers(id),
    UNIQUE(ancestry_id, modifier_id)
);

CREATE TABLE ancestry_requirements (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    ancestry_id INTEGER NOT NULL,
    requirement_expression_id INTEGER NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (ancestry_id) REFERENCES ancestries(id),
    FOREIGN KEY (requirement_expression_id) REFERENCES requirement_expressions(id),
    UNIQUE(ancestry_id, requirement_expression_id)
);

CREATE TABLE edge_modifiers (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    edge_id INTEGER NOT NULL,
    modifier_id INTEGER NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (edge_id) REFERENCES edges(id),
    FOREIGN KEY (modifier_id) REFERENCES modifiers(id),
    UNIQUE(edge_id, modifier_id)
);

CREATE TABLE edge_requirements (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    edge_id INTEGER NOT NULL,
    requirement_expression_id INTEGER NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (edge_id) REFERENCES edges(id),
    FOREIGN KEY (requirement_expression_id) REFERENCES requirement_expressions(id),
    UNIQUE(edge_id, requirement_expression_id)
);

CREATE TABLE hindrance_modifiers (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    hindrance_id INTEGER NOT NULL,
    modifier_id INTEGER NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (hindrance_id) REFERENCES hindrances(id),
    FOREIGN KEY (modifier_id) REFERENCES modifiers(id),
    UNIQUE(hindrance_id, modifier_id)
);

CREATE TABLE hindrance_requirements (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    hindrance_id INTEGER NOT NULL,
    requirement_expression_id INTEGER NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (hindrance_id) REFERENCES hindrances(id),
    FOREIGN KEY (requirement_expression_id) REFERENCES requirement_expressions(id),
    UNIQUE(hindrance_id, requirement_expression_id)
);

CREATE TABLE power_modifiers (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    power_id INTEGER NOT NULL,
    modifier_id INTEGER NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (power_id) REFERENCES powers(id),
    FOREIGN KEY (modifier_id) REFERENCES modifiers(id),
    UNIQUE(power_id, modifier_id)
);

CREATE TABLE power_requirements (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    power_id INTEGER NOT NULL,
    requirement_expression_id INTEGER NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (power_id) REFERENCES powers(id),
    FOREIGN KEY (requirement_expression_id) REFERENCES requirement_expressions(id),
    UNIQUE(power_id, requirement_expression_id)
);

CREATE TABLE weapon_stats (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    gear_id INTEGER NOT NULL UNIQUE,
    damage VARCHAR(30) NOT NULL, -- e.g., "Str+d4", "2d6", "3d6"
    ap INTEGER NOT NULL DEFAULT 0, -- Armor Piercing value
    range_short INTEGER, -- Short range in inches (NULL for melee)
    range_medium INTEGER, -- Medium range in inches (NULL for melee)
    range_long INTEGER, -- Long range in inches (NULL for melee)
    rof INTEGER, -- Rate of Fire (NULL for melee)
    shots INTEGER, -- Magazine capacity (NULL for melee/thrown)
    min_strength INTEGER, -- Minimum strength die size (4, 6, 8, 10, 12)
    is_two_handed BOOLEAN NOT NULL DEFAULT 0,
    reach INTEGER, -- Melee reach in inches (0 for normal, 1+ for extended reach)
    blast_template VARCHAR(10), -- 'SBT', 'MBT', 'LBT' for area effect weapons (alchemical items)
    notes TEXT, -- Weapon-specific notes (reload, snapfire, etc.)
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (gear_id) REFERENCES gear(id) ON DELETE CASCADE
);

CREATE TABLE armor_stats (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    gear_id INTEGER NOT NULL UNIQUE,
    armor_value INTEGER NOT NULL, -- Armor bonus to Toughness
    coverage VARCHAR(50) NOT NULL, -- Body parts covered: "torso", "torso, arms", "head", etc.
    min_strength INTEGER, -- Minimum strength die size
    is_heavy BOOLEAN NOT NULL DEFAULT 0, -- Heavy armor flag
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (gear_id) REFERENCES gear(id) ON DELETE CASCADE
);

CREATE TABLE shield_stats (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    gear_id INTEGER NOT NULL UNIQUE,
    parry_bonus INTEGER NOT NULL, -- Bonus to Parry
    cover_penalty INTEGER NOT NULL DEFAULT 0, -- Penalty to attackers' ranged attacks
    min_strength INTEGER, -- Minimum strength die size
    notes TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (gear_id) REFERENCES gear(id) ON DELETE CASCADE
);

CREATE TABLE ammunition_stats (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    gear_id INTEGER NOT NULL UNIQUE,
    ammo_type VARCHAR(30) NOT NULL, -- e.g., "pistol", "rifle", "shotgun", "arrow"
    quantity_per_unit INTEGER NOT NULL DEFAULT 1, -- How many rounds per purchase
    notes TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (gear_id) REFERENCES gear(id) ON DELETE CASCADE
);

CREATE TABLE poison_stats (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    gear_id INTEGER NOT NULL UNIQUE,
    poison_type VARCHAR(30) NOT NULL, -- 'Lethal', 'Knockout', 'Paralyzing', 'Mild', 'Disabling'
    delivery_method VARCHAR(20) NOT NULL, -- 'Contact', 'Inhaled', 'Ingested', 'Injury'
    affected_attribute VARCHAR(20), -- For Disabling type: 'Agility', 'Smarts', 'Spirit', 'Strength', 'Vigor'
    notes TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (gear_id) REFERENCES gear(id) ON DELETE CASCADE
);

CREATE TABLE pack_contents (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    pack_gear_id INTEGER NOT NULL, -- The pack (e.g., Adventurer's Pack)
    item_gear_id INTEGER NOT NULL, -- An item in the pack (e.g., Backpack)
    quantity INTEGER NOT NULL DEFAULT 1,
    notes TEXT, -- For items like "1 week's rations"
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (pack_gear_id) REFERENCES gear(id) ON DELETE CASCADE,
    FOREIGN KEY (item_gear_id) REFERENCES gear(id) ON DELETE CASCADE
);

CREATE TABLE gear_modifiers (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    gear_id INTEGER NOT NULL,
    modifier_id INTEGER NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (gear_id) REFERENCES gear(id) ON DELETE CASCADE,
    FOREIGN KEY (modifier_id) REFERENCES modifiers(id),
    UNIQUE(gear_id, modifier_id)
);

CREATE TABLE gear_requirements (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    gear_id INTEGER NOT NULL,
    requirement_expression_id INTEGER NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (gear_id) REFERENCES gear(id) ON DELETE CASCADE,
    FOREIGN KEY (requirement_expression_id) REFERENCES requirement_expressions(id),
    UNIQUE(gear_id, requirement_expression_id)
);

CREATE TABLE characters (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    is_wild_card BOOLEAN NOT NULL DEFAULT 1,
    name VARCHAR(100) NOT NULL,
    ancestry_id INTEGER,

    -- Character Creation Tracking
    attribute_points_spent INTEGER NOT NULL DEFAULT 0, -- Starting points spent on attributes
    attribute_points_earned INTEGER NOT NULL DEFAULT 5, -- Start points available for attributes
    skill_points_spent INTEGER NOT NULL DEFAULT 0,    -- Starting points spent on skills
    skill_points_earned INTEGER NOT NULL DEFAULT 12, -- Starting points available for skills (after core skills)
    hindrance_points_spent INTEGER NOT NULL DEFAULT 0,    -- Starting points spent on hindrances
    hindrance_points_earned INTEGER NOT NULL DEFAULT 0, -- Points from hindrances (max 4)

    -- Hindrance Points Conversion Tracking
    hindrance_points_to_edges INTEGER NOT NULL DEFAULT 0, -- Hindrance points converted to edge points (2:1 ratio)
    hindrance_points_to_attributes INTEGER NOT NULL DEFAULT 0, -- Hindrance points converted to attribute points (2:1 ratio)
    hindrance_points_to_skills INTEGER NOT NULL DEFAULT 0, -- Hindrance points converted to skill points (1:1 ratio)
    hindrance_points_to_wealth INTEGER NOT NULL DEFAULT 0, -- Hindrance points converted to wealth (1:1 ratio, $1000 per point)

    -- Additional Character Info
    power_points INTEGER NOT NULL DEFAULT 0,
    power_points_used INTEGER NOT NULL DEFAULT 0,
    wounds INTEGER NOT NULL DEFAULT 0,
    fatigue INTEGER NOT NULL DEFAULT 0,
    wealth INTEGER NOT NULL DEFAULT 500, -- Starting wealth amount
    background TEXT, -- Character background/history
    description TEXT, -- Physical description, personality, etc.

    -- Portrait (stored as resized image, max 1024x1024)
    portrait BLOB, -- Raw image bytes (PNG/JPEG)
    portrait_mime_type TEXT, -- 'image/png' or 'image/jpeg'

    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (ancestry_id) REFERENCES ancestries(id)
);

CREATE TABLE character_advances (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    character_id INTEGER NOT NULL,
    advance_number INTEGER NOT NULL,  -- 1, 2, 3, etc.
    advance_type VARCHAR(30) NOT NULL,  -- 'edge', 'attribute', 'skill_expensive', 'skill_cheap', 'hindrance'
    -- For edge advances
    edge_id INTEGER,
    -- For attribute advances
    attribute_id INTEGER,
    -- For skill advances (expensive = 1 skill, cheap = 2 skills)
    skill_id_1 INTEGER,
    skill_id_2 INTEGER,  -- Only used for cheap skill advances
    -- For hindrance advances
    hindrance_id INTEGER,
    hindrance_action VARCHAR(20),  -- 'remove_minor', 'reduce_major', 'remove_major_half'
    -- Metadata
    notes TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (character_id) REFERENCES characters(id) ON DELETE CASCADE,
    FOREIGN KEY (edge_id) REFERENCES edges(id),
    FOREIGN KEY (attribute_id) REFERENCES attributes(id),
    FOREIGN KEY (skill_id_1) REFERENCES skills(id),
    FOREIGN KEY (skill_id_2) REFERENCES skills(id),
    FOREIGN KEY (hindrance_id) REFERENCES hindrances(id),
    UNIQUE (character_id, advance_number)
);

CREATE TABLE character_ancestry_choices (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    character_id INTEGER NOT NULL,
    choice_id INTEGER NOT NULL,
    selected_option_id INTEGER NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (character_id) REFERENCES characters(id) ON DELETE CASCADE,
    FOREIGN KEY (choice_id) REFERENCES ancestry_choices(id),
    FOREIGN KEY (selected_option_id) REFERENCES ancestry_choice_options(id),
    UNIQUE(character_id, choice_id, selected_option_id) -- Prevent duplicate selections
);

CREATE TABLE character_arcane_backgrounds (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    character_id INTEGER NOT NULL,
    arcane_background_id INTEGER NOT NULL,
    advance_taken INTEGER DEFAULT 0, -- Which advance was used to take this Arcane Background edge
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (character_id) REFERENCES characters(id),
    FOREIGN KEY (arcane_background_id) REFERENCES arcane_backgrounds(id),
    UNIQUE(character_id, arcane_background_id)
);

CREATE TABLE character_arcane_background_choices (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    character_id INTEGER NOT NULL,
    choice_id INTEGER NOT NULL,
    selected_option_id INTEGER NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (character_id) REFERENCES characters(id) ON DELETE CASCADE,
    FOREIGN KEY (choice_id) REFERENCES arcane_background_choices(id),
    FOREIGN KEY (selected_option_id) REFERENCES arcane_background_choice_options(id),
    UNIQUE(character_id, choice_id, selected_option_id) -- Prevent duplicate selections
);

CREATE TABLE IF NOT EXISTS "character_attributes" (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    character_id INTEGER NOT NULL,
    attribute_id INTEGER NOT NULL,
    steps_incremented INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (character_id) REFERENCES characters(id),
    FOREIGN KEY (attribute_id) REFERENCES attributes(id),
    UNIQUE(character_id, attribute_id)
);

CREATE TABLE character_skills (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    character_id INTEGER NOT NULL,
    skill_id INTEGER NOT NULL,
    current_die_size INTEGER, -- NULL for untrained skills
    current_die_modifier INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (character_id) REFERENCES characters(id),
    FOREIGN KEY (skill_id) REFERENCES skills(id),
    UNIQUE(character_id, skill_id)
);

CREATE TABLE character_edges (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    character_id INTEGER NOT NULL,
    edge_id INTEGER NOT NULL,
    advance_taken INTEGER NOT NULL, -- Which advance number was used to take this edge
    notes TEXT, -- Optional notes for edges taken multiple times (e.g., which weapon for Trademark Weapon, which skill for Scholar)
    source VARCHAR(20) NOT NULL DEFAULT 'advancement', -- 'ancestry', 'advancement', 'hindrance_points'
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (character_id) REFERENCES characters(id),
    FOREIGN KEY (edge_id) REFERENCES edges(id)
    -- Note: UNIQUE constraint removed to allow taking same edge multiple times (for edges where can_take_multiple_times = 1)
);

CREATE TABLE character_hindrances (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    character_id INTEGER NOT NULL,
    hindrance_id INTEGER NOT NULL,
    source VARCHAR(20) NOT NULL DEFAULT 'chosen', -- 'ancestry', 'chosen'
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (character_id) REFERENCES characters(id),
    FOREIGN KEY (hindrance_id) REFERENCES hindrances(id),
    UNIQUE(character_id, hindrance_id)
);

CREATE TABLE character_powers (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    character_id INTEGER NOT NULL,
    power_id INTEGER NOT NULL,
    advance_taken INTEGER, -- Which advance was used (NULL for starting powers from arcane background)
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (character_id) REFERENCES characters(id),
    FOREIGN KEY (power_id) REFERENCES powers(id),
    UNIQUE(character_id, power_id)
);

CREATE TABLE character_modifiers (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    character_id INTEGER NOT NULL,
    modifier_id INTEGER NOT NULL,
    advance_taken INTEGER, -- Which advance was used (NULL for character creation)
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (character_id) REFERENCES characters(id),
    FOREIGN KEY (modifier_id) REFERENCES modifiers(id),
    UNIQUE(character_id, modifier_id) -- Enforces one-to-many
);

CREATE TABLE character_gear (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    character_id INTEGER NOT NULL,
    gear_id INTEGER NOT NULL,
    quantity INTEGER NOT NULL DEFAULT 1,
    is_equipped BOOLEAN NOT NULL DEFAULT 0,
    custom_notes TEXT, -- Player notes about this specific item
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (character_id) REFERENCES characters(id) ON DELETE CASCADE,
    FOREIGN KEY (gear_id) REFERENCES gear(id)
);

CREATE TABLE character_notes (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    character_id INTEGER NOT NULL,
    title VARCHAR(100) NOT NULL,
    body TEXT NOT NULL DEFAULT '',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (character_id) REFERENCES characters(id) ON DELETE CASCADE
);

-- ============================================================================
-- INDEXES
-- ============================================================================

