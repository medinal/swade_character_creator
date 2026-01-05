PRAGMA foreign_keys=OFF;
BEGIN TRANSACTION;
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
INSERT INTO hindrance_requirements VALUES(1,84,474,'2026-01-02 18:47:03','2026-01-02 18:47:03');
INSERT INTO hindrance_requirements VALUES(2,85,475,'2026-01-02 18:47:03','2026-01-02 18:47:03');
INSERT INTO hindrance_requirements VALUES(3,87,228,'2026-01-02 18:47:38','2026-01-02 18:47:38');
INSERT INTO hindrance_requirements VALUES(4,94,231,'2026-01-02 18:48:33','2026-01-02 18:48:33');
INSERT INTO hindrance_requirements VALUES(5,97,234,'2026-01-02 18:48:45','2026-01-02 18:48:45');
INSERT INTO hindrance_requirements VALUES(6,98,253,'2026-01-02 18:48:45','2026-01-02 18:48:45');
COMMIT;
