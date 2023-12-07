use actix_web::delete;
use actix_web::get;
use actix_web::http::StatusCode;
use actix_web::patch;
use actix_web::post;
use actix_web::put;
use actix_web::web;
use actix_web::HttpResponse;
use common::UserAliases;
use common::UserAliasesGetResponse;
use common::UserAliasesPostRequest;
use common::UserAliasesPostResponse;
use common::UserAliasesPutRequest;
use common::UserAliasesPutResponse;
use common::UserRolesGetResponse;
use common::UserRolesPostRequest;
use common::UserRolesPostResponse;
use common::UserRolesPutRequest;
use common::UserRolesPutResponse;
use common::UserStatus;
use common::UserStatusGetResponse;
use database::Pool;
use database::User;
use database::UserAlias;

use crate::auth::AdministratorRole;
use crate::auth::ApiTokenOrUserWithAccessRole;
use crate::auth::UserRole;
use crate::error;
use crate::public_doc::ex;
use crate::Result;

/// Disable a user
///
/// This request is treated as a non-operation if the user is already disabled.
///
/// All roles will be removed from the disabled user.
#[utoipa::path(
    context_path = "/v1/user",
    tag = "users",
    security(("user_token" = []), ("api_token" = [])),
    responses(
        (status = 200),
        (status = 401, body = ErrorResponse, example = json!(ex(StatusCode::UNAUTHORIZED))),
        (status = 404, body = ErrorResponse, example = json!(ex(StatusCode::NOT_FOUND))),
        (status = 500, body = ErrorResponse, example = json!(ex(StatusCode::INTERNAL_SERVER_ERROR))),
    )
)]
#[patch("/{email}/disable")]
async fn user_disable_patch(
    pool: web::Data<Pool>,
    authenticated: ApiTokenOrUserWithAccessRole<AdministratorRole>,
    email: web::Path<String>,
) -> Result<HttpResponse> {
    let connection = &mut *pool.get()?;

    let company = authenticated.information().company(connection)?;
    let mut user = User::read_by_email(connection, &email)?;

    if !user.belongs_to_company(connection, company.id)? {
        return Err(error::Api::UnauthorizedNotFound);
    }

    if user.active {
        user.active = false;
        user.update(connection)?;
        // Remove all roles from this user
        company.update_roles(connection, &user, &vec![])?;
    }

    Ok(HttpResponse::Ok().finish())
}

/// Enable a user
///
/// This request is treated as a non-operation if the user is already enabled.
///
/// Any desired roles for this user should be set after this request succeeds
#[utoipa::path(
    context_path = "/v1/user",
    tag = "users",
    security(("user_token" = []), ("api_token" = [])),
    responses(
        (status = 200),
        (status = 401, body = ErrorResponse, example = json!(ex(StatusCode::UNAUTHORIZED))),
        (status = 404, body = ErrorResponse, example = json!(ex(StatusCode::NOT_FOUND))),
        (status = 500, body = ErrorResponse, example = json!(ex(StatusCode::INTERNAL_SERVER_ERROR))),
    )
)]
#[patch("/{email}/enable")]
async fn user_enable_patch(
    pool: web::Data<Pool>,
    authenticated: ApiTokenOrUserWithAccessRole<AdministratorRole>,
    email: web::Path<String>,
) -> Result<HttpResponse> {
    let connection = &mut *pool.get()?;

    let company = authenticated.information().company(connection)?;
    let mut user = User::read_by_email(connection, &email)?;

    if !user.belongs_to_company(connection, company.id)? {
        return Err(error::Api::UnauthorizedNotFound);
    }

    if !user.active {
        user.active = true;
        user.update(connection)?;
    }

    Ok(HttpResponse::Ok().finish())
}

/// Get the status of a user
#[utoipa::path(
    context_path = "/v1/user",
    tag = "users",
    security(("user_token" = []), ("api_token" = [])),
    responses(
        (status = 200, body = UserStatusGetResponse),
        (status = 401, body = ErrorResponse, example = json!(ex(StatusCode::UNAUTHORIZED))),
        (status = 404, body = ErrorResponse, example = json!(ex(StatusCode::NOT_FOUND))),
        (status = 500, body = ErrorResponse, example = json!(ex(StatusCode::INTERNAL_SERVER_ERROR))),
    )
)]
#[get("/{email}/status")]
async fn user_status_get(
    pool: web::Data<Pool>,
    authenticated: ApiTokenOrUserWithAccessRole<UserRole>,
    email: web::Path<String>,
) -> Result<web::Json<UserStatusGetResponse>> {
    let connection = &mut *pool.get()?;

    let company = authenticated.information().company(connection)?;
    let user = User::read_by_email(connection, &email)?;

    if !user.belongs_to_company(connection, company.id)? {
        return Err(error::Api::UnauthorizedNotFound);
    }

    let status = if user.active {
        UserStatus::Enabled
    } else {
        UserStatus::Disabled
    };

    Ok(web::Json(UserStatusGetResponse(status)))
}

/// Set roles for a user
///
/// A full set of roles should be given.
///
/// Any previously set roles will be removed if not included in the request body.
///
/// Pass an empty list to remove all roles from a user.
#[utoipa::path(
    context_path = "/v1/user",
    tag = "users",
    security(("user_token" = []), ("api_token" = [])),
    request_body = UserRolesPostRequest,
    responses(
        (status = 200, body = UserRolesPostResponse),
        (status = 401, body = ErrorResponse, example = json!(ex(StatusCode::UNAUTHORIZED))),
        (status = 404, body = ErrorResponse, example = json!(ex(StatusCode::NOT_FOUND))),
        (status = 500, body = ErrorResponse, example = json!(ex(StatusCode::INTERNAL_SERVER_ERROR))),
    )
)]
#[post("/{email}/roles")]
async fn user_roles_post(
    pool: web::Data<Pool>,
    authenticated: ApiTokenOrUserWithAccessRole<AdministratorRole>,
    body: web::Json<UserRolesPostRequest>,
    email: web::Path<String>,
) -> Result<web::Json<UserRolesPostResponse>> {
    let connection = &mut *pool.get()?;

    let company = authenticated.information().company(connection)?;
    let user = User::read_by_email(connection, &email)?;

    if !user.belongs_to_company(connection, company.id)? {
        return Err(error::Api::UnauthorizedNotFound);
    }

    let roles = company.update_roles(connection, &user, &*body)?;

    Ok(web::Json(UserRolesPostResponse(roles)))
}

/// Replace roles for a user
///
/// A full set of roles should be given.
///
/// Pass an empty list to remove all roles from a user.
#[utoipa::path(
    context_path = "/v1/user",
    tag = "users",
    security(("user_token" = []), ("api_token" = [])),
    request_body = UserRolesPutRequest,
    responses(
        (status = 200, body = UserRolesPutResponse),
        (status = 401, body = ErrorResponse, example = json!(ex(StatusCode::UNAUTHORIZED))),
        (status = 404, body = ErrorResponse, example = json!(ex(StatusCode::NOT_FOUND))),
        (status = 500, body = ErrorResponse, example = json!(ex(StatusCode::INTERNAL_SERVER_ERROR))),
    )
)]
#[put("/{email}/roles")]
async fn user_roles_put(
    pool: web::Data<Pool>,
    authenticated: ApiTokenOrUserWithAccessRole<AdministratorRole>,
    body: web::Json<UserRolesPutRequest>,
    email: web::Path<String>,
) -> Result<web::Json<UserRolesPutResponse>> {
    let connection = &mut *pool.get()?;

    let company = authenticated.information().company(connection)?;
    let user = User::read_by_email(connection, &email)?;

    if !user.belongs_to_company(connection, company.id)? {
        return Err(error::Api::UnauthorizedNotFound);
    }

    let roles = company.update_roles(connection, &user, &*body)?;

    Ok(web::Json(UserRolesPutResponse(roles)))
}

/// View roles for a user
#[utoipa::path(
    context_path = "/v1/user",
    tag = "users",
    security(("user_token" = []), ("api_token" = [])),
    responses(
        (status = 200, body = UserRolesGetResponse),
        (status = 401, body = ErrorResponse, example = json!(ex(StatusCode::UNAUTHORIZED))),
        (status = 404, body = ErrorResponse, example = json!(ex(StatusCode::NOT_FOUND))),
        (status = 500, body = ErrorResponse, example = json!(ex(StatusCode::INTERNAL_SERVER_ERROR))),
    )
)]
#[get("/{email}/roles")]
async fn user_roles_get(
    pool: web::Data<Pool>,
    authenticated: ApiTokenOrUserWithAccessRole<UserRole>,
    email: web::Path<String>,
) -> Result<web::Json<UserRolesGetResponse>> {
    let connection = &mut *pool.get()?;

    let company_id = authenticated.information().company_id(connection)?;
    let user = User::read_by_email(connection, &email)?;

    if !user.belongs_to_company(connection, company_id)? {
        return Err(error::Api::UnauthorizedNotFound);
    }

    let roles = user.roles(connection)?;

    Ok(web::Json(UserRolesGetResponse(roles)))
}

/// Set service-specific aliases for a user
///
/// A full set of aliases should be given.
///
/// Any previously set aliases will be removed if not included in the request body.
#[utoipa::path(
    context_path = "/v1/user",
    tag = "users",
    security(("user_token" = []), ("api_token" = [])),
    request_body = UserAliasesPostRequest,
    responses(
        (status = 200, body = UserAliasesPostResponse),
        (status = 401, body = ErrorResponse, example = json!(ex(StatusCode::UNAUTHORIZED))),
        (status = 404, body = ErrorResponse, example = json!(ex(StatusCode::NOT_FOUND))),
        (status = 500, body = ErrorResponse, example = json!(ex(StatusCode::INTERNAL_SERVER_ERROR))),
    )
)]
#[post("/{email}/aliases")]
async fn user_aliases_post(
    pool: web::Data<Pool>,
    authenticated: ApiTokenOrUserWithAccessRole<AdministratorRole>,
    body: web::Json<UserAliasesPostRequest>,
    email: web::Path<String>,
) -> Result<web::Json<UserAliasesPostResponse>> {
    let connection = &mut *pool.get()?;

    let company_id = authenticated.information().company_id(connection)?;
    let user = User::read_by_email(connection, &email)?;

    if !user.belongs_to_company(connection, company_id)? {
        return Err(error::Api::UnauthorizedNotFound);
    }

    let response = if let Some(mut alias) = UserAlias::read_optional(connection, user.id)? {
        alias.aws = body.aws.clone();
        alias.cloudflare = body.cloudflare.clone();
        alias.gcp = body.gcp.clone();
        alias.update(&mut *pool.get()?)?
    } else {
        UserAlias::create(
            connection,
            &UserAlias {
                user_id: user.id,
                aws: body.aws.clone(),
                cloudflare: body.cloudflare.clone(),
                gcp: body.gcp.clone(),
            },
        )?
    };

    Ok(web::Json(UserAliasesPostResponse(UserAliases {
        aws: response.aws,
        cloudflare: response.cloudflare,
        gcp: response.gcp,
    })))
}

/// Replace service-specific aliases for a user
///
/// A full set of aliases should be given.
#[utoipa::path(
    context_path = "/v1/user",
    tag = "users",
    security(("user_token" = []), ("api_token" = [])),
    request_body = UserAliasesPutRequest,
    responses(
        (status = 200, body = UserAliasesPutResponse),
        (status = 401, body = ErrorResponse, example = json!(ex(StatusCode::UNAUTHORIZED))),
        (status = 404, body = ErrorResponse, example = json!(ex(StatusCode::NOT_FOUND))),
        (status = 500, body = ErrorResponse, example = json!(ex(StatusCode::INTERNAL_SERVER_ERROR))),
    )
)]
#[put("/{email}/aliases")]
async fn user_aliases_put(
    pool: web::Data<Pool>,
    authenticated: ApiTokenOrUserWithAccessRole<AdministratorRole>,
    body: web::Json<UserAliasesPutRequest>,
    email: web::Path<String>,
) -> Result<web::Json<UserAliasesPutResponse>> {
    let connection = &mut *pool.get()?;

    let company_id = authenticated.information().company_id(connection)?;
    let user = User::read_by_email(connection, &email)?;

    if !user.belongs_to_company(connection, company_id)? {
        return Err(error::Api::UnauthorizedNotFound);
    }

    let response = if let Some(mut alias) = UserAlias::read_optional(connection, user.id)? {
        alias.aws = body.aws.clone();
        alias.cloudflare = body.cloudflare.clone();
        alias.gcp = body.gcp.clone();
        alias.update(&mut *pool.get()?)?
    } else {
        UserAlias::create(
            connection,
            &UserAlias {
                user_id: user.id,
                aws: body.aws.clone(),
                cloudflare: body.cloudflare.clone(),
                gcp: body.gcp.clone(),
            },
        )?
    };

    Ok(web::Json(UserAliasesPutResponse(UserAliases {
        aws: response.aws,
        cloudflare: response.cloudflare,
        gcp: response.gcp,
    })))
}

/// View service-specific aliases for a user
#[utoipa::path(
    context_path = "/v1/user",
    tag = "users",
    security(("user_token" = []), ("api_token" = [])),
    responses(
        (status = 200, body = UserAliasesGetResponse),
        (status = 401, body = ErrorResponse, example = json!(ex(StatusCode::UNAUTHORIZED))),
        (status = 404, body = ErrorResponse, example = json!(ex(StatusCode::NOT_FOUND))),
        (status = 500, body = ErrorResponse, example = json!(ex(StatusCode::INTERNAL_SERVER_ERROR))),
    )
)]
#[get("/{email}/aliases")]
async fn user_aliases_get(
    pool: web::Data<Pool>,
    authenticated: ApiTokenOrUserWithAccessRole<UserRole>,
    email: web::Path<String>,
) -> Result<web::Json<UserAliasesGetResponse>> {
    let connection = &mut *pool.get()?;

    let company_id = authenticated.information().company_id(connection)?;
    let user = User::read_by_email(connection, &email)?;

    if !user.belongs_to_company(connection, company_id)? {
        return Err(error::Api::UnauthorizedNotFound);
    }

    Ok(web::Json(UserAliasesGetResponse(
        user.aliases(connection).map(Into::into)?,
    )))
}

/// Delete service-specific aliases for a user
#[utoipa::path(
    context_path = "/v1/user",
    tag = "users",
    security(("user_token" = []), ("api_token" = [])),
    responses(
        (status = 200),
        (status = 401, body = ErrorResponse, example = json!(ex(StatusCode::UNAUTHORIZED))),
        (status = 404, body = ErrorResponse, example = json!(ex(StatusCode::NOT_FOUND))),
        (status = 500, body = ErrorResponse, example = json!(ex(StatusCode::INTERNAL_SERVER_ERROR))),
    )
)]
#[delete("/{email}/aliases")]
async fn user_aliases_delete(
    pool: web::Data<Pool>,
    authenticated: ApiTokenOrUserWithAccessRole<AdministratorRole>,
    email: web::Path<String>,
) -> Result<HttpResponse> {
    let connection = &mut *pool.get()?;

    let company_id = authenticated.information().company_id(connection)?;
    let user = User::read_by_email(connection, &email)?;

    if !user.belongs_to_company(&mut *pool.get()?, company_id)? {
        return Err(error::Api::UnauthorizedNotFound);
    }

    UserAlias::read(connection, user.id)?.delete(connection)?;

    Ok(HttpResponse::Ok().finish())
}
