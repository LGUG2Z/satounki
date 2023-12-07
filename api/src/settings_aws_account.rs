use actix_web::delete;
use actix_web::get;
use actix_web::http::StatusCode;
use actix_web::post;
use actix_web::put;
use actix_web::web;
use actix_web::HttpResponse;
use common::AwsAccount;
use common::SettingsAwsAccountGetResponse;
use common::SettingsAwsAccountPostRequest;
use common::SettingsAwsAccountPostResponse;
use common::SettingsAwsAccountPutRequest;
use common::SettingsAwsAccountPutResponse;
use database::CompanyAwsAccount;
use database::Pool;
use uuid::Uuid;

use crate::auth::AdministratorRole;
use crate::auth::ApiTokenOrUserWithAccessRole;
use crate::error;
use crate::public_doc::ex;
use crate::Result;

/// Register an AWS account
#[utoipa::path(
    context_path = "/v1/settings",
    tag = "settings",
    security(("user_token" = []), ("api_token" = [])),
    request_body = SettingsAwsAccountPostRequest,
    responses(
        (status = 200, body = SettingsAwsAccountPostResponse),
        (status = 401, body = ErrorResponse, example = json!(ex(StatusCode::UNAUTHORIZED))),
        (status = 404, body = ErrorResponse, example = json!(ex(StatusCode::NOT_FOUND))),
        (status = 500, body = ErrorResponse, example = json!(ex(StatusCode::INTERNAL_SERVER_ERROR))),
    )
)]
#[post("/aws-account")]
async fn settings_aws_account_post(
    pool: web::Data<Pool>,
    authenticated: ApiTokenOrUserWithAccessRole<AdministratorRole>,
    body: web::Json<SettingsAwsAccountPostRequest>,
) -> Result<web::Json<SettingsAwsAccountPostResponse>> {
    let payload = body.into_inner();

    let connection = &mut *pool.get()?;
    let company = authenticated.information().company(connection)?;

    let company_id = company.id;

    let account = CompanyAwsAccount::create(
        connection,
        CompanyAwsAccount {
            id: Uuid::new_v4().to_string(),
            aws_account_alias: payload.account.clone(),
            approvals_required: payload.approvals_required,
            company_id,
            admin_approval_required: payload.admin_approval_required,
        },
    )?;

    Ok(web::Json(SettingsAwsAccountPostResponse(AwsAccount {
        id: account.id,
        account: account.aws_account_alias,
        approvals_required: account.approvals_required,
        admin_approval_required: account.admin_approval_required,
    })))
}

/// Update an AWS account
#[utoipa::path(
    context_path = "/v1/settings",
    tag = "settings",
    security(("user_token" = []), ("api_token" = [])),
    request_body = SettingsAwsAccountPutRequest,
    responses(
        (status = 200, body = SettingsAwsAccountPutResponse),
        (status = 401, body = ErrorResponse, example = json!(ex(StatusCode::UNAUTHORIZED))),
        (status = 404, body = ErrorResponse, example = json!(ex(StatusCode::NOT_FOUND))),
        (status = 500, body = ErrorResponse, example = json!(ex(StatusCode::INTERNAL_SERVER_ERROR))),
    )
)]
#[put("/aws-account/{id}")]
async fn settings_aws_account_put(
    pool: web::Data<Pool>,
    authenticated: ApiTokenOrUserWithAccessRole<AdministratorRole>,
    body: web::Json<SettingsAwsAccountPutRequest>,
    id: web::Path<String>,
) -> Result<web::Json<SettingsAwsAccountPutResponse>> {
    let payload = body.into_inner();

    let connection = &mut *pool.get()?;
    let company_id = authenticated.information().company_id(connection)?;
    let mut project = CompanyAwsAccount::read(connection, id.as_str())?;
    if company_id != project.company_id {
        return Err(error::Api::UnauthorizedNotFound);
    }

    project.aws_account_alias = payload.account.clone();
    project.approvals_required = payload.approvals_required;
    project.admin_approval_required = payload.admin_approval_required;

    let updated = project.update(connection)?;

    Ok(web::Json(SettingsAwsAccountPutResponse(AwsAccount {
        id: updated.id,
        account: updated.aws_account_alias,
        approvals_required: updated.approvals_required,
        admin_approval_required: updated.admin_approval_required,
    })))
}

/// View an AWS account
#[utoipa::path(
    context_path = "/v1/settings",
    tag = "settings",
    security(("user_token" = []), ("api_token" = [])),
    responses(
        (status = 200, body = SettingsAwsAccountGetResponse),
        (status = 401, body = ErrorResponse, example = json!(ex(StatusCode::UNAUTHORIZED))),
        (status = 404, body = ErrorResponse, example = json!(ex(StatusCode::NOT_FOUND))),
        (status = 500, body = ErrorResponse, example = json!(ex(StatusCode::INTERNAL_SERVER_ERROR))),
    )
)]
#[get("/aws-account/{id}")]
async fn settings_aws_account_get(
    pool: web::Data<Pool>,
    authenticated: ApiTokenOrUserWithAccessRole<AdministratorRole>,
    id: web::Path<String>,
) -> Result<web::Json<SettingsAwsAccountGetResponse>> {
    let connection = &mut *pool.get()?;
    let company_id = authenticated.information().company_id(connection)?;

    let account = CompanyAwsAccount::read(connection, id.as_str())?;
    if company_id != account.company_id {
        return Err(error::Api::UnauthorizedNotFound);
    }

    Ok(web::Json(SettingsAwsAccountGetResponse(AwsAccount {
        id: account.id,
        account: account.aws_account_alias,
        approvals_required: account.approvals_required,
        admin_approval_required: account.admin_approval_required,
    })))
}

/// Delete an AWS account
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
#[delete("/aws-account/{id}")]
async fn settings_aws_account_delete(
    pool: web::Data<Pool>,
    authenticated: ApiTokenOrUserWithAccessRole<AdministratorRole>,
    id: web::Path<String>,
) -> Result<HttpResponse> {
    let connection = &mut *pool.get()?;

    let company_id = authenticated.information().company_id(connection)?;
    let account = CompanyAwsAccount::read(&mut *pool.get()?, &id)?;

    if company_id != account.company_id {
        return Err(error::Api::UnauthorizedNotFound);
    }

    account.delete(&mut *pool.get()?)?;
    Ok(HttpResponse::Ok().finish())
}
