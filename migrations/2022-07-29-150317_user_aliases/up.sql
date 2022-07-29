-- Your SQL goes here
CREATE TABLE user_aliases
(
    user_id INTEGER NOT NULL,
    aws     TEXT,
    cloudflare      TEXT,
    gcp     TEXT,

    FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE,
    PRIMARY KEY (user_id)
);
