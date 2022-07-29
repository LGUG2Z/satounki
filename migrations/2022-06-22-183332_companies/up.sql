-- Your SQL goes here

CREATE TABLE users
(
    id         INTEGER PRIMARY KEY NOT NULL,
    email      TEXT UNIQUE         NOT NULL,
    first_name TEXT                NOT NULL,
    last_name  TEXT                NOT NULL,
    active     INTEGER             NOT NULL DEFAULT (TRUE)
);

CREATE TABLE companies
(
    id                  INTEGER PRIMARY KEY NOT NULL,
    name                TEXT                NOT NULL,
    domain              TEXT                NOT NULL UNIQUE,
    root_user           TEXT                NOT NULL,
--     active              INTEGER             NOT NULL DEFAULT (TRUE),
--     expiry              TEXT                NOT NULL,
--     user_seats          INTEGER             NOT NULL DEFAULT 10,
--     approver_seats      INTEGER             NOT NULL DEFAULT 3,
--     administrator_seats INTEGER             NOT NULL DEFAULT 1,
--     account_limit       INTEGER             NOT NULL DEFAULT 1,

    FOREIGN KEY (root_user) REFERENCES users (email)
);

CREATE TABLE company_roles
(
    id   INTEGER PRIMARY KEY NOT NULL,
    name TEXT                NOT NULL
);

INSERT INTO company_roles (id, name)
VALUES (1, 'administrator');

INSERT INTO company_roles (id, name)
VALUES (2, 'approver');

INSERT INTO company_roles (id, name)
VALUES (3, 'user');

CREATE TABLE users_companies
(
    user_id    INTEGER NOT NULL,
    company_id INTEGER NOT NULL,

    FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE,
    FOREIGN KEY (company_id) REFERENCES companies (id) ON DELETE CASCADE,
    PRIMARY KEY (user_id, company_id)
);

CREATE TABLE users_companies_roles
(
    user_id    INTEGER NOT NULL,
    company_id INTEGER NOT NULL,
    role_id    INTEGER NOT NULL,

    FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE,
    FOREIGN KEY (company_id) REFERENCES companies (id) ON DELETE CASCADE,
    FOREIGN KEY (role_id) REFERENCES company_roles (id) ON DELETE CASCADE,
    PRIMARY KEY (user_id, company_id, role_id)
);

CREATE TABLE worker_keys
(
    company_id INTEGER     NOT NULL,
    key        TEXT UNIQUE NOT NULL,

    FOREIGN KEY (company_id) REFERENCES companies (id) ON DELETE CASCADE,
    PRIMARY KEY (company_id)
);

CREATE TABLE company_policies
(
    id          TEXT PRIMARY KEY UNIQUE NOT NULL,
    company_id  INTEGER                 NOT NULL,
    name        TEXT                    NOT NULL,
    policy      TEXT                    NOT NULL,
    description TEXT                    NOT NULL,

    FOREIGN KEY (company_id) REFERENCES companies (id) ON DELETE CASCADE,
    UNIQUE (company_id, name)
);

CREATE TABLE company_aws_accounts
(
    id                      TEXT PRIMARY KEY UNIQUE NOT NULL,
    company_id              INTEGER                 NOT NULL,
    aws_account_alias       TEXT                    NOT NULL,
    approvals_required      INTEGER                 NOT NULL,
    admin_approval_required INTEGER                 NOT NULL,

    FOREIGN KEY (company_id) REFERENCES companies (id) ON DELETE CASCADE,
    UNIQUE (company_id, aws_account_alias)
);

CREATE TABLE company_gcp_projects
(
    id                      TEXT PRIMARY KEY UNIQUE NOT NULL,
    company_id              INTEGER                 NOT NULL,
    gcp_project             TEXT                    NOT NULL,
    approvals_required      INTEGER                 NOT NULL,
    admin_approval_required INTEGER                 NOT NULL,

    FOREIGN KEY (company_id) REFERENCES companies (id) ON DELETE CASCADE,
    UNIQUE (company_id, gcp_project)
);

CREATE TABLE company_cloudflare_accounts
(
    id                       TEXT PRIMARY KEY UNIQUE NOT NULL,
    company_id               INTEGER                 NOT NULL,
    cloudflare_account_alias TEXT                    NOT NULL,
    approvals_required       INTEGER                 NOT NULL,
    admin_approval_required  INTEGER                 NOT NULL,

    FOREIGN KEY (company_id) REFERENCES companies (id) ON DELETE CASCADE,
    UNIQUE (company_id, cloudflare_account_alias)
);

CREATE TABLE api_tokens
(
    token      TEXT PRIMARY KEY UNIQUE NOT NULL,
    company_id INTEGER                 NOT NULL,

    FOREIGN KEY (company_id) REFERENCES companies (id) ON DELETE CASCADE
);

CREATE TABLE user_tokens
(
    token   TEXT PRIMARY KEY UNIQUE NOT NULL,
    user_id INTEGER                 NOT NULL,

    FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE
);
