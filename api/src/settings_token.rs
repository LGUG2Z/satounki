use actix_web::get;
use actix_web::http::StatusCode;
use actix_web::put;
use actix_web::web;
use common::SettingsTokenGetResponse;
use common::SettingsTokenPutResponse;
use database::ApiToken;
use database::Pool;

use crate::auth::AdministratorRole;
use crate::auth::ApiTokenOrUserWithAccessRole;
use crate::public_doc::ex;
use crate::Result;

/// View API token
#[utoipa::path(
    context_path = "/v1/settings",
    tag = "settings",
    security(("user_token" = []), ("api_token" = [])),
    responses(
        (status = 200, body = SettingsTokenGetResponse),
        (status = 401, body = ErrorResponse, example = json!(ex(StatusCode::UNAUTHORIZED))),
        (status = 404, body = ErrorResponse, example = json!(ex(StatusCode::NOT_FOUND))),
        (status = 500, body = ErrorResponse, example = json!(ex(StatusCode::INTERNAL_SERVER_ERROR))),
    )
)]
#[get("/token")]
async fn settings_token_get(
    pool: web::Data<Pool>,
    authenticated: ApiTokenOrUserWithAccessRole<AdministratorRole>,
) -> Result<web::Json<SettingsTokenGetResponse>> {
    let connection = &mut *pool.get()?;
    let token = authenticated
        .information()
        .company(connection)?
        .api_token(connection)?
        .token;

    Ok(web::Json(SettingsTokenGetResponse(common::ApiToken {
        token,
    })))
}

/// Regenerate API token
#[utoipa::path(
    context_path = "/v1/settings",
    tag = "settings",
    security(("user_token" = []), ("api_token" = [])),
    responses(
        (status = 200, body = SettingsTokenPutResponse),
        (status = 401, body = ErrorResponse, example = json!(ex(StatusCode::UNAUTHORIZED))),
        (status = 404, body = ErrorResponse, example = json!(ex(StatusCode::NOT_FOUND))),
        (status = 500, body = ErrorResponse, example = json!(ex(StatusCode::INTERNAL_SERVER_ERROR))),
    )
)]
#[put("/token")]
async fn settings_token_put(
    pool: web::Data<Pool>,
    authenticated: ApiTokenOrUserWithAccessRole<AdministratorRole>,
) -> Result<web::Json<SettingsTokenPutResponse>> {
    let connection = &mut *pool.get()?;
    let company_id = authenticated.information().company_id(connection)?;
    let token = ApiToken::replace(connection, company_id)?.token;

    Ok(web::Json(SettingsTokenPutResponse(common::ApiToken {
        token,
    })))
}
