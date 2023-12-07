use actix_web::delete;
use actix_web::get;
use actix_web::http::StatusCode;
use actix_web::post;
use actix_web::put;
use actix_web::web;
use actix_web::HttpResponse;
use common::ManagedRoleValidator;
use common::NewPolicy;
use common::Policy;
use common::PolicyGetResponse;
use common::PolicyPostRequest;
use common::PolicyPostResponse;
use common::PolicyPutRequest;
use common::PolicyPutResponse;
use database::CompanyPolicy;
use database::Pool;
use uuid::Uuid;

use crate::auth::AdministratorRole;
use crate::auth::ApiTokenOrUserWithAccessRole;
use crate::auth::UserRole;
use crate::error;
use crate::public_doc::ex;
use crate::Result;
use crate::AWS_ROLES;
use crate::GCP_ROLES;

/// Create a policy
#[utoipa::path(
    context_path = "/v1/policy",
    tag = "policies",
    security(("user_token" = []), ("api_token" = [])),
    request_body = PolicyPostRequest,
    responses(
        (status = 200, body = PolicyPostResponse),
        (status = 401, body = ErrorResponse, example = json!(ex(StatusCode::UNAUTHORIZED))),
        (status = 404, body = ErrorResponse, example = json!(ex(StatusCode::NOT_FOUND))),
        (status = 422, body = ErrorResponse, example = json!(ex(StatusCode::UNPROCESSABLE_ENTITY))),
        (status = 500, body = ErrorResponse, example = json!(ex(StatusCode::INTERNAL_SERVER_ERROR))),
    )
)]
#[post("")]
async fn policy_post(
    pool: web::Data<Pool>,
    authenticated: ApiTokenOrUserWithAccessRole<AdministratorRole>,
    body: web::Json<PolicyPostRequest>,
) -> Result<web::Json<PolicyPostResponse>> {
    let authenticated = authenticated.information();

    let policy = body.into_inner();

    let invalid_roles = extract_invalid_roles(&policy);
    if !invalid_roles.is_empty() {
        return Err(error::Api::InvalidRoles(invalid_roles));
    }

    let connection = &mut *pool.get()?;

    let company = authenticated.company(connection)?;

    let stringified_policy = serde_json::to_string(&policy)?;
    let company_policy = CompanyPolicy {
        id: Uuid::new_v4().to_string(),
        company_id: company.id,
        name: policy.name.clone(),
        description: policy.description.clone(),
        policy: stringified_policy,
    };

    let policy = CompanyPolicy::create(connection, &company_policy)?;
    let partially_parsed: NewPolicy = serde_json::from_str(&policy.policy)?;

    Ok(web::Json(PolicyPostResponse(Policy {
        id: policy.id,
        name: policy.name,
        description: policy.description,
        gcp: partially_parsed.gcp,
        aws: partially_parsed.aws,
        cloudflare: partially_parsed.cloudflare,
    })))
}

/// Update a policy
#[utoipa::path(
    context_path = "/v1/policy",
    tag = "policies",
    security(("user_token" = []), ("api_token" = [])),
    request_body = PolicyPutRequest,
    responses(
        (status = 200, body = PolicyPutResponse),
        (status = 401, body = ErrorResponse, example = json!(ex(StatusCode::UNAUTHORIZED))),
        (status = 404, body = ErrorResponse, example = json!(ex(StatusCode::NOT_FOUND))),
        (status = 422, body = ErrorResponse, example = json!(ex(StatusCode::UNPROCESSABLE_ENTITY))),
        (status = 500, body = ErrorResponse, example = json!(ex(StatusCode::INTERNAL_SERVER_ERROR))),
    )
)]
#[put("/{id}")]
async fn policy_put(
    pool: web::Data<Pool>,
    authenticated: ApiTokenOrUserWithAccessRole<AdministratorRole>,
    body: web::Json<PolicyPutRequest>,
    id: web::Path<String>,
) -> Result<web::Json<PolicyPutResponse>> {
    let new_policy = body.into_inner();

    let connection = &mut *pool.get()?;

    let company = authenticated.information().company(connection)?;
    let mut policy = CompanyPolicy::read(connection, &id)?;

    if policy.company_id != company.id {
        return Err(error::Api::UnauthorizedNotFound);
    }

    let invalid_roles = extract_invalid_roles(&new_policy);
    if !invalid_roles.is_empty() {
        return Err(error::Api::InvalidRoles(invalid_roles));
    }

    policy.policy = serde_json::to_string(&new_policy)?;
    policy.name = new_policy.name.clone();
    let policy = policy.update(connection)?;
    let partially_parsed: NewPolicy = serde_json::from_str(&policy.policy)?;

    Ok(web::Json(PolicyPutResponse(Policy {
        id: policy.id,
        name: policy.name,
        description: policy.description,
        gcp: partially_parsed.gcp,
        aws: partially_parsed.aws,
        cloudflare: partially_parsed.cloudflare,
    })))
}

/// View a policy
#[utoipa::path(
    context_path = "/v1/policy",
    tag = "policies",
    security(("user_token" = []), ("api_token" = [])),
    responses(
        (status = 200, body = PolicyGetResponse),
        (status = 401, body = ErrorResponse, example = json!(ex(StatusCode::UNAUTHORIZED))),
        (status = 404, body = ErrorResponse, example = json!(ex(StatusCode::NOT_FOUND))),
        (status = 500, body = ErrorResponse, example = json!(ex(StatusCode::INTERNAL_SERVER_ERROR))),
    )
)]
#[get("/{id}")]
async fn policy_get(
    pool: web::Data<Pool>,
    authenticated: ApiTokenOrUserWithAccessRole<UserRole>,
    id: web::Path<String>,
) -> Result<web::Json<PolicyGetResponse>> {
    let connection = &mut *pool.get()?;

    let company = authenticated.information().company(connection)?;
    let policy = CompanyPolicy::read(connection, &id)?;

    if policy.company_id != company.id {
        return Err(error::Api::UnauthorizedNotFound);
    }

    let partially_parsed: NewPolicy = serde_json::from_str(&policy.policy)?;

    Ok(web::Json(PolicyGetResponse(Policy {
        id: policy.id,
        name: policy.name,
        description: policy.description,
        gcp: partially_parsed.gcp,
        aws: partially_parsed.aws,
        cloudflare: partially_parsed.cloudflare,
    })))
}

/// View a policy
#[utoipa::path(
    context_path = "/v1/policy",
    tag = "policies",
    security(("user_token" = []), ("api_token" = [])),
    responses(
        (status = 200, body = PolicyGetResponse),
        (status = 401, body = ErrorResponse, example = json!(ex(StatusCode::UNAUTHORIZED))),
        (status = 404, body = ErrorResponse, example = json!(ex(StatusCode::NOT_FOUND))),
        (status = 500, body = ErrorResponse, example = json!(ex(StatusCode::INTERNAL_SERVER_ERROR))),
    )
)]
#[get("/name/{name}")]
async fn policy_name_get(
    pool: web::Data<Pool>,
    authenticated: ApiTokenOrUserWithAccessRole<UserRole>,
    name: web::Path<String>,
) -> Result<web::Json<PolicyGetResponse>> {
    let connection = &mut *pool.get()?;
    let company_id = authenticated.information().company_id(connection)?;
    let policy = CompanyPolicy::read_by_name(connection, &name, company_id)?;

    if policy.company_id != company_id {
        return Err(error::Api::UnauthorizedNotFound);
    }

    let partially_parsed: NewPolicy = serde_json::from_str(&policy.policy)?;

    Ok(web::Json(PolicyGetResponse(Policy {
        id: policy.id,
        name: policy.name,
        description: policy.description,
        gcp: partially_parsed.gcp,
        aws: partially_parsed.aws,
        cloudflare: partially_parsed.cloudflare,
    })))
}

/// Delete a policy
#[utoipa::path(
    context_path = "/v1/policy",
    tag = "policies",
    security(("user_token" = []), ("api_token" = [])),
    responses(
        (status = 200),
        (status = 401, body = ErrorResponse, example = json!(ex(StatusCode::UNAUTHORIZED))),
        (status = 404, body = ErrorResponse, example = json!(ex(StatusCode::NOT_FOUND))),
        (status = 500, body = ErrorResponse, example = json!(ex(StatusCode::INTERNAL_SERVER_ERROR))),
    )
)]
#[delete("/{id}")]
async fn policy_delete(
    pool: web::Data<Pool>,
    authenticated: ApiTokenOrUserWithAccessRole<AdministratorRole>,
    id: web::Path<String>,
) -> Result<HttpResponse> {
    let connection = &mut *pool.get()?;
    let company_id = authenticated.information().company_id(connection)?;
    let policy = CompanyPolicy::read(connection, &id)?;

    if policy.company_id != company_id {
        return Err(error::Api::UnauthorizedNotFound);
    }

    policy.delete(connection)?;
    Ok(HttpResponse::Ok().finish())
}

fn extract_invalid_roles(policy: &NewPolicy) -> Vec<String> {
    let mut invalid_roles = vec![];

    if let Some(gcp_roles) = &policy.gcp {
        for role in gcp_roles {
            if !role.is_valid(&GCP_ROLES.iter().map(|r| r.name.clone()).collect()) {
                invalid_roles.push(role.0.clone());
            }
        }
    }

    if let Some(aws_roles) = &policy.aws {
        for role in aws_roles {
            if !role.is_valid(&AWS_ROLES.iter().map(|r| r.arn.clone()).collect()) {
                invalid_roles.push(role.0.clone());
            }
        }
    }

    invalid_roles
}
