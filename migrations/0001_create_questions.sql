CREATE TABLE IF NOT EXISTS questions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    prompt TEXT NOT NULL,
    question_type TEXT CHECK(question_type IN ('TRUTH', 'DARE')) NOT NULL,
    rating TEXT CHECK(rating IN ('PG', 'PG-13')) NOT NULL
)