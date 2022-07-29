use actix_web::get;
use actix_web::http::StatusCode;
use actix_web::web;
use common::GcpProject;
use common::SettingsGcpProjectsGetResponse;
use database::Pool;

use crate::auth::ApiTokenOrUserWithAccessRole;
use crate::auth::UserRole;
use crate::public_doc::ex;
use crate::Result;

/// List GCP projects
#[utoipa::path(
    context_path = "/v1/settings",
    tag = "settings",
    security(("user_token" = []), ("api_token" = [])),
    responses(
        (status = 200, body = SettingsGcpProjectsGetResponse),
        (status = 401, body = ErrorResponse, example = json!(ex(StatusCode::UNAUTHORIZED))),
        (status = 404, body = ErrorResponse, example = json!(ex(StatusCode::NOT_FOUND))),
        (status = 500, body = ErrorResponse, example = json!(ex(StatusCode::INTERNAL_SERVER_ERROR))),
    )
)]
#[get("/gcp-projects")]
async fn settings_gcp_projects_get(
    pool: web::Data<Pool>,
    authenticated: ApiTokenOrUserWithAccessRole<UserRole>,
) -> Result<web::Json<SettingsGcpProjectsGetResponse>> {
    let connection = &mut *pool.get()?;
    let company = authenticated.information().company(connection)?;
    let results = company.gcp_projects(connection)?;

    Ok(web::Json(SettingsGcpProjectsGetResponse(
        results
            .into_iter()
            .map(|project| GcpProject {
                id: project.id,
                project: project.gcp_project,
                approvals_required: project.approvals_required,
                admin_approval_required: project.admin_approval_required,
            })
            .collect(),
    )))
}
