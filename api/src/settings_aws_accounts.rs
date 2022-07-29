use actix_web::get;
use actix_web::http::StatusCode;
use actix_web::web;
use common::AwsAccount;
use common::SettingsAwsAccountsGetResponse;
use database::Pool;

use crate::auth::ApiTokenOrUserWithAccessRole;
use crate::auth::UserRole;
use crate::public_doc::ex;
use crate::Result;

/// List AWS accounts
#[utoipa::path(
    context_path = "/v1/settings",
    tag = "settings",
    security(("user_token" = []), ("api_token" = [])),
    responses(
        (status = 200, body = SettingsAwsAccountsGetResponse),
        (status = 401, body = ErrorResponse, example = json!(ex(StatusCode::UNAUTHORIZED))),
        (status = 404, body = ErrorResponse, example = json!(ex(StatusCode::NOT_FOUND))),
        (status = 500, body = ErrorResponse, example = json!(ex(StatusCode::INTERNAL_SERVER_ERROR))),
    )
)]
#[get("/aws-accounts")]
async fn settings_aws_accounts_get(
    pool: web::Data<Pool>,
    authenticated: ApiTokenOrUserWithAccessRole<UserRole>,
) -> Result<web::Json<SettingsAwsAccountsGetResponse>> {
    let connection = &mut *pool.get()?;
    let company = authenticated.information().company(connection)?;
    let results = company.aws_accounts(&mut *connection)?;

    Ok(web::Json(SettingsAwsAccountsGetResponse(
        results
            .into_iter()
            .map(|account| AwsAccount {
                id: account.id,
                account: account.aws_account_alias,
                approvals_required: account.approvals_required,
                admin_approval_required: account.admin_approval_required,
            })
            .collect(),
    )))
}
