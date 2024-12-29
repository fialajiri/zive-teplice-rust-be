CREATE TABLE events (
    id SERIAL PRIMARY KEY,
    title VARCHAR(128) NOT NULL,
    year INTEGER NOT NULL,
    is_current BOOLEAN NOT NULL,
    program_title VARCHAR(128),
    program_text TEXT,
    image_id INTEGER REFERENCES images(id),
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

