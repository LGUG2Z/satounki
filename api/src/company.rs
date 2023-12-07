use actix_web::delete;
use actix_web::get;
use actix_web::http::StatusCode;
use actix_web::post;
use actix_web::put;
use actix_web::web;
use actix_web::HttpResponse;
use common::AccessRole;
use common_platform::CompaniesGetResponse;
use common_platform::CompanyGetResponse;
use common_platform::CompanyPostRequest;
use common_platform::CompanyPostResponse;
use common_platform::CompanyPutRequest;
use common_platform::CompanyPutResponse;
use database::Company;
use database::NewCompany;
use database::NewUser;
use database::Pool;
use database::User;

use crate::auth::PlatformTokenWithScope;
use crate::auth::Read;
use crate::auth::Write;
use crate::platform_doc::ex;
use crate::Result;

/// Create a company
#[utoipa::path(
    context_path = "/platform",
    tag = "companies",
    security(("platform_token" = ["write"])),
    request_body = CompanyPostRequest,
    responses(
        (status = 200, body = CompanyPostResponse),
        (status = 401, body = ErrorResponse, example = json!(ex(StatusCode::UNAUTHORIZED))),
        (status = 404, body = ErrorResponse, example = json!(ex(StatusCode::NOT_FOUND))),
        (status = 500, body = ErrorResponse, example = json!(ex(StatusCode::INTERNAL_SERVER_ERROR))),
    )
)]
#[post("/company")]
async fn company_post(
    pool: web::Data<Pool>,
    _platform: PlatformTokenWithScope<Write>,
    body: web::Json<CompanyPostRequest>,
) -> Result<web::Json<CompanyPostResponse>> {
    let connection = &mut *pool.get()?;

    let user: Option<User> = if let (Some(first_name), Some(last_name)) = (
        body.root_user_first_name.as_ref(),
        body.root_user_last_name.as_ref(),
    ) {
        Option::from(User::create(
            connection,
            &NewUser {
                email: body.root_user_email.clone(),
                first_name: first_name.clone(),
                last_name: last_name.clone(),
                active: true,
            },
        )?)
    } else {
        None
    };

    let company = Company::create(
        connection,
        &NewCompany {
            name: body.name.clone(),
            domain: body.domain.clone(),
            root_user: body.root_user_email.clone(),
        },
    )?;

    if let Some(user) = user {
        company.add_user(connection, &user)?;
        company.assign_role(connection, &user, &AccessRole::Administrator)?;
        company.assign_role(connection, &user, &AccessRole::Approver)?;
        company.assign_role(connection, &user, &AccessRole::User)?;
    }

    Ok(web::Json(CompanyPostResponse(
        company.to_api_response(connection)?,
    )))
}

/// View a company
#[utoipa::path(
    context_path = "/platform",
    tag = "companies",
    security(("platform_token" = ["read"])),
    responses(
        (status = 200, body = CompanyGetResponse),
        (status = 401, body = ErrorResponse, example = json!(ex(StatusCode::UNAUTHORIZED))),
        (status = 404, body = ErrorResponse, example = json!(ex(StatusCode::NOT_FOUND))),
        (status = 500, body = ErrorResponse, example = json!(ex(StatusCode::INTERNAL_SERVER_ERROR))),
    )
)]
#[get("/company/{id}")]
async fn company_get(
    pool: web::Data<Pool>,
    _platform: PlatformTokenWithScope<Read>,
    id: web::Path<i32>,
) -> Result<web::Json<CompanyGetResponse>> {
    let connection = &mut *pool.get()?;

    Ok(web::Json(CompanyGetResponse(
        Company::read(connection, *id)?.to_api_response(connection)?,
    )))
}

/// View all companies
#[utoipa::path(
    context_path = "/platform",
    tag = "companies",
    security(("platform_token" = ["read"])),
    responses(
        (status = 200, body = CompaniesGetResponse),
        (status = 401, body = ErrorResponse, example = json!(ex(StatusCode::UNAUTHORIZED))),
        (status = 404, body = ErrorResponse, example = json!(ex(StatusCode::NOT_FOUND))),
        (status = 500, body = ErrorResponse, example = json!(ex(StatusCode::INTERNAL_SERVER_ERROR))),
    )
)]
#[get("/companies")]
async fn companies_get(
    pool: web::Data<Pool>,
    _platform: PlatformTokenWithScope<Read>,
) -> Result<web::Json<CompaniesGetResponse>> {
    let connection = &mut *pool.get()?;

    Ok(web::Json(CompaniesGetResponse(
        Company::all(connection)?
            .into_iter()
            .map(Into::into)
            .collect(),
    )))
}

/// Edit a company
#[utoipa::path(
    context_path = "/platform",
    tag = "companies",
    security(("platform_token" = ["write"])),
    request_body = CompanyPutRequest,
    responses(
        (status = 200, body = CompanyPutResponse),
        (status = 401, body = ErrorResponse, example = json!(ex(StatusCode::UNAUTHORIZED))),
        (status = 404, body = ErrorResponse, example = json!(ex(StatusCode::NOT_FOUND))),
        (status = 500, body = ErrorResponse, example = json!(ex(StatusCode::INTERNAL_SERVER_ERROR))),
    )
)]
#[put("/company/{id}")]
async fn company_put(
    pool: web::Data<Pool>,
    _platform: PlatformTokenWithScope<Write>,
    body: web::Json<CompanyPutRequest>,
    id: web::Path<i32>,
) -> Result<web::Json<CompanyPutResponse>> {
    let connection = &mut *pool.get()?;

    let mut company = Company::read(connection, *id)?;

    company.name = body.name.clone();
    company.domain = body.domain.clone();

    Ok(web::Json(CompanyPutResponse(
        company.update(connection)?.to_api_response(connection)?,
    )))
}

/// Delete a company
#[utoipa::path(
    context_path = "/platform",
    tag = "companies",
    security(("platform_token" = ["write"])),
    responses(
        (status = 200, body = CompanyGetResponse),
        (status = 401, body = ErrorResponse, example = json!(ex(StatusCode::UNAUTHORIZED))),
        (status = 404, body = ErrorResponse, example = json!(ex(StatusCode::NOT_FOUND))),
        (status = 500, body = ErrorResponse, example = json!(ex(StatusCode::INTERNAL_SERVER_ERROR))),
    )
)]
#[delete("/company/{id}")]
async fn company_delete(
    pool: web::Data<Pool>,
    _platform: PlatformTokenWithScope<Read>,
    id: web::Path<i32>,
) -> Result<HttpResponse> {
    let connection = &mut *pool.get()?;

    Company::delete(connection, *id)?;

    Ok(HttpResponse::Ok().finish())
}
