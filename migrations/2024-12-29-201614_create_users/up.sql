CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(64) NOT NULL,
    email VARCHAR(64) NOT NULL,
    password VARCHAR(128) NOT NULL,
    phone_number VARCHAR(64) NOT NULL,
    description TEXT,
    user_type VARCHAR(64) NOT NULL,
    role VARCHAR(64) NOT NULL,
    event_id INTEGER REFERENCES events(id),
    request VARCHAR(64) NOT NULL,
    image_id INTEGER REFERENCES images(id),
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
