CREATE TABLE IF NOT EXISTS guild_settings (
    guild_id INTEGER PRIMARY KEY,
    rating TEXT CHECK(rating IN ('PG', 'PG-13')) NOT NULL
)