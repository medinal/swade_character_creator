PRAGMA foreign_keys=OFF;
BEGIN TRANSACTION;
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
COMMIT;
