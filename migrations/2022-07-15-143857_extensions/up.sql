-- Your SQL goes here

CREATE TABLE extensions
(
    access_request_id TEXT    NOT NULL,
    user              TEXT    NOT NULL REFERENCES users (email),
    timestamp         TEXT    NOT NULL,
    duration          INTEGER NOT NULL,

    FOREIGN KEY (access_request_id)
        REFERENCES access_requests (id) ON DELETE CASCADE,

    PRIMARY KEY (access_request_id, user)
)
