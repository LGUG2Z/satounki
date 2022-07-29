use actix_web::get;
use actix_web::http::StatusCode;
use actix_web::put;
use actix_web::web;
use common::UserTokenGetResponse;
use common::UserTokenPutResponse;
use database::Pool;
use database::UserToken;

use crate::auth::UserRole;
use crate::auth::UserWithAccessRole;
use crate::public_doc::ex;
use crate::Result;

/// View user token
#[utoipa::path(
    context_path = "/v1/user",
    tag = "users",
    security(("user_token" = []), ("api_token" = [])),
    responses(
        (status = 200, body = UserTokenGetResponse),
        (status = 401, body = ErrorResponse, example = json!(ex(StatusCode::UNAUTHORIZED))),
        (status = 404, body = ErrorResponse, example = json!(ex(StatusCode::NOT_FOUND))),
        (status = 500, body = ErrorResponse, example = json!(ex(StatusCode::INTERNAL_SERVER_ERROR))),
    )
)]
#[get("/token")]
async fn user_token_get(
    pool: web::Data<Pool>,
    authenticated: UserWithAccessRole<UserRole>,
) -> Result<web::Json<UserTokenGetResponse>> {
    let connection = &mut *pool.get()?;
    let token = authenticated
        .user(connection)?
        .user_token(connection)?
        .token;

    Ok(web::Json(UserTokenGetResponse(common::UserToken { token })))
}

/// Regenerate user token
#[utoipa::path(
    context_path = "/v1/user",
    tag = "users",
    security(("user_token" = []), ("api_token" = [])),
    responses(
        (status = 200, body = UserTokenPutResponse),
        (status = 401, body = ErrorResponse, example = json!(ex(StatusCode::UNAUTHORIZED))),
        (status = 404, body = ErrorResponse, example = json!(ex(StatusCode::NOT_FOUND))),
        (status = 500, body = ErrorResponse, example = json!(ex(StatusCode::INTERNAL_SERVER_ERROR))),
    )
)]
#[put("/token")]
async fn user_token_put(
    pool: web::Data<Pool>,
    authenticated: UserWithAccessRole<UserRole>,
) -> Result<web::Json<UserTokenPutResponse>> {
    let connection = &mut *pool.get()?;
    let user_id = authenticated.user_id()?;
    let token = UserToken::replace(connection, user_id)?.token;

    Ok(web::Json(UserTokenPutResponse(common::UserToken { token })))
}
