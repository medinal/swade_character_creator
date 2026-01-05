PRAGMA foreign_keys=OFF;
BEGIN TRANSACTION;
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
COMMIT;
