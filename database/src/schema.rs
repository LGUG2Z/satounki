// @generated automatically by Diesel CLI.

diesel::table! {
    use diesel::sql_types::*;
    use common::AccessRequestStateMapping;

    access_requests (id) {
        id -> Text,
        company_id -> Integer,
        requester -> Text,
        timestamp -> TimestamptzSqlite,
        duration -> Integer,
        approved -> Bool,
        access_expiry -> Nullable<TimestamptzSqlite>,
        state -> AccessRequestStateMapping,
        modified -> TimestamptzSqlite,
        req_alias -> Text,
        policy -> Text,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use common::AccessRequestStateMapping;
    use common::CloudflareRoleMapping;

    api_tokens (token) {
        token -> Text,
        company_id -> Integer,
    }
}

diesel::table! {
    use diesel::sql_types::*;

    approvals (access_request_id, user) {
        access_request_id -> Text,
        user -> Text,
        timestamp -> TimestamptzSqlite,
    }
}

diesel::table! {
    use diesel::sql_types::*;

    aws_requests (access_request_id, user, account_alias, role) {
        access_request_id -> Text,
        company_id -> Integer,
        user -> Text,
        account_alias -> Text,
        role -> Text,
    }
}

diesel::table! {
    use diesel::sql_types::*;

    cancellations (access_request_id) {
        access_request_id -> Text,
        user -> Text,
        timestamp -> TimestamptzSqlite,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use common::CloudflareRoleMapping;

    cloudflare_requests (access_request_id, user, role) {
        access_request_id -> Text,
        company_id -> Integer,
        user -> Text,
        account_alias -> Text,
        role -> CloudflareRoleMapping,
    }
}

diesel::table! {
    use diesel::sql_types::*;

    companies (id) {
        id -> Integer,
        name -> Text,
        domain -> Text,
        root_user -> Text,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use common::AccessRequestStateMapping;
    use common::CloudflareRoleMapping;

    company_aws_accounts (id) {
        id -> Text,
        company_id -> Integer,
        aws_account_alias -> Text,
        approvals_required -> Integer,
        admin_approval_required -> Bool,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use common::AccessRequestStateMapping;
    use common::CloudflareRoleMapping;

    company_cloudflare_accounts (id) {
        id -> Text,
        company_id -> Integer,
        cloudflare_account_alias -> Text,
        approvals_required -> Integer,
        admin_approval_required -> Bool,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use common::AccessRequestStateMapping;
    use common::CloudflareRoleMapping;

    company_gcp_projects (id) {
        id -> Text,
        company_id -> Integer,
        gcp_project -> Text,
        approvals_required -> Integer,
        admin_approval_required -> Bool,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use common::AccessRequestStateMapping;
    use common::CloudflareRoleMapping;

    company_policies (id) {
        id -> Text,
        company_id -> Integer,
        name -> Text,
        policy -> Text,
        description -> Text,
    }
}

diesel::table! {
    use diesel::sql_types::*;

    gcp_requests (access_request_id, user, project, role) {
        access_request_id -> Text,
        company_id -> Integer,
        user -> Text,
        project -> Text,
        role -> Text,
    }
}

diesel::table! {
    use diesel::sql_types::*;

    justifications (access_request_id) {
        access_request_id -> Text,
        justification -> Text,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use common::AccessRequestStateMapping;
    use common::CloudflareRoleMapping;
    use common_platform::PlatformTokenScopeMapping;

    platform_tokens (token) {
        token -> Text,
        scope -> PlatformTokenScopeMapping,
    }
}

diesel::table! {
    use diesel::sql_types::*;

    rejections (access_request_id) {
        access_request_id -> Text,
        user -> Text,
        timestamp -> TimestamptzSqlite,
    }
}

diesel::table! {
    use diesel::sql_types::*;

    requests_slack (access_request_id) {
        access_request_id -> Text,
        company_id -> Integer,
        channel_id -> Text,
        ts -> Text,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use common::AccessRoleMapping;

    company_roles (id) {
        id -> Integer,
        name -> AccessRoleMapping,
    }
}

diesel::table! {
    use diesel::sql_types::*;

    company_slack (company_id) {
        company_id -> Integer,
        team_id -> Text,
        team_name -> Text,
        channel_id -> Text,
        access_token -> Text,
        incoming_webhook -> Text,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use common::AccessRequestStateMapping;
    use common::CloudflareRoleMapping;

    extensions (access_request_id, user) {
        access_request_id -> Text,
        user -> Text,
        timestamp -> TimestamptzSqlite,
        duration -> Integer,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use common::AccessRequestStateMapping;
    use common::CloudflareRoleMapping;

    revocations (access_request_id) {
        access_request_id -> Text,
        user -> Text,
        timestamp -> TimestamptzSqlite,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use common::AccessRequestStateMapping;
    use common::CloudflareRoleMapping;

    user_aliases (user_id) {
        user_id -> Integer,
        aws -> Nullable<Text>,
        cloudflare -> Nullable<Text>,
        gcp -> Nullable<Text>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use common::AccessRequestStateMapping;
    use common::CloudflareRoleMapping;

    user_tokens (token) {
        token -> Text,
        user_id -> Integer,
    }
}

diesel::table! {
    use diesel::sql_types::*;

    users (id) {
        id -> Integer,
        email -> Text,
        first_name -> Text,
        last_name -> Text,
        active -> Bool,
    }
}

diesel::table! {
    use diesel::sql_types::*;

    users_companies (user_id, company_id) {
        user_id -> Integer,
        company_id -> Integer,
    }
}

diesel::table! {
    use diesel::sql_types::*;

    users_companies_roles (user_id, company_id, role_id) {
        user_id -> Integer,
        company_id -> Integer,
        role_id -> Integer,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use common::AccessRequestStateMapping;
    use common::CloudflareRoleMapping;

    worker_keys (company_id) {
        company_id -> Integer,
        key -> Text,
    }
}

diesel::joinable!(api_tokens -> companies (company_id));
diesel::joinable!(approvals -> access_requests (access_request_id));
diesel::joinable!(aws_requests -> access_requests (access_request_id));
diesel::joinable!(cancellations -> access_requests (access_request_id));
diesel::joinable!(cloudflare_requests -> access_requests (access_request_id));
diesel::joinable!(company_aws_accounts -> companies (company_id));
diesel::joinable!(company_cloudflare_accounts -> companies (company_id));
diesel::joinable!(company_gcp_projects -> companies (company_id));
diesel::joinable!(company_policies -> companies (company_id));
diesel::joinable!(company_slack -> companies (company_id));
diesel::joinable!(extensions -> access_requests (access_request_id));
diesel::joinable!(gcp_requests -> access_requests (access_request_id));
diesel::joinable!(justifications -> access_requests (access_request_id));
diesel::joinable!(rejections -> access_requests (access_request_id));
diesel::joinable!(requests_slack -> access_requests (access_request_id));
diesel::joinable!(requests_slack -> companies (company_id));
diesel::joinable!(revocations -> access_requests (access_request_id));
diesel::joinable!(user_aliases -> users (user_id));
diesel::joinable!(user_tokens -> users (user_id));
diesel::joinable!(users_companies -> companies (company_id));
diesel::joinable!(users_companies -> users (user_id));
diesel::joinable!(users_companies_roles -> companies (company_id));
diesel::joinable!(users_companies_roles -> company_roles (role_id));
diesel::joinable!(users_companies_roles -> users (user_id));
diesel::joinable!(worker_keys -> companies (company_id));

diesel::allow_tables_to_appear_in_same_query!(
    access_requests,
    api_tokens,
    approvals,
    aws_requests,
    cancellations,
    cloudflare_requests,
    companies,
    company_aws_accounts,
    company_cloudflare_accounts,
    company_gcp_projects,
    company_policies,
    company_roles,
    company_slack,
    extensions,
    gcp_requests,
    justifications,
    rejections,
    requests_slack,
    revocations,
    user_aliases,
    user_tokens,
    users,
    users_companies,
    users_companies_roles,
    worker_keys,
);
