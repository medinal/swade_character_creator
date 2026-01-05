PRAGMA foreign_keys=OFF;
BEGIN TRANSACTION;
CREATE TABLE IF NOT EXISTS "attributes" (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name VARCHAR(20) NOT NULL UNIQUE,
    description TEXT NOT NULL,
    base_value INTEGER NOT NULL DEFAULT 4,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
INSERT INTO attributes VALUES(1,'Agility','A measure of a character''s nimbleness, dexterity, and general coordination.',4,'2025-10-08 06:13:12','2025-10-08 06:13:12');
INSERT INTO attributes VALUES(2,'Smarts','Measures raw intelligence, mental acuity, and how fast a heroine thinks on her feet. It''s used to resist certain types of mental and social attacks.',4,'2025-10-08 06:13:12','2025-10-08 06:13:12');
INSERT INTO attributes VALUES(3,'Spirit','Self-confidence, backbone, and willpower. It''s used to resist social and supernatural attacks as well as fear.',4,'2025-10-08 06:13:12','2025-10-08 06:13:12');
INSERT INTO attributes VALUES(4,'Strength','Physical power and fitness. It''s also used as the basis of a warrior''s damage in hand-to-hand combat, and to determine how much he can wear or carry.',4,'2025-10-08 06:13:12','2025-10-08 06:13:12');
INSERT INTO attributes VALUES(5,'Vigor','Represents an individual''s endurance, resistance to disease, poison, or toxins, and how much physical damage she can take before she can''t go on. It is most often used to resist Fatigue effects, and as the basis for the derived stat of Toughness.',4,'2025-10-08 06:13:12','2025-10-08 06:13:12');
COMMIT;
