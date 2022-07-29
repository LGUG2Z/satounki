-- Your SQL goes here

CREATE TABLE gcp_requests
(
    access_request_id TEXT    NOT NULL,
    company_id        INTEGER NOT NULL,
    user              TEXT    NOT NULL,
    project           TEXT    NOT NULL,
    role              TEXT    NOT NULL,

    FOREIGN KEY (access_request_id)
        REFERENCES access_requests (id) ON DELETE CASCADE,

    FOREIGN KEY (company_id, project)
        REFERENCES company_gcp_projects (company_id, gcp_project) ON DELETE NO ACTION,

    PRIMARY KEY (access_request_id, user, project, role)
)