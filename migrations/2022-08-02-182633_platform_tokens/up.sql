-- Your SQL goes here

CREATE TABLE platform_tokens
(
    token TEXT PRIMARY KEY UNIQUE NOT NULL,
    scope  TEXT UNIQUE             NOT NULL
);