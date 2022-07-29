-- Your SQL goes here

CREATE TABLE company_slack
(
    company_id        INTEGER NOT NULL,
    team_id           TEXT NOT NULL UNIQUE,
    team_name         TEXT NOT NULL,
    channel_id        TEXT NOT NULL UNIQUE,
    access_token      TEXT NOT NULL UNIQUE,
    incoming_webhook  TEXT NOT NULL UNIQUE,

    FOREIGN KEY (company_id)
        REFERENCES companies (id) ON DELETE NO ACTION,

    PRIMARY KEY (company_id)
);


CREATE TABLE requests_slack
(
    access_request_id TEXT    NOT NULL,
    company_id        INTEGER NOT NULL,
    channel_id        TEXT    NOT NULL,
    ts                TEXT    NOT NULL UNIQUE,

    FOREIGN KEY (access_request_id)
        REFERENCES access_requests (id) ON DELETE CASCADE,
    FOREIGN KEY (company_id)
        REFERENCES companies (id) ON DELETE NO ACTION,

    PRIMARY KEY (access_request_id)
);