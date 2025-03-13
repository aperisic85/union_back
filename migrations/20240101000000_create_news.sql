-- migrations/YYYYMMDDHHMMSS_create_news.sql
CREATE TABLE news (
    id SERIAL PRIMARY KEY,
    title TEXT NOT NULL,
    content TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW () -- Dodajte NOT NULL
);
