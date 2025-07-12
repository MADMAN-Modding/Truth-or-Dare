-- Rename the old table
ALTER TABLE guild_settings RENAME TO guild_settings_old;

-- Create the new table with the updated CHECK constraint
CREATE TABLE guild_settings (
    guild_id INTEGER PRIMARY KEY,
    rating TEXT CHECK(rating IN ('PG', 'PG-13', 'ALL')) NOT NULL
);

-- Copy old data
INSERT INTO guild_settings (guild_id, rating)
SELECT guild_id, rating FROM guild_settings_old;

-- Drop the old table
DROP TABLE guild_settings_old;