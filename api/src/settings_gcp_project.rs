use actix_web::delete;
use actix_web::get;
use actix_web::http::StatusCode;
use actix_web::post;
use actix_web::put;
use actix_web::web;
use actix_web::HttpResponse;
use common::GcpProject;
use common::SettingsGcpProjectGetResponse;
use common::SettingsGcpProjectPostBody;
use common::SettingsGcpProjectPostResponse;
use common::SettingsGcpProjectPutBody;
use common::SettingsGcpProjectPutResponse;
use database::CompanyGcpProject;
use database::Pool;
use uuid::Uuid;

use crate::auth::AdministratorRole;
use crate::auth::ApiTokenOrUserWithAccessRole;
use crate::auth::UserRole;
use crate::error;
use crate::public_doc::ex;
use crate::Result;

/// Register a GCP project
#[utoipa::path(
    context_path = "/v1/settings",
    tag = "settings",
    security(("user_token" = []), ("api_token" = [])),
    request_body = SettingsGcpProjectPostBody,
    responses(
        (status = 200, body = SettingsGcpProjectPostResponse),
        (status = 401, body = ErrorResponse, example = json!(ex(StatusCode::UNAUTHORIZED))),
        (status = 404, body = ErrorResponse, example = json!(ex(StatusCode::NOT_FOUND))),
        (status = 500, body = ErrorResponse, example = json!(ex(StatusCode::INTERNAL_SERVER_ERROR))),
    )
)]
#[post("/gcp-project")]
async fn settings_gcp_project_post(
    pool: web::Data<Pool>,
    authenticated: ApiTokenOrUserWithAccessRole<AdministratorRole>,
    body: web::Json<SettingsGcpProjectPostBody>,
) -> Result<web::Json<SettingsGcpProjectPostResponse>> {
    let payload = body.into_inner();

    let connection = &mut *pool.get()?;
    let company = authenticated.information().company(connection)?;

    let company_id = company.id;

    let project = CompanyGcpProject::create(
        connection,
        CompanyGcpProject {
            id: Uuid::new_v4().to_string(),
            gcp_project: payload.project.clone(),
            approvals_required: payload.approvals_required,
            company_id,
            admin_approval_required: payload.admin_approval_required,
        },
    )?;

    Ok(web::Json(SettingsGcpProjectPostResponse(GcpProject {
        id: project.id,
        project: project.gcp_project,
        approvals_required: project.approvals_required,
        admin_approval_required: project.admin_approval_required,
    })))
}

/// Update a GCP project
#[utoipa::path(
    context_path = "/v1/settings",
    tag = "settings",
    security(("user_token" = []), ("api_token" = [])),
    request_body = SettingsGcpProjectPutBody,
    responses(
        (status = 200, body = SettingsGcpProjectPutResponse),
        (status = 401, body = ErrorResponse, example = json!(ex(StatusCode::UNAUTHORIZED))),
        (status = 404, body = ErrorResponse, example = json!(ex(StatusCode::NOT_FOUND))),
        (status = 500, body = ErrorResponse, example = json!(ex(StatusCode::INTERNAL_SERVER_ERROR))),
    )
)]
#[put("/gcp-project/{id}")]
async fn settings_gcp_project_put(
    pool: web::Data<Pool>,
    authenticated: ApiTokenOrUserWithAccessRole<AdministratorRole>,
    body: web::Json<SettingsGcpProjectPutBody>,
    id: web::Path<String>,
) -> Result<web::Json<SettingsGcpProjectPutResponse>> {
    let payload = body.into_inner();

    let connection = &mut *pool.get()?;
    let company_id = authenticated.information().company_id(connection)?;
    let mut project = CompanyGcpProject::read(connection, id.as_str())?;
    if company_id != project.company_id {
        return Err(error::Api::UnauthorizedNotFound);
    }

    project.gcp_project = payload.project.clone();
    project.approvals_required = payload.approvals_required;
    project.admin_approval_required = payload.admin_approval_required;

    let updated = project.update(connection)?;

    Ok(web::Json(SettingsGcpProjectPutResponse(GcpProject {
        id: updated.id,
        project: updated.gcp_project,
        approvals_required: updated.approvals_required,
        admin_approval_required: updated.admin_approval_required,
    })))
}

/// View a GCP project
#[utoipa::path(
    context_path = "/v1/settings",
    tag = "settings",
    security(("user_token" = []), ("api_token" = [])),
    responses(
        (status = 200, body = SettingsGcpProjectGetResponse),
        (status = 401, body = ErrorResponse, example = json!(ex(StatusCode::UNAUTHORIZED))),
        (status = 404, body = ErrorResponse, example = json!(ex(StatusCode::NOT_FOUND))),
        (status = 500, body = ErrorResponse, example = json!(ex(StatusCode::INTERNAL_SERVER_ERROR))),
    )
)]
#[get("/gcp-project/{id}")]
async fn settings_gcp_project_get(
    pool: web::Data<Pool>,
    authenticated: ApiTokenOrUserWithAccessRole<UserRole>,
    id: web::Path<String>,
) -> Result<web::Json<SettingsGcpProjectGetResponse>> {
    let connection = &mut *pool.get()?;
    let company_id = authenticated.information().company_id(connection)?;

    let project = CompanyGcpProject::read(connection, id.as_str())?;
    if company_id != project.company_id {
        return Err(error::Api::UnauthorizedNotFound);
    }

    Ok(web::Json(SettingsGcpProjectGetResponse(GcpProject {
        id: project.id,
        project: project.gcp_project,
        approvals_required: project.approvals_required,
        admin_approval_required: project.admin_approval_required,
    })))
}

/// Delete a GCP project
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
#[delete("/gcp-project/{id}")]
async fn settings_gcp_project_delete(
    pool: web::Data<Pool>,
    authenticated: ApiTokenOrUserWithAccessRole<AdministratorRole>,
    id: web::Path<String>,
) -> Result<HttpResponse> {
    let connection = &mut *pool.get()?;

    let company_id = authenticated.information().company_id(connection)?;
    let project = CompanyGcpProject::read(connection, &id)?;

    if company_id != project.company_id {
        return Err(error::Api::UnauthorizedNotFound);
    }

    project.delete(&mut *pool.get()?)?;
    Ok(HttpResponse::Ok().finish())
}
