-- Your SQL goes here

CREATE TABLE cloudflare_requests
(
    access_request_id TEXT    NOT NULL,
    company_id        INTEGER NOT NULL,
    user              TEXT    NOT NULL,
    account_alias     TEXT    NOT NULL,
    role              TEXT    NOT NULL,

    FOREIGN KEY (access_request_id)
        REFERENCES access_requests (id) ON DELETE CASCADE,

    PRIMARY KEY (access_request_id, user, role)
)
