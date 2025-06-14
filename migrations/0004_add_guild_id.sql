-- This migration adds a guild_id column to the questions table.
ALTER TABLE questions ADD COLUMN guild_id INTEGER DEFAULT NULL;