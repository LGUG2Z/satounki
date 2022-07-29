-- Your SQL goes here

CREATE TABLE aws_requests
(
    access_request_id TEXT    NOT NULL,
    company_id        INTEGER NOT NULL,
    user              TEXT    NOT NULL,
    account_alias     TEXT    NOT NULL,
    role              TEXT    NOT NULL,

    FOREIGN KEY (access_request_id)
        REFERENCES access_requests (id) ON DELETE CASCADE,

    FOREIGN KEY (company_id, account_alias)
        REFERENCES company_aws_accounts (company_id, aws_account_alias) ON DELETE NO ACTION,

    PRIMARY KEY (access_request_id, user, account_alias, role)
)
