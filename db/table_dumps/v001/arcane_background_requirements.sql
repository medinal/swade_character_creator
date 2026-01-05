PRAGMA foreign_keys=OFF;
BEGIN TRANSACTION;
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
INSERT INTO arcane_background_requirements VALUES(1,1,469,'2025-10-18 03:20:11','2025-10-18 03:20:11');
INSERT INTO arcane_background_requirements VALUES(2,2,470,'2025-10-18 03:20:11','2025-10-18 03:20:11');
INSERT INTO arcane_background_requirements VALUES(3,3,471,'2025-10-18 03:20:11','2025-10-18 03:20:11');
INSERT INTO arcane_background_requirements VALUES(4,4,472,'2025-10-18 03:20:11','2025-10-18 03:20:11');
INSERT INTO arcane_background_requirements VALUES(5,5,473,'2025-10-18 03:20:11','2025-10-18 03:20:11');
COMMIT;
