use std::str::FromStr;

use actix_web::get;
use actix_web::http::StatusCode;
use actix_web::put;
use actix_web::web;
use common_platform::PlatformTokenGetResponse;
use common_platform::PlatformTokenPutResponse;
use common_platform::PlatformTokenScope;
use database::PlatformToken;
use database::Pool;

use crate::auth::PlatformTokenWithScope;
use crate::auth::Read;
use crate::auth::Write;
use crate::public_doc::ex;
use crate::Result;

/// View platform token
#[utoipa::path(
    context_path = "/platform",
    tag = "tokens",
    security(("platform_token" = [])),
    responses(
        (status = 200, body = PlatformTokenGetResponse),
        (status = 401, body = ErrorResponse, example = json!(ex(StatusCode::UNAUTHORIZED))),
        (status = 404, body = ErrorResponse, example = json!(ex(StatusCode::NOT_FOUND))),
        (status = 500, body = ErrorResponse, example = json!(ex(StatusCode::INTERNAL_SERVER_ERROR))),
    )
)]
#[get("/token/{scope}")]
async fn platform_token_get(
    pool: web::Data<Pool>,
    _platform: PlatformTokenWithScope<Read>,
    scope: web::Path<String>,
) -> Result<web::Json<PlatformTokenGetResponse>> {
    let connection = &mut *pool.get()?;
    let scope = PlatformTokenScope::from_str(&*scope)?;
    let token = PlatformToken::read_by_scope(connection, scope)?.token;

    Ok(web::Json(PlatformTokenGetResponse(
        common_platform::PlatformToken { token },
    )))
}

/// Regenerate platform token
#[utoipa::path(
    context_path = "/platform",
    tag = "tokens",
    security(("platform_token" = [])),
    responses(
        (status = 200, body = PlatformTokenPutResponse),
        (status = 401, body = ErrorResponse, example = json!(ex(StatusCode::UNAUTHORIZED))),
        (status = 404, body = ErrorResponse, example = json!(ex(StatusCode::NOT_FOUND))),
        (status = 500, body = ErrorResponse, example = json!(ex(StatusCode::INTERNAL_SERVER_ERROR))),
    )
)]
#[put("/token/{scope}")]
async fn platform_token_put(
    pool: web::Data<Pool>,
    _platform: PlatformTokenWithScope<Write>,
    scope: web::Path<String>,
) -> Result<web::Json<PlatformTokenPutResponse>> {
    let connection = &mut *pool.get()?;

    let scope = PlatformTokenScope::from_str(&*scope)?;
    let token = PlatformToken::replace(connection, scope)?.token;

    Ok(web::Json(PlatformTokenPutResponse(
        common_platform::PlatformToken { token },
    )))
}
