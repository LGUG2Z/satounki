use actix::Addr;
use actix_web::post;
use actix_web::web;
use actix_web::HttpResponse;
use common::AccessRequestState;
use common::ServerMessage;
use common::UserInteraction;
use database::AccessRequest;
use database::Company;
use database::CompanySlack;
use database::Pool;
use database::RequestSlack;
use database::RequestWrapper;
use database::User;
use serde::Deserialize;
use serde::Serialize;
use slack::Attachment;
use slack::Slack;
use slack::SlackApiChatUpdateRequest;
use slack::SlackMessageContent;
use slack_blocks::blocks::Actions;
use slack_blocks::blocks::Section;
use slack_blocks::elems::button::Style;
use slack_blocks::elems::Button;
use slack_blocks::text::ToSlackMarkdown;
use slack_blocks::Block;

use crate::error;
use crate::worker::CheckConnection;
use crate::worker::Outgoing;
use crate::worker::Server;
use crate::Result;
use crate::SATOUNKI_URL;

#[derive(Debug, Serialize, Deserialize)]
struct InteractivityPayload {
    payload: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InteractivityPayloadSchema<'a> {
    pub actions: Vec<Action>,
    pub message: SlackMessageContent<'a>,
    pub response_url: String,
    pub user: UserInformation,
    pub team: Team,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Action {
    pub action_id: String,
    pub action_ts: String,
    pub block_id: String,
    pub style: String,
    pub text: Text,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Text {
    pub emoji: bool,
    pub text: String,
    #[serde(rename = "type")]
    pub type_field: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct UserInformation {
    pub id: String,
    pub name: String,
    pub team_id: String,
    pub username: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Team {
    pub id: String,
    pub domain: String,
}

pub const COLOR_APPROVED: &str = "#36C5F0";
pub const COLOR_ACTIVE: &str = "#007A5A";
pub const COLOR_DANGER: &str = "#E01E5A";

/// Handle Slack interactions
#[post("/slack")]
#[allow(clippy::too_many_lines)]
async fn slack_post(
    pool: web::Data<Pool>,
    websocket: web::Data<Addr<Server>>,
    body: web::Form<InteractivityPayload>,
) -> Result<HttpResponse> {
    let payload: InteractivityPayloadSchema = serde_json::from_str(&body.payload)?;

    let connection = &mut *pool.get()?;

    let company_slack = CompanySlack::read_by_team_id(connection, &payload.team.id)?;
    let company = Company::read(connection, company_slack.company_id)?;

    let slack = Slack::new(&company_slack.access_token)?;
    let profile_response = slack.users_profile_get(&payload.user.id).await?;
    let session_user = User::read_by_email(connection, &profile_response.profile.email)?;

    if !session_user.belongs_to_company(connection, company.id)? {
        return Err(error::Api::UnauthorizedNotFound);
    }

    if let Some(action) = payload.actions.first() {
        let access_request =
            AccessRequest::read_from_alias(connection, &action.block_id, company.id)?;
        let request = RequestWrapper::read(connection, &access_request.id)?;

        let original_request_user = User::read_by_email(connection, &request.requester)?;

        let can_approve_reject_extend_request = (session_user.is_approver(connection)?
            || session_user.is_administrator(connection)?)
            && session_user.id != original_request_user.id;

        let mut approved = false;
        let response = match action.action_id.as_str() {
            "approve" => {
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
            "reject" => {
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
            _ => unreachable!(),
        };

        let worker_connected = websocket
            .send(CheckConnection {
                company_domain: company.domain.clone(),
            })
            .await
            .unwrap_or(false);

        if worker_connected {
            match action.action_id.as_str() {
                "approve" => {
                    if approved {
                        let notification = ServerMessage::AccessRequestApproved {
                            data: Box::new(response.clone()),
                        };

                        websocket.do_send(Outgoing {
                            company_domain: company.domain.clone(),
                            msg: notification,
                        });
                    }
                }
                "reject" => {}
                _ => unreachable!(),
            }
        }

        let request_slack = RequestSlack::read(connection, &response.id)?;
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

#[derive(Debug, Clone)]
pub struct AccessRequestSlackTemplate {
    request_alias: String,
    request_url: String,
    policy: String,
    policy_url: String,
    duration: i32,
    json: String,
    approved: bool,
    approvals: Option<Vec<UserInteraction>>,
    state: AccessRequestState,
    justification: String,
}

impl From<&common::Request> for AccessRequestSlackTemplate {
    fn from(r: &common::Request) -> Self {
        Self {
            request_alias: r.alias.clone(),
            request_url: format!("{}/request/{}", *SATOUNKI_URL, &r.alias),
            policy: r.policy.clone(),
            policy_url: format!("{}/policy/{}", *SATOUNKI_URL, &r.policy),
            json: serde_json::to_string_pretty(r).expect("could not convert request to json"),
            duration: r.minutes,
            approved: r.approved,
            approvals: r.approvals.clone(),
            state: r.state,
            justification: r.justification.clone(),
        }
    }
}

impl AccessRequestSlackTemplate {
    #[allow(clippy::too_many_lines)]
    pub fn render(&self) -> SlackMessageContent {
        let access_request = Section::builder()
            .field(format!("*Request:*\n<{}|{}>", self.request_url, self.request_alias).markdown())
            .field(format!("*Justification:*\n{}", self.justification).markdown())
            .build();

        let policy_duration = Section::builder()
            .field(format!("*Policy:*\n<{}|{}>", self.policy_url, self.policy).markdown())
            .field(format!("*Duration:*\n{} minutes", self.duration).markdown())
            .build();

        let mut blocks = vec![
            Block::Section(access_request),
            Block::Section(policy_duration),
        ];

        let mut color = None;

        match self.state {
            AccessRequestState::Pending => {
                let mut state_fields = vec![];

                if let Some(approvals) = &self.approvals {
                    let approvals: Vec<_> = approvals.iter().map(|i| i.user.clone()).collect();

                    state_fields.push(format!("*Approvals:*\n{}", approvals.join("\n")).markdown());
                };

                state_fields.push(format!("*Approved:*\n{}", self.approved).markdown());
                state_fields.push(format!("*State:*\n{}", "Pending").markdown());

                blocks.push(Block::Section(
                    Section::builder()
                        .fields(state_fields.into_iter().map(slack_blocks::text::Text::from))
                        .build(),
                ));

                if self.approved {
                    color = Option::from(String::from(COLOR_APPROVED));
                } else {
                    let actions = Actions::builder()
                        .element(
                            Button::builder()
                                .style(Style::Primary)
                                .text("Approve")
                                .action_id("approve")
                                .value("approve")
                                .build(),
                        )
                        .element(
                            Button::builder()
                                .style(Style::Danger)
                                .text("Reject")
                                .action_id("reject")
                                .value("reject")
                                .build(),
                        )
                        .block_id(&self.request_alias)
                        .build();

                    blocks.push(Block::Actions(actions));
                }
            }
            AccessRequestState::Active => {
                color = Option::from(String::from(COLOR_ACTIVE));
                blocks.push(Block::Section(
                    Section::builder()
                        .field(format!("*State:*\n{}", "Active").markdown())
                        .build(),
                ));
            }
            AccessRequestState::Completed => {
                color = Option::from(String::from(COLOR_ACTIVE));
                blocks.push(Block::Section(
                    Section::builder()
                        .field(format!("*State:*\n{}", "Completed").markdown())
                        .build(),
                ));
            }
            AccessRequestState::Cancelled => {
                color = Option::from(String::from(COLOR_DANGER));
                blocks.push(Block::Section(
                    Section::builder()
                        .field(format!("*State:*\n{}", "Cancelled").markdown())
                        .build(),
                ));
            }
            AccessRequestState::Rejected => {
                color = Option::from(String::from(COLOR_DANGER));
                blocks.push(Block::Section(
                    Section::builder()
                        .field(format!("*State:*\n{}", "Rejected").markdown())
                        .build(),
                ));
            }
            AccessRequestState::Revoked => {
                color = Option::from(String::from(COLOR_DANGER));
                blocks.push(Block::Section(
                    Section::builder()
                        .field(format!("*State:*\n{}", "Revoked").markdown())
                        .build(),
                ));
            }
        }

        SlackMessageContent {
            text: None,
            blocks: Option::from(blocks),
            attachments: Option::from(vec![Attachment {
                blocks: vec![Block::Section(
                    Section::builder()
                        .text(format!("```{}```", self.json).markdown())
                        .build(),
                )],
                color,
            }]),
        }
    }
}
