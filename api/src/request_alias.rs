use actix::Addr;
use actix_web::get;
use actix_web::http::StatusCode;
use actix_web::patch;
use actix_web::web;
use actix_web::HttpResponse;
use common::AccessRequestState;
use common::RequestAliasGetResponse;
use common::RequestAliasPatchBody;
use common::RequestOperation;
use common::ServerMessage;
use database::AccessRequest;
use database::CompanySlack;
use database::Pool;
use database::RequestSlack;
use database::RequestWrapper;
use database::User;
use slack::Slack;
use slack::SlackApiChatUpdateRequest;

use crate::auth::ApiTokenOrUserWithAccessRole;
use crate::auth::UserRole;
use crate::auth::UserWithAccessRole;
use crate::error;
use crate::public_doc::ex;
use crate::slack_integration::AccessRequestSlackTemplate;
use crate::worker::CheckConnection;
use crate::worker::Outgoing;
use crate::worker::Server;
use crate::Result;

/// View a request
#[utoipa::path(
    context_path = "/v1/request/alias",
    tag = "requests",
    security(("user_token" = []), ("api_token" = [])),
    responses(
        (status = 200, body = RequestAliasGetResponse, example = json!(RequestAliasGetResponse::example())),
        (status = 401, body = ErrorResponse, example = json!(ex(StatusCode::UNAUTHORIZED))),
        (status = 404, body = ErrorResponse, example = json!(ex(StatusCode::NOT_FOUND))),
        (status = 500, body = ErrorResponse, example = json!(ex(StatusCode::INTERNAL_SERVER_ERROR))),
    )
)]
#[get("/{alias}")]
async fn request_alias_get(
    pool: web::Data<Pool>,
    authenticated: ApiTokenOrUserWithAccessRole<UserRole>,
    alias: web::Path<String>,
) -> Result<web::Json<RequestAliasGetResponse>> {
    let connection = &mut *pool.get()?;
    let company_id = authenticated.information().company_id(connection)?;
    let access_request = AccessRequest::read_from_alias(connection, &alias, company_id)?;
    let request = RequestWrapper::read(connection, &access_request.id)?;

    if request.company_id != company_id {
        return Err(error::Api::UnauthorizedNotFound);
    }

    Ok(web::Json(RequestAliasGetResponse(request)))
}

/// Update the state of a request
#[utoipa::path(
    context_path = "/v1/request/alias",
    tag = "requests",
    security(("user_token" = [])),
    request_body = RequestAliasPatchBody,
    responses(
        (status = 200),
        (status = 401, body = ErrorResponse, example = json!(ex(StatusCode::UNAUTHORIZED))),
        (status = 422, body = ErrorResponse, example = json!(ex(StatusCode::UNPROCESSABLE_ENTITY))),
        (status = 404, body = ErrorResponse, example = json!(ex(StatusCode::NOT_FOUND))),
        (status = 500, body = ErrorResponse, example = json!(ex(StatusCode::INTERNAL_SERVER_ERROR))),
    )
)]
#[allow(clippy::too_many_lines)]
#[patch("/{alias}")]
async fn request_alias_patch(
    pool: web::Data<Pool>,
    authenticated: UserWithAccessRole<UserRole>,
    alias: web::Path<String>,
    websocket: web::Data<Addr<Server>>,
    body: web::Json<RequestAliasPatchBody>,
) -> Result<HttpResponse> {
    let connection = &mut *pool.get()?;

    let session_user = authenticated.user(connection)?;
    let company = session_user.company(connection)?;

    let access_request = AccessRequest::read_from_alias(connection, &alias, company.id)?;
    let request = RequestWrapper::read(connection, &access_request.id)?;

    if request.company_id != company.id {
        return Err(error::Api::UnauthorizedNotFound);
    }

    let mut can_cancel_request = false;
    let mut can_approve_reject_extend_request = false;
    let mut can_revoke_request = false;
    let mut can_complete_request = false;

    let original_request_user = User::read_by_email(connection, &request.requester)?;

    if original_request_user.id == session_user.id {
        can_cancel_request = true;
        can_complete_request = true;
    } else {
        if session_user.is_approver(connection)? {
            can_approve_reject_extend_request = true;
        }

        if session_user.is_administrator(connection)? {
            can_approve_reject_extend_request = true;
            can_revoke_request = true;
        }
    }

    let mut approved = false;
    let response = match &**body {
        RequestOperation::Approve => {
            if !matches!(request.state, AccessRequestState::Pending) {
                return Err(error::Api::RequestNotPendingCannotApprove);
            }

            if !can_approve_reject_extend_request {
                if original_request_user.id == session_user.id {
                    return Err(error::Api::UnauthorizedSelfApproval);
                }

                return Err(error::Api::UnauthorizedApproval);
            }

            if request.approved {
                return Err(error::Api::RequestAlreadyApproved);
            }

            if let Some(approvals) = &request.approvals {
                for approval in approvals {
                    if approval.user == session_user.email {
                        return Err(error::Api::UnauthorizedMultipleApprovals);
                    }
                }
            }

            let response =
                RequestWrapper(request).approve(connection, &session_user, company.id)?;

            if response.approved {
                approved = true;
            }

            response
        }
        RequestOperation::Reject => {
            if !matches!(request.state, AccessRequestState::Pending) {
                return Err(error::Api::RequestNotPendingCannotReject);
            }

            if !can_approve_reject_extend_request {
                if original_request_user.id == session_user.id {
                    return Err(error::Api::UnauthorizedSelfRejection);
                }

                return Err(error::Api::UnauthorizedRejection);
            }

            if request.approved {
                return Err(error::Api::RequestAlreadyApproved);
            }

            RequestWrapper(request).reject(connection, &session_user.email)?
        }
        RequestOperation::Cancel => {
            if !matches!(request.state, AccessRequestState::Pending) {
                return Err(error::Api::RequestNotPendingCannotCancel);
            }

            if !can_cancel_request {
                return Err(error::Api::UnauthorizedRequesterRequired);
            }

            if request.approved {
                return Err(error::Api::RequestAlreadyApproved);
            }

            RequestWrapper(request).cancel(connection, &session_user.email)?
        }
        RequestOperation::Complete => {
            if !matches!(request.state, AccessRequestState::Active) {
                return Err(error::Api::RequestNotActiveCannotComplete);
            }

            if !can_complete_request {
                return Err(error::Api::UnauthorizedRequesterRequired);
            }

            RequestWrapper(request).complete(connection)?
        }
        RequestOperation::Extend(minutes) => {
            if !matches!(request.state, AccessRequestState::Active) {
                return Err(error::Api::RequestNotActiveCannotExtend);
            }

            if !can_approve_reject_extend_request {
                if original_request_user.id == session_user.id {
                    return Err(error::Api::UnauthorizedSelfExtension);
                }

                return Err(error::Api::UnauthorizedExtension);
            }

            RequestWrapper(request).extend(connection, &session_user.email, *minutes)?
        }
        RequestOperation::Revoke => {
            if !matches!(request.state, AccessRequestState::Active) {
                return Err(error::Api::RequestNotActiveCannotRevoke);
            }

            if !can_revoke_request {
                return Err(error::Api::UnauthorizedRevocation);
            }

            RequestWrapper(request).revoke(connection, &session_user.email)?
        }
    };

    let worker_connected = websocket
        .send(CheckConnection {
            company_domain: company.domain.clone(),
        })
        .await
        .unwrap_or(false);

    if worker_connected {
        let response_cl = response.clone();
        match &**body {
            RequestOperation::Approve => {
                if approved {
                    let notification = ServerMessage::AccessRequestApproved {
                        data: Box::new(response_cl),
                    };

                    websocket.do_send(Outgoing {
                        company_domain: company.domain.clone(),
                        msg: notification,
                    });
                }
            }
            RequestOperation::Complete => {
                let notification = ServerMessage::AccessRequestCompleted {
                    data: Box::new(response_cl),
                };

                websocket.do_send(Outgoing {
                    company_domain: company.domain.clone(),
                    msg: notification,
                });
            }
            RequestOperation::Revoke => {
                let notification = ServerMessage::AccessRequestRevoked {
                    data: Box::new(response_cl),
                };

                websocket.do_send(Outgoing {
                    company_domain: company.domain.clone(),
                    msg: notification,
                });
            }
            _ => {}
        }
    }

    if let (Ok(request_slack), Ok(company_slack)) = (
        RequestSlack::read(connection, &response.id),
        CompanySlack::read(connection, company.id),
    ) {
        let slack_template = AccessRequestSlackTemplate::from(&response);
        let slack = Slack::new(&company_slack.access_token)?;

        match slack
            .chat_update(&SlackApiChatUpdateRequest {
                channel: request_slack.channel_id,
                content: slack_template.render(),
                ts: request_slack.ts,
                as_user: None,
                link_names: None,
                parse: None,
            })
            .await
        {
            Ok(response) => {
                if !response.ok {
                    log::error!("{}", response.error.unwrap());
                }
            }
            Err(error) => {
                log::error!("{}", error);
            }
        }
    }

    Ok(HttpResponse::Ok().finish())
}
