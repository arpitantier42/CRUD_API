-- Your SQL goes here
CREATE TABLE my_todos (
    id SERIAL PRIMARY KEY,
    fantasy_name VARCHAR NOT NULL,
    real_name VARCHAR NULL,
    spotted_photo TEXT NOT NULL,
    strength_level INT NOT NULL DEFAULT 0
);