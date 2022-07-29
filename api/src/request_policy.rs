use actix_web::http::StatusCode;
use actix_web::post;
use actix_web::web;
use common::NewPolicy;
use common::PolicyRequestConfirmation;
use common::RequestPolicyPostBody;
use common::RequestPolicyPostResponse;
use database::CompanyAwsAccount;
use database::CompanyCloudflareAccount;
use database::CompanyGcpProject;
use database::CompanyPolicy;
use database::CompanySlack;
use database::NewRequestFromPolicy;
use database::Pool;
use database::RequestSlack;
use database::RequestWrapper;
use slack::Slack;
use slack::SlackApiChatPostMessageRequest;

use crate::auth::UserRole;
use crate::auth::UserWithAccessRole;
use crate::error;
use crate::public_doc::ex;
use crate::slack_integration::AccessRequestSlackTemplate;
use crate::Result;

/// Make an access request for a policy
#[utoipa::path(
    context_path = "/v1/request/policy",
    tag = "requests",
    security(("user_token" = [])),
    request_body = RequestPolicyPostBody,
    responses(
        (status = 200, body = RequestPolicyPostResponse),
        (status = 400, body = ErrorResponse, example = json!(ex(StatusCode::BAD_REQUEST))),
        (status = 401, body = ErrorResponse, example = json!(ex(StatusCode::UNAUTHORIZED))),
        (status = 404, body = ErrorResponse, example = json!(ex(StatusCode::NOT_FOUND))),
        (status = 500, body = ErrorResponse, example = json!(ex(StatusCode::INTERNAL_SERVER_ERROR))),
    )
)]
#[post("/{id}")]
#[allow(clippy::too_many_lines)]
async fn request_policy_post(
    pool: web::Data<Pool>,
    authenticated: UserWithAccessRole<UserRole>,
    id: web::Path<String>,
    body: web::Json<RequestPolicyPostBody>,
) -> Result<web::Json<RequestPolicyPostResponse>> {
    let connection = &mut *pool.get()?;

    let user = authenticated.user(connection)?;
    let company = user.company(connection)?;

    let policy = CompanyPolicy::read(connection, &id)?;

    if policy.company_id != company.id {
        return Err(error::Api::UnauthorizedNotFound);
    }

    if !user.can_make_requests(connection)? {
        return Err(error::Api::UnauthorizedRequest);
    }

    let partially_parsed: NewPolicy = serde_json::from_str(&policy.policy)?;

    if partially_parsed.gcp.is_some() && body.gcp_project.is_none() {
        return Err(error::Api::GcpProjectRequiredForPolicy);
    }

    if partially_parsed.aws.is_some() && body.aws_account.is_none() {
        return Err(error::Api::AwsAccountRequiredForPolicy);
    }

    if partially_parsed.cloudflare.is_some() && body.cloudflare_account.is_none() {
        return Err(error::Api::CloudflareAccountRequiredForPolicy);
    }

    let mut approvals_required = 0;
    let mut admin_approval_required = false;

    if let Some(gcp_project) = &body.gcp_project {
        match CompanyGcpProject::read_by_project(connection, gcp_project, company.id) {
            Ok(project) => {
                approvals_required = project.approvals_required;
                admin_approval_required = project.admin_approval_required;
            }
            Err(_) => return Err(error::Api::UnknownGcpProject(gcp_project.clone())),
        }
    }

    if let Some(aws_account) = &body.aws_account {
        match CompanyAwsAccount::read_by_alias(connection, aws_account, company.id) {
            Ok(account) => {
                if approvals_required == 0 && !admin_approval_required {
                    approvals_required = account.approvals_required;
                    admin_approval_required = account.admin_approval_required;
                }
            }
            Err(_) => return Err(error::Api::UnknownAwsAccount(aws_account.clone())),
        }
    }

    if let Some(cloudflare_account) = &body.cloudflare_account {
        match CompanyCloudflareAccount::read_by_alias(connection, cloudflare_account, company.id) {
            Ok(account) => {
                if approvals_required == 0 && !admin_approval_required {
                    approvals_required = account.approvals_required;
                    admin_approval_required = account.admin_approval_required;
                }
            }
            Err(_) => {
                return Err(error::Api::UnknownCloudflareAccount(
                    cloudflare_account.clone(),
                ));
            }
        }
    }

    let is_pre_approved = approvals_required == 0 && !admin_approval_required;

    let request = RequestWrapper::create_from_policy(
        connection,
        &user,
        company.id,
        is_pre_approved,
        &NewRequestFromPolicy {
            duration: body.minutes,
            gcp_project: body.gcp_project.clone(),
            gcp: partially_parsed.gcp,
            cloudflare_account: body.cloudflare_account.clone(),
            cloudflare: partially_parsed.cloudflare,
            aws_account: body.aws_account.clone(),
            aws: partially_parsed.aws,
            justification: body.justification.clone(),
            policy: policy.name,
        },
    )?;

    if let Ok(company_slack) = CompanySlack::read(connection, company.id) {
        let slack = Slack::new(&company_slack.access_token)?;
        let slack_blocks = AccessRequestSlackTemplate::from(&request);
        match slack
            .chat_post_message(&SlackApiChatPostMessageRequest {
                channel: company_slack.channel_id.clone(),
                content: slack_blocks.render(),
                as_user: None,
                icon_emoji: None,
                icon_url: None,
                link_names: None,
                parse: None,
                thread_ts: None,
                username: None,
                reply_broadcast: None,
                unfurl_links: None,
                unfurl_media: None,
            })
            .await
        {
            Ok(response) => {
                if response.ok {
                    RequestSlack::create(
                        connection,
                        RequestSlack {
                            access_request_id: request.id.clone(),
                            company_id: company.id,
                            channel_id: company_slack.channel_id.clone(),
                            ts: response.ts,
                        },
                    )?;
                } else {
                    log::error!("{}", response.error.unwrap());
                }
            }
            Err(error) => {
                log::error!("{}", error);
            }
        }
    }

    Ok(web::Json(RequestPolicyPostResponse(
        PolicyRequestConfirmation {
            request_id: request.id.clone(),
            request_alias: request.alias,
        },
    )))
}
