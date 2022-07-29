use actix_web::http::StatusCode;
// use common::*;
use common::AccessRequestState;
use common::AccessRole;
use common::ApiToken;
use common::AwsAccount;
use common::AwsPolicy;
use common::CloudflareAccount;
use common::CloudflareRole;
use common::ErrorResponse;
use common::GcpProject;
use common::GcpRole;
use common::NewAwsAccount;
use common::NewCloudflareAccount;
use common::NewGcpProject;
use common::NewPolicy;
use common::PoliciesGetResponse;
use common::Policy;
use common::PolicyGetResponse;
use common::PolicyPostBody;
use common::PolicyPostResponse;
use common::PolicyPutBody;
use common::PolicyPutResponse;
use common::PolicyRequest;
use common::PolicyRequestConfirmation;
use common::Request;
use common::RequestAliasGetResponse;
use common::RequestAliasPatchBody;
use common::RequestOperation;
use common::RequestPolicyPostBody;
use common::RequestPolicyPostResponse;
use common::RequestsGetResponse;
use common::SettingsAwsAccountGetResponse;
use common::SettingsAwsAccountPostBody;
use common::SettingsAwsAccountPostResponse;
use common::SettingsAwsAccountPutBody;
use common::SettingsAwsAccountPutResponse;
use common::SettingsAwsAccountsGetResponse;
use common::SettingsCloudflareAccountGetResponse;
use common::SettingsCloudflareAccountPostBody;
use common::SettingsCloudflareAccountPostResponse;
use common::SettingsCloudflareAccountPutBody;
use common::SettingsCloudflareAccountPutResponse;
use common::SettingsCloudflareAccountsGetResponse;
use common::SettingsGcpProjectGetResponse;
use common::SettingsGcpProjectPostBody;
use common::SettingsGcpProjectPostResponse;
use common::SettingsGcpProjectPutBody;
use common::SettingsGcpProjectPutResponse;
use common::SettingsGcpProjectsGetResponse;
use common::SettingsTokenGetResponse;
use common::SettingsTokenPutResponse;
use common::UserAliases;
use common::UserAliasesGetResponse;
use common::UserAliasesPostBody;
use common::UserAliasesPostResponse;
use common::UserAliasesPutBody;
use common::UserAliasesPutResponse;
use common::UserInteraction;
use common::UserRolesGetResponse;
use common::UserRolesPostBody;
use common::UserRolesPostResponse;
use common::UserRolesPutBody;
use common::UserRolesPutResponse;
use common::UserStatus;
use common::UserStatusGetResponse;
use common::UserToken;
use common::UserTokenGetResponse;
use common::UserTokenPutResponse;
use utoipa::openapi::security::Http;
use utoipa::openapi::security::HttpAuthScheme;
use utoipa::openapi::security::SecurityScheme;
use utoipa::Modify;
use utoipa::OpenApi;

use crate::policies;
use crate::policy;
use crate::request_alias;
use crate::request_policy;
use crate::requests;
use crate::settings_aws_account;
use crate::settings_aws_accounts;
use crate::settings_cf_account;
use crate::settings_cf_accounts;
use crate::settings_gcp_project;
use crate::settings_gcp_projects;
use crate::settings_token;
use crate::user;
use crate::user_token;

pub fn ex(code: StatusCode) -> ErrorResponse {
    ErrorResponse {
        code: code.as_u16(),
        error: code.to_string().replace(&format!("{} ", code.as_u16()), ""),
    }
}

#[derive(OpenApi)]
#[openapi(
    modifiers(&SecurityAddon),
    paths(
        policies::policies_get,
        policy::policy_delete,
        policy::policy_get,
        policy::policy_name_get,
        policy::policy_post,
        policy::policy_put,
        settings_aws_account::settings_aws_account_delete,
        settings_aws_account::settings_aws_account_get,
        settings_aws_account::settings_aws_account_post,
        settings_aws_account::settings_aws_account_put,
        settings_aws_accounts::settings_aws_accounts_get,
        settings_cf_account::settings_cf_account_delete,
        settings_cf_account::settings_cf_account_get,
        settings_cf_account::settings_cf_account_post,
        settings_cf_account::settings_cf_account_put,
        settings_cf_accounts::settings_cf_accounts_get,
        settings_gcp_project::settings_gcp_project_delete,
        settings_gcp_project::settings_gcp_project_get,
        settings_gcp_project::settings_gcp_project_post,
        settings_gcp_project::settings_gcp_project_put,
        settings_gcp_projects::settings_gcp_projects_get,
        settings_token::settings_token_get,
        settings_token::settings_token_put,
        user::user_status_get,
        user::user_enable_patch,
        user::user_disable_patch,
        user::user_aliases_get,
        user::user_aliases_delete,
        user::user_aliases_get,
        user::user_aliases_post,
        user::user_aliases_put,
        user::user_roles_get,
        user::user_roles_post,
        user::user_roles_put,
        user_token::user_token_get,
        user_token::user_token_put,
        request_alias::request_alias_get,
        request_alias::request_alias_patch,
        request_policy::request_policy_post,
        requests::requests_get,
    ),
    components(
        responses(
            ErrorResponse,
            PoliciesGetResponse,
            PolicyGetResponse,
            PolicyPostResponse,
            PolicyPutResponse,
            RequestAliasGetResponse,
            RequestPolicyPostResponse,
            RequestsGetResponse,
            SettingsAwsAccountGetResponse,
            SettingsAwsAccountPostResponse,
            SettingsAwsAccountPutResponse,
            SettingsAwsAccountsGetResponse,
            SettingsCloudflareAccountGetResponse,
            SettingsCloudflareAccountPostResponse,
            SettingsCloudflareAccountPutResponse,
            SettingsCloudflareAccountsGetResponse,
            SettingsGcpProjectGetResponse,
            SettingsGcpProjectPostResponse,
            SettingsGcpProjectPutResponse,
            SettingsGcpProjectsGetResponse,
            SettingsTokenGetResponse,
            SettingsTokenPutResponse,
            UserAliasesGetResponse,
            UserAliasesPostResponse,
            UserAliasesPutResponse,
            UserRolesGetResponse,
            UserRolesPostResponse,
            UserRolesPutResponse,
            UserStatusGetResponse,
            UserTokenGetResponse,
            UserTokenPutResponse,
        ),
        schemas(
            // these need to be duplicated here
            ErrorResponse,
            PoliciesGetResponse,
            PolicyGetResponse,
            PolicyPostResponse,
            PolicyPutResponse,
            RequestAliasGetResponse,
            RequestPolicyPostResponse,
            RequestsGetResponse,
            SettingsAwsAccountGetResponse,
            SettingsAwsAccountPostResponse,
            SettingsAwsAccountPutResponse,
            SettingsAwsAccountsGetResponse,
            SettingsCloudflareAccountGetResponse,
            SettingsCloudflareAccountPostResponse,
            SettingsCloudflareAccountPutResponse,
            SettingsCloudflareAccountsGetResponse,
            SettingsGcpProjectGetResponse,
            SettingsGcpProjectPostResponse,
            SettingsGcpProjectPutResponse,
            SettingsGcpProjectsGetResponse,
            SettingsTokenGetResponse,
            SettingsTokenPutResponse,
            UserAliasesGetResponse,
            UserAliasesPostResponse,
            UserAliasesPutResponse,
            UserRolesGetResponse,
            UserRolesPostResponse,
            UserRolesPutResponse,
            UserStatusGetResponse,
            UserTokenGetResponse,
            UserTokenPutResponse,

            AccessRequestState,
            ApiToken,
            AwsAccount,
            AwsPolicy,
            CloudflareRole,
            CloudflareAccount,
            AccessRole,
            PolicyPostBody,
            PolicyPutBody,
            SettingsAwsAccountPostBody,
            SettingsAwsAccountPutBody,
            SettingsCloudflareAccountPostBody,
            SettingsCloudflareAccountPutBody,
            SettingsGcpProjectPostBody,
            SettingsGcpProjectPutBody,
            UserAliases,
            UserAliasesPostBody,
            UserAliasesPutBody,
            UserRolesPostBody,
            UserRolesPutBody,
            UserStatus,
            GcpProject,
            GcpRole,
            NewAwsAccount,
            NewCloudflareAccount,
            NewGcpProject,
            NewPolicy,
            Policy,
            PolicyRequest,
            PolicyRequestConfirmation,
            Request,
            RequestOperation,
            RequestAliasPatchBody,
            RequestPolicyPostBody,
            UserInteraction,
            UserToken,
        )
    )
)]
pub struct PublicDoc;
pub struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.as_mut().unwrap(); // we can unwrap safely since there already is components registered.
        components.add_security_scheme(
            "api_token",
            SecurityScheme::Http(Http::new(HttpAuthScheme::Bearer)),
        );

        components.add_security_scheme(
            "user_token",
            SecurityScheme::Http(Http::new(HttpAuthScheme::Bearer)),
        );

        openapi.info.title = String::from("Satounki Public API");
        openapi.info.description = Option::from(String::from(
            "Used for managing your company's Satounki account via Terraform and other tools, and by the `satounki` CLI for making and approving requests",
        ));
    }
}
