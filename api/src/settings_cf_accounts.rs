use actix_web::get;
use actix_web::http::StatusCode;
use actix_web::web;
use common::CloudflareAccount;
use common::SettingsCloudflareAccountsGetResponse;
use database::Pool;

use crate::auth::ApiTokenOrUserWithAccessRole;
use crate::auth::UserRole;
use crate::public_doc::ex;
use crate::Result;

/// List CF accounts
#[utoipa::path(
    context_path = "/v1/settings",
    tag = "settings",
    security(("user_token" = []), ("api_token" = [])),
    responses(
        (status = 200, body = SettingsCloudflareAccountsGetResponse),
        (status = 401, body = ErrorResponse, example = json!(ex(StatusCode::UNAUTHORIZED))),
        (status = 404, body = ErrorResponse, example = json!(ex(StatusCode::NOT_FOUND))),
        (status = 500, body = ErrorResponse, example = json!(ex(StatusCode::INTERNAL_SERVER_ERROR))),
    )
)]
#[get("/cloudflare-accounts")]
async fn settings_cf_accounts_get(
    pool: web::Data<Pool>,
    authenticated: ApiTokenOrUserWithAccessRole<UserRole>,
) -> Result<web::Json<SettingsCloudflareAccountsGetResponse>> {
    let connection = &mut *pool.get()?;
    let company = authenticated.information().company(connection)?;
    let results = company.cloudflare_accounts(&mut *connection)?;

    Ok(web::Json(SettingsCloudflareAccountsGetResponse(
        results
            .into_iter()
            .map(|account| CloudflareAccount {
                id: account.id,
                account: account.cloudflare_account_alias,
                approvals_required: account.approvals_required,
                admin_approval_required: account.admin_approval_required,
            })
            .collect(),
    )))
}
