use actix_web::delete;
use actix_web::get;
use actix_web::http::StatusCode;
use actix_web::post;
use actix_web::put;
use actix_web::web;
use actix_web::HttpResponse;
use common::CloudflareAccount;
use common::SettingsCloudflareAccountGetResponse;
use common::SettingsCloudflareAccountPostRequest;
use common::SettingsCloudflareAccountPostResponse;
use common::SettingsCloudflareAccountPutRequest;
use common::SettingsCloudflareAccountPutResponse;
use database::CompanyCloudflareAccount;
use database::Pool;
use uuid::Uuid;

use crate::auth::AdministratorRole;
use crate::auth::ApiTokenOrUserWithAccessRole;
use crate::error;
use crate::public_doc::ex;
use crate::Result;

/// Register a Cloudflare account
#[utoipa::path(
    context_path = "/v1/settings",
    tag = "settings",
    security(("user_token" = []), ("api_token" = [])),
    request_body = SettingsCloudflareAccountPostRequest,
    responses(
        (status = 200, body = SettingsCloudflareAccountPostResponse),
        (status = 401, body = ErrorResponse, example = json!(ex(StatusCode::UNAUTHORIZED))),
        (status = 404, body = ErrorResponse, example = json!(ex(StatusCode::NOT_FOUND))),
        (status = 500, body = ErrorResponse, example = json!(ex(StatusCode::INTERNAL_SERVER_ERROR))),
    )
)]
#[post("/cloudflare-account")]
async fn settings_cf_account_post(
    pool: web::Data<Pool>,
    authenticated: ApiTokenOrUserWithAccessRole<AdministratorRole>,
    body: web::Json<SettingsCloudflareAccountPostRequest>,
) -> Result<web::Json<SettingsCloudflareAccountPostResponse>> {
    let payload = body.into_inner();

    let connection = &mut *pool.get()?;
    let company = authenticated.information().company(connection)?;

    let company_id = company.id;

    let account = CompanyCloudflareAccount::create(
        connection,
        CompanyCloudflareAccount {
            id: Uuid::new_v4().to_string(),
            cloudflare_account_alias: payload.account.clone(),
            approvals_required: payload.approvals_required,
            company_id,
            admin_approval_required: payload.admin_approval_required,
        },
    )?;

    Ok(web::Json(SettingsCloudflareAccountPostResponse(
        CloudflareAccount {
            id: account.id,
            account: account.cloudflare_account_alias,
            approvals_required: account.approvals_required,
            admin_approval_required: account.admin_approval_required,
        },
    )))
}

/// Update a Cloudflare account
#[utoipa::path(
    context_path = "/v1/settings",
    tag = "settings",
    security(("user_token" = []), ("api_token" = [])),
    request_body = SettingsCloudflareAccountPutRequest,
    responses(
        (status = 200, body = SettingsCloudflareAccountPutResponse),
        (status = 401, body = ErrorResponse, example = json!(ex(StatusCode::UNAUTHORIZED))),
        (status = 404, body = ErrorResponse, example = json!(ex(StatusCode::NOT_FOUND))),
        (status = 500, body = ErrorResponse, example = json!(ex(StatusCode::INTERNAL_SERVER_ERROR))),
    )
)]
#[put("/cloudflare-account/{id}")]
async fn settings_cf_account_put(
    pool: web::Data<Pool>,
    authenticated: ApiTokenOrUserWithAccessRole<AdministratorRole>,
    body: web::Json<SettingsCloudflareAccountPutRequest>,
    id: web::Path<String>,
) -> Result<web::Json<SettingsCloudflareAccountPutResponse>> {
    let payload = body.into_inner();

    let connection = &mut *pool.get()?;
    let company_id = authenticated.information().company_id(connection)?;
    let mut project = CompanyCloudflareAccount::read(connection, id.as_str())?;
    if company_id != project.company_id {
        return Err(error::Api::UnauthorizedNotFound);
    }

    project.cloudflare_account_alias = payload.account.clone();
    project.approvals_required = payload.approvals_required;
    project.admin_approval_required = payload.admin_approval_required;

    let updated = project.update(connection)?;

    Ok(web::Json(SettingsCloudflareAccountPutResponse(
        CloudflareAccount {
            id: updated.id,
            account: updated.cloudflare_account_alias,
            approvals_required: updated.approvals_required,
            admin_approval_required: updated.admin_approval_required,
        },
    )))
}

/// View a Cloudflare account
#[utoipa::path(
    context_path = "/v1/settings",
    tag = "settings",
    security(("user_token" = []), ("api_token" = [])),
    responses(
        (status = 200, body = SettingsCloudflareAccountGetResponse),
        (status = 401, body = ErrorResponse, example = json!(ex(StatusCode::UNAUTHORIZED))),
        (status = 404, body = ErrorResponse, example = json!(ex(StatusCode::NOT_FOUND))),
        (status = 500, body = ErrorResponse, example = json!(ex(StatusCode::INTERNAL_SERVER_ERROR))),
    )
)]
#[get("/cloudflare-account/{id}")]
async fn settings_cf_account_get(
    pool: web::Data<Pool>,
    authenticated: ApiTokenOrUserWithAccessRole<AdministratorRole>,
    id: web::Path<String>,
) -> Result<web::Json<SettingsCloudflareAccountGetResponse>> {
    let connection = &mut *pool.get()?;
    let company_id = authenticated.information().company_id(connection)?;

    let account = CompanyCloudflareAccount::read(connection, id.as_str())?;
    if company_id != account.company_id {
        return Err(error::Api::UnauthorizedNotFound);
    }

    Ok(web::Json(SettingsCloudflareAccountGetResponse(
        CloudflareAccount {
            id: account.id,
            account: account.cloudflare_account_alias,
            approvals_required: account.approvals_required,
            admin_approval_required: account.admin_approval_required,
        },
    )))
}

/// Delete a Cloudflare account
#[utoipa::path(
    context_path = "/v1/settings",
    tag = "settings",
    security(("user_token" = []), ("api_token" = [])),
    responses(
        (status = 200),
        (status = 401, body = ErrorResponse, example = json!(ex(StatusCode::UNAUTHORIZED))),
        (status = 404, body = ErrorResponse, example = json!(ex(StatusCode::NOT_FOUND))),
        (status = 500, body = ErrorResponse, example = json!(ex(StatusCode::INTERNAL_SERVER_ERROR))),
    )
)]
#[delete("/cloudflare-account/{id}")]
async fn settings_cf_account_delete(
    pool: web::Data<Pool>,
    authenticated: ApiTokenOrUserWithAccessRole<AdministratorRole>,
    id: web::Path<String>,
) -> Result<HttpResponse> {
    let connection = &mut *pool.get()?;

    let company_id = authenticated.information().company_id(connection)?;
    let account = CompanyCloudflareAccount::read(&mut *pool.get()?, &id)?;

    if company_id != account.company_id {
        return Err(error::Api::UnauthorizedNotFound);
    }

    account.delete(&mut *pool.get()?)?;
    Ok(HttpResponse::Ok().finish())
}
