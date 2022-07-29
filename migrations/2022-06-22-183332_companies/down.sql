-- This file should undo anything in `up.sql`

DROP TABLE users;
DROP TABLE companies;
DROP TABLE company_roles;
DROP TABLE users_companies;
DROP TABLE users_companies_roles;
DROP TABLE company_policies;
DROP TABLE company_aws_accounts;
DROP TABLE company_cloudflare_accounts;
DROP TABLE company_gcp_projects;
DROP TABLE worker_keys;
DROP TABLE api_tokens;
DROP TABLE user_tokens;
