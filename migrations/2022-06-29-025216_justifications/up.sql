-- Your SQL goes here

CREATE TABLE justifications
(
    access_request_id TEXT NOT NULL,
    justification     TEXT NOT NULL,

    FOREIGN KEY (access_request_id)
        REFERENCES access_requests (id) ON DELETE CASCADE,

    PRIMARY KEY (access_request_id)
)