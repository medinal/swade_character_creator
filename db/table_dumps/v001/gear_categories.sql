PRAGMA foreign_keys=OFF;
BEGIN TRANSACTION;
CREATE TABLE gear_categories (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name VARCHAR(30) NOT NULL UNIQUE,
    description TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
INSERT INTO gear_categories VALUES(1,'Melee Weapons','Hand-to-hand combat weapons','2025-12-30 18:25:49','2025-12-30 18:25:49');
INSERT INTO gear_categories VALUES(2,'Ranged Weapons','Projectile and thrown weapons','2025-12-30 18:25:49','2025-12-30 18:25:49');
INSERT INTO gear_categories VALUES(3,'Armor','Protective gear worn on the body','2025-12-30 18:25:49','2025-12-30 18:25:49');
INSERT INTO gear_categories VALUES(4,'Shields','Defensive equipment held or strapped to arm','2025-12-30 18:25:49','2025-12-30 18:25:49');
INSERT INTO gear_categories VALUES(5,'Common Gear','General adventuring equipment','2025-12-30 18:25:49','2025-12-30 18:25:49');
INSERT INTO gear_categories VALUES(6,'Ammunition','Projectiles for ranged weapons','2025-12-30 18:25:49','2025-12-30 18:25:49');
INSERT INTO gear_categories VALUES(7,'Alchemical Items','Alchemical creations with special effects','2026-01-04 23:28:29','2026-01-04 23:28:29');
INSERT INTO gear_categories VALUES(8,'Animals','Mounts and companion animals','2026-01-04 23:28:29','2026-01-04 23:28:29');
INSERT INTO gear_categories VALUES(9,'Poisons','Harmful substances with delivery methods','2026-01-04 23:28:29','2026-01-04 23:28:29');
INSERT INTO gear_categories VALUES(10,'Clothing','Wearable non-armor items','2026-01-04 23:28:29','2026-01-04 23:28:29');
INSERT INTO gear_categories VALUES(11,'Food & Drink','Consumable provisions','2026-01-04 23:28:29','2026-01-04 23:28:29');
INSERT INTO gear_categories VALUES(12,'Barding','Armor for mounts and animals','2026-01-04 23:28:29','2026-01-04 23:28:29');
INSERT INTO gear_categories VALUES(13,'Packs','Pre-made equipment bundles','2026-01-04 23:28:29','2026-01-04 23:28:29');
COMMIT;
