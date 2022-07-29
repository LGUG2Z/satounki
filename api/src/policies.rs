use actix_web::get;
use actix_web::http::StatusCode;
use actix_web::web;
use common::NewPolicy;
use common::PoliciesGetResponse;
use common::Policy;
use database::Pool;

use crate::auth::ApiTokenOrUserWithAccessRole;
use crate::auth::UserRole;
use crate::public_doc::ex;
use crate::Result;

/// List policies
#[utoipa::path(
    context_path = "/v1",
    tag = "policies",
    security(("user_token" = []), ("api_token" = [])),
    responses(
        (status = 200, body = PoliciesGetResponse),
        (status = 401, body = ErrorResponse, example = json!(ex(StatusCode::UNAUTHORIZED))),
        (status = 404, body = ErrorResponse, example = json!(ex(StatusCode::NOT_FOUND))),
        (status = 500, body = ErrorResponse, example = json!(ex(StatusCode::INTERNAL_SERVER_ERROR))),
    )
)]
#[get("/policies")]
async fn policies_get(
    pool: web::Data<Pool>,
    authenticated: ApiTokenOrUserWithAccessRole<UserRole>,
) -> Result<web::Json<PoliciesGetResponse>> {
    let connection = &mut *pool.get()?;

    let user = authenticated.information().user(connection)?;
    let company = user.company(connection)?;
    let policies = company.policies(connection)?;

    let mut parsed = vec![];
    for policy in policies {
        let partially_parsed: NewPolicy = serde_json::from_str(&policy.policy)?;
        parsed.push(Policy {
            id: policy.id.clone(),
            description: policy.description.clone(),
            name: policy.name.clone(),
            gcp: partially_parsed.gcp,
            aws: partially_parsed.aws,
            cloudflare: partially_parsed.cloudflare,
        });
    }

    Ok(web::Json(PoliciesGetResponse(parsed)))
}
