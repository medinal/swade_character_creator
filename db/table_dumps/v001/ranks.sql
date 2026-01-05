PRAGMA foreign_keys=OFF;
BEGIN TRANSACTION;
CREATE TABLE ranks (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name VARCHAR(20) NOT NULL UNIQUE,
    min_advances INTEGER NOT NULL,
    max_advances INTEGER NULL, -- NULL for Legendary (16+)
    description TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
INSERT INTO ranks VALUES(1,'Novice',0,3,'Starting rank for new characters. Characters begin with 0 advances and remain Novice until they have earned 4 advances.','2025-10-08 06:13:12','2025-10-08 06:13:12');
INSERT INTO ranks VALUES(2,'Seasoned',4,7,'Characters who have gained some experience and proven themselves. Unlocks access to more powerful Edges and abilities.','2025-10-08 06:13:12','2025-10-08 06:13:12');
INSERT INTO ranks VALUES(3,'Veteran',8,11,'Experienced characters who have survived many adventures. Can access veteran-level Edges and abilities.','2025-10-08 06:13:12','2025-10-08 06:13:12');
INSERT INTO ranks VALUES(4,'Heroic',12,15,'Truly heroic characters of great renown. Can access heroic-level Edges and legendary abilities.','2025-10-08 06:13:12','2025-10-08 06:13:12');
INSERT INTO ranks VALUES(5,'Legendary',16,NULL,'The most powerful characters in the setting. Can access the most potent Edges and abilities. No upper limit on advances.','2025-10-08 06:13:12','2025-10-08 06:13:12');
COMMIT;
