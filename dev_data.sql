REPLACE INTO users (id, email, first_name, last_name, active)
VALUES (1, 'lgug2z@satounki.com', 'Jeezy', 'LGUG2Z', 1);

REPLACE INTO users (id, email, first_name, last_name, active)
VALUES (2, 'samir@satounki.com', 'Samir', 'Jan', 1);

REPLACE INTO companies (id, name, domain, root_user)
VALUES (1, 'Satounki Development', 'satounki.com', 'lgug2z@satounki.com');

REPLACE INTO users_companies (user_id, company_id)
VALUES (1, 1);

REPLACE INTO users_companies (user_id, company_id)
VALUES (2, 1);

REPLACE INTO worker_keys (company_id, key)
VALUES (1, 'swk-e0c43bd0-38a4-4e7b-9c0f-8bd5f47f20d2');

REPLACE INTO api_tokens (token, company_id)
VALUES ('e-mo-tion', 1);

REPLACE INTO user_tokens (token, user_id)
VALUES ('crj', 1);

REPLACE INTO user_tokens (token, user_id)
VALUES ('smr', 2)
