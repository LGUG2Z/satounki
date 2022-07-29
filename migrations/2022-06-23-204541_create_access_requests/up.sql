-- Your SQL goes here

CREATE TABLE access_requests
(
    id            TEXT PRIMARY KEY NOT NULL,
    company_id    INTEGER          NOT NULL,
    requester     TEXT             NOT NULL REFERENCES users (email),
    timestamp     TEXT             NOT NULL,
    duration      INTEGER          NOT NULL,
    approved      INTEGER          NOT NULL DEFAULT (FALSE),
    access_expiry TEXT,
    state         TEXT             NOT NULL,
    modified      TEXT             NOT NULL,
    req_alias     TEXT UNIQUE      NOT NULL,
    policy        TEXT             NOT NULL,

    FOREIGN KEY (company_id, policy) REFERENCES company_policies (company_id, name) ON DELETE NO ACTION
)