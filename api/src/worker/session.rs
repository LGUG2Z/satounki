use std::time::Instant;

use actix::fut;
use actix::Actor;
use actix::ActorContext;
use actix::ActorFutureExt;
use actix::Addr;
use actix::AsyncContext;
use actix::ContextFutureSpawner;
use actix::Handler;
use actix::StreamHandler;
use actix::WrapFuture;
use actix_web_actors::ws;
use common::AccessRequestState;
use common::ClientMessage;
use common::ServerMessage;
use database::Company;
use database::CompanySlack;
use database::Pool;
use database::RequestSlack;
use database::RequestWrapper;
use slack::Slack;
use slack::SlackApiChatUpdateRequest;

use super::Connect;
use super::Disconnect;
use super::Message;
use super::Outgoing;
use super::Server;
use super::CLIENT_TIMEOUT;
use super::HEARTBEAT_INTERVAL;
use crate::slack_integration::AccessRequestSlackTemplate;

pub struct Session {
    company: Company,
    hb: Instant,
    server_addr: Addr<Server>,
    pool: Pool,
}

impl Session {
    #[must_use]
    pub fn new(company: Company, server_addr: Addr<Server>, pool: Pool) -> Self {
        Self {
            company,
            hb: Instant::now(),
            server_addr,
            pool,
        }
    }
}

impl Actor for Session {
    type Context = ws::WebsocketContext<Self>;

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        log::info!(
            "[{}] worker session stopped, disconnecting",
            self.company.domain
        );

        self.server_addr.do_send(Disconnect {
            company_domain: self.company.domain.clone(),
        });
    }

    fn started(&mut self, ctx: &mut Self::Context) {
        let company_domain = self.company.domain.clone();

        log::info!("[{}] worker session starting", &company_domain);

        ctx.run_interval(HEARTBEAT_INTERVAL, move |act, ctx| {
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                log::info!("[{}] heartbeat failed, disconnecting", &company_domain);

                act.server_addr.do_send(Disconnect {
                    company_domain: act.company.domain.clone(),
                });
                // stop actor
                ctx.stop();

                // don't try to send a ping
                return;
            }
            ctx.ping(b"");
        });

        let session_addr = ctx.address();

        self.server_addr
            .send(Connect {
                addr: session_addr.recipient(),
                company_domain: self.company.domain.clone(),
            })
            .into_actor(self)
            .then(|res, _act, ctx| {
                res.map_or_else(|_| ctx.stop(), |_res| {});
                fut::ready(())
            })
            .wait(ctx);
    }
}

impl Handler<Message> for Session {
    type Result = ();

    fn handle(&mut self, msg: Message, ctx: &mut Self::Context) -> Self::Result {
        ctx.text(msg.0);
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for Session {
    #[allow(clippy::too_many_lines)]
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Err(err) => {
                log::warn!("Error handling msg: {:?}", err);
                ctx.stop();
            }
            Ok(ws::Message::Ping(msg)) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                self.hb = Instant::now();
            }
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            Ok(ws::Message::Close(reason)) => {
                self.server_addr.do_send(Disconnect {
                    company_domain: self.company.domain.clone(),
                });
                ctx.close(reason);
                ctx.stop();
            }
            Ok(ws::Message::Text(text)) => {
                let incoming: ClientMessage = match serde_json::from_slice(text.as_ref()) {
                    Ok(incoming) => incoming,
                    Err(error) => {
                        self.server_addr.do_send(Outgoing {
                            company_domain: self.company.domain.clone(),
                            msg: ServerMessage::Error {
                                message: error.to_string(),
                            },
                        });
                        return;
                    }
                };

                log::info!(
                    "[{}] received: {}",
                    self.company.domain,
                    incoming.to_string()
                );

                let connection = &mut self.pool.get().unwrap();

                let server_message = match incoming {
                    ClientMessage::GetRevokedAccessRequests => {
                        ServerMessage::RevokedAccessRequests {
                            data: RequestWrapper::unexpired_and_revoked(
                                connection,
                                self.company.id,
                            )
                            .unwrap(),
                        }
                    }
                    ClientMessage::GetCompletedAccessRequests => {
                        ServerMessage::CompletedAccessRequests {
                            data: RequestWrapper::unexpired_and_completed(
                                connection,
                                self.company.id,
                            )
                            .unwrap(),
                        }
                    }
                    ClientMessage::GetExpiredAccessRequests => {
                        ServerMessage::ExpiredAccessRequests {
                            data: RequestWrapper::expired_and_active(connection, self.company.id)
                                .unwrap(),
                        }
                    }
                    ClientMessage::GetApprovedAccessRequests => {
                        ServerMessage::ApprovedAccessRequests {
                            data: RequestWrapper::approved_and_pending(connection, self.company.id)
                                .unwrap(),
                        }
                    }
                    ClientMessage::UpdateAccessRequest { id, new_state } => {
                        let request = RequestWrapper::read(connection, &id).unwrap();
                        let request_id = request.id.clone();
                        let company_id = request.company_id;

                        let response = match new_state {
                            AccessRequestState::Active => {
                                RequestWrapper(request).activate(connection).unwrap()
                            }
                            AccessRequestState::Completed => {
                                RequestWrapper(request).complete(connection).unwrap()
                            }
                            _ => unreachable!(),
                        };

                        if let (Ok(request_slack), Ok(company_slack)) = (
                            RequestSlack::read(connection, &request_id),
                            CompanySlack::read(connection, company_id),
                        ) {
                            let slack_template = AccessRequestSlackTemplate::from(&response);
                            let slack = Slack::new(&company_slack.access_token).unwrap();

                            actix_web::rt::spawn(async move {
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
                            });
                        }

                        return;
                    }
                    ClientMessage::GetGcpProjects => ServerMessage::GcpProjects {
                        data: self
                            .company
                            .gcp_projects(connection)
                            .unwrap()
                            .into_iter()
                            .map(Into::into)
                            .collect(),
                    },
                    ClientMessage::GetAwsAccounts => ServerMessage::AwsAccounts {
                        data: self
                            .company
                            .aws_accounts(connection)
                            .unwrap()
                            .into_iter()
                            .map(Into::into)
                            .collect(),
                    },
                    ClientMessage::GetCloudflareAccounts => ServerMessage::CloudflareAccounts {
                        data: self
                            .company
                            .cloudflare_accounts(connection)
                            .unwrap()
                            .into_iter()
                            .map(Into::into)
                            .collect(),
                    },
                    ClientMessage::Error { .. } => todo!(),
                };

                self.server_addr.do_send(Outgoing {
                    company_domain: self.company.domain.clone(),
                    msg: server_message,
                });
            }
            _ => {
                self.server_addr.do_send(Disconnect {
                    company_domain: self.company.domain.clone(),
                });

                ctx.stop();
            }
        }
    }
}
