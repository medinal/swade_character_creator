PRAGMA foreign_keys=OFF;
BEGIN TRANSACTION;
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
COMMIT;
