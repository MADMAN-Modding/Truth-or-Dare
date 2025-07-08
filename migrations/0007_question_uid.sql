ALTER TABLE questions ADD uid INTEGER;
UPDATE questions
SET uid = lower(hex(randomblob(4))) || '-' || lower(hex(randomblob(2))) || '-' ||
          lower(hex(randomblob(2))) || '-' || lower(hex(randomblob(2))) || '-' ||
          lower(hex(randomblob(6)))
WHERE uid IS NULL;