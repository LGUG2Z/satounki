use actix_web::get;
use actix_web::http::StatusCode;
use actix_web::web;
use common::RequestsGetQueryParams;
use common::RequestsGetResponse;
use database::Pool;
use database::RequestWrapper;

use crate::auth::ApiTokenOrUserWithAccessRole;
use crate::auth::UserRole;
use crate::error;
use crate::public_doc::ex;
use crate::Result;

/// List requests
///
/// The state of the request must be specified.
///
/// The number of requests to return must be specified.
///
/// The maximum number of matching requests that can be returned is 20.
#[utoipa::path(
    context_path = "/v1",
    tag = "requests",
    security(("user_token" = []), ("api_token" = [])),
    responses(
        (status = 200, body = RequestsGetResponse),
        (status = 401, body = ErrorResponse, example = json!(ex(StatusCode::UNAUTHORIZED))),
        (status = 404, body = ErrorResponse, example = json!(ex(StatusCode::NOT_FOUND))),
        (status = 422, body = ErrorResponse, example = json!(ex(StatusCode::UNPROCESSABLE_ENTITY))),
        (status = 500, body = ErrorResponse, example = json!(ex(StatusCode::INTERNAL_SERVER_ERROR))),
    )
)]
#[get("/requests")]
async fn requests_get(
    pool: web::Data<Pool>,
    authenticated: ApiTokenOrUserWithAccessRole<UserRole>,
    query: web::Query<RequestsGetQueryParams>,
) -> Result<web::Json<RequestsGetResponse>> {
    const MAXIMUM_REQUEST_COUNT: i64 = 20;

    if query.count > MAXIMUM_REQUEST_COUNT {
        return Err(error::Api::MaximumRequestListExceeded(
            MAXIMUM_REQUEST_COUNT,
        ));
    }

    let connection = &mut *pool.get()?;

    let company = authenticated.information().company(connection)?;
    let ids = company.access_request_ids(connection, query.state, query.count)?;

    let mut responses = vec![];
    let requests = RequestWrapper::read_all(connection, &ids)?;
    for request in requests {
        responses.push(request);
    }

    Ok(web::Json(RequestsGetResponse(responses)))
}
