PRAGMA foreign_keys=OFF;
BEGIN TRANSACTION;
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
INSERT INTO skills VALUES(1,'Athletics','Overall athletic coordination and ability. Climbing, jumping, balancing, wrestling, skiing, swimming, throwing, or catching.',1,1,4,12,0,'core','2025-10-08 06:13:12','2025-10-08 06:13:12');
INSERT INTO skills VALUES(2,'Common Knowledge','General knowledge of a character''s world.',2,1,4,12,0,'core','2025-10-08 06:13:12','2025-10-08 06:13:12');
INSERT INTO skills VALUES(3,'Notice','General awareness and perception.',2,1,4,12,0,'core','2025-10-08 06:13:12','2025-10-08 06:13:12');
INSERT INTO skills VALUES(4,'Persuasion','The ability to convince others to do what you want.',3,1,4,12,0,'core','2025-10-08 06:13:12','2025-10-08 06:13:12');
INSERT INTO skills VALUES(5,'Stealth','The ability to sneak and hide.',1,1,4,12,0,'core','2025-10-08 06:13:12','2025-10-08 06:13:12');
INSERT INTO skills VALUES(6,'Academics','Knowledge of liberal arts, social sciences, literature, history, etc.',2,0,NULL,12,0,'core','2025-10-08 06:13:12','2025-10-08 06:13:12');
INSERT INTO skills VALUES(7,'Battle','Strategy, tactics, and understanding military operations. A key skill in Mass Battles.',2,0,NULL,12,0,'core','2025-10-08 06:13:12','2025-10-08 06:13:12');
INSERT INTO skills VALUES(8,'Boating','Ability to sail or pilot a boat, ship, or other watercraft.',1,0,NULL,12,0,'core','2025-10-08 06:13:12','2025-10-08 06:13:12');
INSERT INTO skills VALUES(9,'Driving','The ability to control, steer, and operate ground vehicles.',1,0,NULL,12,0,'core','2025-10-08 06:13:12','2025-10-08 06:13:12');
INSERT INTO skills VALUES(10,'Electronics','The use of electronic devices and systems.',2,0,NULL,12,0,'core','2025-10-08 06:13:12','2025-10-08 06:13:12');
INSERT INTO skills VALUES(11,'Faith','The arcane skill for Arcane Background (Miracles).',3,0,NULL,12,0,'core','2025-10-08 06:13:12','2025-10-08 06:13:12');
INSERT INTO skills VALUES(12,'Fighting','Skill in armed and unarmed combat.',1,0,NULL,12,0,'core','2025-10-08 06:13:12','2025-10-08 06:13:12');
INSERT INTO skills VALUES(13,'Focus','The arcane skill for Arcane Background (Gifted).',3,0,NULL,12,0,'core','2025-10-08 06:13:12','2025-10-08 06:13:12');
INSERT INTO skills VALUES(14,'Gambling','Skill and familiarity with games of chance.',2,0,NULL,12,0,'core','2025-10-08 06:13:12','2025-10-08 06:13:12');
INSERT INTO skills VALUES(15,'Hacking','Coding, programming, and breaking into computer systems.',2,0,NULL,12,0,'core','2025-10-08 06:13:12','2025-10-08 06:13:12');
INSERT INTO skills VALUES(16,'Healing','The ability to treat and heal Wounds and diseases, and decipher forensic evidence.',2,0,NULL,12,0,'core','2025-10-08 06:13:12','2025-10-08 06:13:12');
INSERT INTO skills VALUES(17,'Intimidation','A character''s ability to threaten others into doing what she wants.',3,0,NULL,12,0,'core','2025-10-08 06:13:12','2025-10-08 06:13:12');
INSERT INTO skills VALUES(18,'Language','Knowledge and fluency in a particular language.',2,0,NULL,12,0,'core','2025-10-08 06:13:12','2025-10-08 06:13:12');
INSERT INTO skills VALUES(19,'Occult','Knowledge of supernatural events, creatures, history, and ways.',2,0,NULL,12,0,'core','2025-10-08 06:13:12','2025-10-08 06:13:12');
INSERT INTO skills VALUES(20,'Performance','Singing, dancing, acting, or other forms of public expression.',3,0,NULL,12,0,'core','2025-10-08 06:13:12','2025-10-08 06:13:12');
INSERT INTO skills VALUES(21,'Piloting','Skill with maneuvering vehicles that operate in three dimensions, such as airplanes, helicopters, spaceships, etc.',1,0,NULL,12,0,'core','2025-10-08 06:13:12','2025-10-08 06:13:12');
INSERT INTO skills VALUES(22,'Psionics','The arcane skill for Arcane Background (Psionics).',2,0,NULL,12,0,'core','2025-10-08 06:13:12','2025-10-08 06:13:12');
INSERT INTO skills VALUES(23,'Repair','The ability to fix mechanical and electrical gadgets.',2,0,NULL,12,0,'core','2025-10-08 06:13:12','2025-10-08 06:13:12');
INSERT INTO skills VALUES(24,'Research','Finding written information from various sources.',2,0,NULL,12,0,'core','2025-10-08 06:13:12','2025-10-08 06:13:12');
INSERT INTO skills VALUES(25,'Riding','A character''s skill in mounting, controlling, and riding a tamed beast.',1,0,NULL,12,0,'core','2025-10-08 06:13:12','2025-10-08 06:13:12');
INSERT INTO skills VALUES(26,'Science','Knowledge of scientific fields such as biology, chemistry, geology, engineering, etc.',2,0,NULL,12,0,'core','2025-10-08 06:13:12','2025-10-08 06:13:12');
INSERT INTO skills VALUES(27,'Shooting','Precision with any type of ranged weapon.',1,0,NULL,12,0,'core','2025-10-08 06:13:12','2025-10-08 06:13:12');
INSERT INTO skills VALUES(28,'Spellcasting','The arcane skill for Arcane Background (Magic).',2,0,NULL,12,0,'core','2025-10-08 06:13:12','2025-10-08 06:13:12');
INSERT INTO skills VALUES(29,'Survival','How to find food, water, or shelter, and tracking.',2,0,NULL,12,0,'core','2025-10-08 06:13:12','2025-10-08 06:13:12');
INSERT INTO skills VALUES(30,'Taunt','Insulting or belittling another. Almost always done as a Test.',2,0,NULL,12,0,'core','2025-10-08 06:13:12','2025-10-08 06:13:12');
INSERT INTO skills VALUES(31,'Thievery','Sleight of hand, pickpocketing, lockpicking, and other typically shady feats.',1,0,NULL,12,0,'core','2025-10-08 06:13:12','2025-10-08 06:13:12');
INSERT INTO skills VALUES(32,'Weird Science','The arcane skill for Arcane Background (Weird Science).',2,0,NULL,12,0,'core','2025-10-08 06:13:12','2025-10-08 06:13:12');
INSERT INTO skills VALUES(33,'Alchemy','This is the arcane skill for alchemists but may also be used to craft alchemical items. It can be used in place of Science when examining chemical reactions, studying reagents, and other related topics.',2,0,NULL,12,0,'fantasy_companion','2026-01-02 18:23:01','2026-01-02 18:23:01');
COMMIT;
