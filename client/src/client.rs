use aws::Aws;
use chrono::Duration;
use cloudflare::Cloudflare;
use color_eyre::Result;
use common::AccessRequestState;
use common::ClientMessage;
use common::PermissionsAction;
use common::ServerMessage;
use futures_util::SinkExt;
use futures_util::StreamExt;
use gcloud::Gcloud;
use gcloud::IamPolicy;
use tokio::net::TcpStream;
use tokio::time;
use tokio::time::Interval;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::error::ProtocolError;
use tokio_tungstenite::tungstenite::Error;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::MaybeTlsStream;
use tokio_tungstenite::WebSocketStream;
use url::Url;

use crate::request::Executor;
use crate::Configuration;
use crate::GcpServiceAccount;
use crate::SKIP_CHECKS;

pub struct Client {
    pub company_domain: String,
    pub company_worker_key: String,
    pub configuration: Configuration,
    pub websocket_stream: WebSocketStream<MaybeTlsStream<TcpStream>>,
    pub poll_interval: Interval,
}
impl Client {
    pub async fn new(
        company_domain: String,
        company_worker_key: String,
        configuration: Configuration,
        port: i32,
    ) -> Result<Self> {
        let mut url = Url::parse(&format!("ws://localhost:{port}/socket/worker"))?;
        url.set_query(Some(&format!(
            "company_domain={company_domain}&company_worker_key={company_worker_key}",
        )));

        log::info!("attempting to establish websocket connection");
        let (websocket_stream, _) = connect_async(url).await?;
        log::info!("connection established");
        let poll_interval = time::interval(Duration::minutes(1).to_std()?);
        log::info!("poll interval set to 1 minute");

        Ok(Self {
            company_domain,
            company_worker_key,
            configuration,
            websocket_stream,
            poll_interval,
        })
    }

    #[allow(clippy::too_many_lines)]
    pub async fn listen(&mut self) -> Result<()> {
        log::info!("listening on websocket connection");
        log::info!("validating required access for credentials");

        self.websocket_stream
            .send(Message::Text(ClientMessage::GetGcpProjects.to_string()))
            .await?;

        self.websocket_stream
            .send(Message::Text(ClientMessage::GetAwsAccounts.to_string()))
            .await?;

        self.websocket_stream
            .send(Message::Text(
                ClientMessage::GetCloudflareAccounts.to_string(),
            ))
            .await?;

        loop {
            tokio::select! {
                _ = self.poll_interval.tick() => {
                    log::info!("polling for expired (but active) access requests");
                    self.websocket_stream
                        .send(Message::Text(ClientMessage::GetExpiredAccessRequests.to_string()))
                        .await?;

                    log::info!("polling for approved (but inactive) access requests");
                    self.websocket_stream
                        .send(Message::Text(
                            ClientMessage::GetApprovedAccessRequests.to_string(),
                        ))
                        .await?;

                    log::info!("polling for completed (but unexpired) access requests");
                    self.websocket_stream
                        .send(Message::Text(
                            ClientMessage::GetCompletedAccessRequests.to_string(),
                        ))
                        .await?;

                    log::info!("polling for revoked (but unexpired) access requests");
                    self.websocket_stream
                        .send(Message::Text(
                            ClientMessage::GetRevokedAccessRequests.to_string(),
                        ))
                        .await?;
                }
                msg = self.websocket_stream.next() => {
                    match msg {
                        None => break,
                        Some(Err(msg)) => {
                            match msg {
                                Error::ConnectionClosed |
                                Error::AlreadyClosed |
                                Error::Protocol(ProtocolError::ResetWithoutClosingHandshake) => {
                                    let mut success = false;
                                    while !success {
                                        let mut url = Url::parse("ws://localhost:8080/socket/worker")?;
                                        url.set_query(Some(&format!(
                                            "company_domain={}&company_worker_key={}",
                                            self.company_domain, self.company_worker_key
                                        )));

                                        log::warn!("connection lost");
                                        log::info!("attempting to re-establish websocket connection");
                                        match connect_async(url).await {
                                            Err(_) => {
                                                log::warn!("connection failed, will try again in 5 seconds");
                                                std::thread::sleep(Duration::seconds(5).to_std()?);
                                            }
                                            Ok((websocket_stream, _)) => {
                                                self.websocket_stream = websocket_stream;
                                                log::info!("connection re-established");

                                                success = true;
                                            }
                                        }
                                    }
                                },
                                _ => log::error!("{}", msg),
                            };
                        }
                        Some(Ok(msg)) => {
                            match msg {
                                Message::Close(_) => break,
                                Message::Text(text) => {
                                    log::info!("received: {}", &text);
                                    let message: ServerMessage = serde_json::from_str(&text)?;
                                    let message_cl = message.clone();
                                    match message {
                                        ServerMessage::Error { message: _ } => {
                                            // dbg!(message);
                                        }
                                        ServerMessage::ExpiredAccessRequests { data }
                                        | ServerMessage::RevokedAccessRequests { data }
                                        | ServerMessage::CompletedAccessRequests { data } => {
                                            for request in data {
                                                let id = request.id.clone();
                                                Executor(request)
                                                    .execute(PermissionsAction::Remove, &self.configuration).await?;

                                                let new_state = match message_cl {
                                                    ServerMessage::ExpiredAccessRequests { .. }
                                                    | ServerMessage::CompletedAccessRequests { .. } => AccessRequestState::Completed,
                                                    ServerMessage::RevokedAccessRequests { .. } => AccessRequestState::Revoked,
                                                    _ => unreachable!()
                                                };

                                                self.websocket_stream
                                                    .send(Message::Text(
                                                        ClientMessage::UpdateAccessRequest { id, new_state }.to_string()
                                                    ))
                                                    .await?;
                                            }

                                        }
                                        ServerMessage::ApprovedAccessRequests { data } => {
                                            for request in data {
                                                let id = request.id.clone();
                                                Executor(request)
                                                    .execute(PermissionsAction::Add, &self.configuration).await?;

                                                self.websocket_stream
                                                    .send(Message::Text(
                                                        ClientMessage::UpdateAccessRequest {
                                                            id,
                                                            new_state: AccessRequestState::Active,
                                                        }.to_string()
                                                    ))
                                                    .await?;
                                            }
                                        }
                                        ServerMessage::AccessRequestApproved { data } => {
                                            let id = data.id.clone();
                                            Executor(*data)
                                                .execute(PermissionsAction::Add, &self.configuration).await?;

                                            self.websocket_stream
                                                .send(Message::Text(
                                                    ClientMessage::UpdateAccessRequest {
                                                        id,
                                                        new_state: AccessRequestState::Active
                                                    }.to_string()
                                                ))
                                                .await?;
                                        }
                                        ServerMessage::AccessRequestCompleted { data } => {
                                            let id = data.id.clone();
                                            Executor(*data)
                                                .execute(PermissionsAction::Remove, &self.configuration).await?;

                                            self.websocket_stream
                                                .send(Message::Text(
                                                    ClientMessage::UpdateAccessRequest {
                                                        id,
                                                        new_state: AccessRequestState::Completed
                                                    }.to_string()
                                                ))
                                                .await?;
                                        }
                                        ServerMessage::AccessRequestRevoked { data } => {
                                            let id = data.id.clone();
                                            Executor(*data)
                                                .execute(PermissionsAction::Remove, &self.configuration).await?;

                                            self.websocket_stream
                                                .send(Message::Text(
                                                    ClientMessage::UpdateAccessRequest {
                                                        id,
                                                        new_state: AccessRequestState::Revoked
                                                    }.to_string()
                                                ))
                                                .await?;

                                        }
                                        ServerMessage::GcpProjects { data } => {
                                            if SKIP_CHECKS {
                                                continue;
                                            }

                                            if let Some(gcp_sa_path) = &self.configuration.gcp {
                                                let mut failed = vec![];

                                                for p in data {
                                                    let output = Gcloud::projects()
                                                        .get_iam_policy()
                                                        .project(&p.project)
                                                        .execute()?;

                                                    let project_policy: IamPolicy = serde_yaml::from_slice(&output.stdout)?;

                                                    let service_account: GcpServiceAccount = serde_yaml::from_str(&std::fs::read_to_string(gcp_sa_path)?)?;
                                                    if !project_policy.service_account_is_project_iam_admin(&service_account.client_email) {
                                                        failed.push(p.project);
                                                    }
                                                }

                                                if !failed.is_empty() {
                                                    log::error!(
                                                        "the provided service account does not have the \"roles/resourcemanager.projectIamAdmin\" role on the following project(s): {:?}",
                                                        failed
                                                    );

                                                    log::error!(
                                                        "please try starting the client again after ensuring the role has been added for all gcp projects registered with satounki"
                                                    );

                                                    break
                                                }
                                            } else {
                                                log::error!(
                                                    "a gcp service account path was not provided in the configuration"
                                                );

                                                log::error!(
                                                    "please try starting the client again after providing a gcp service account path"
                                                );
                                            }
                                        },
                                        ServerMessage::AwsAccounts { data } => {
                                            if SKIP_CHECKS {
                                                continue;
                                            }

                                           if let Some(credentials) = &self.configuration.aws {
                                                let mut failed = vec![];

                                                for a in data {
                                                    if let Some(creds) = credentials.get(&a.account) {
                                                        let client = Aws::new(&creds.aws_access_key_id, &creds.aws_secret_access_key);
                                                        if !client.iam_full_access().await? {
                                                            failed.push(a.account);
                                                        }
                                                    }
                                                }

                                                if !failed.is_empty() {
                                                    log::error!(
                                                        "the provided credentials not have the \"arn:aws:iam::aws:policy/IAMFullAccess\" policy on the following account(s): {:?}",
                                                        failed
                                                    );

                                                    log::error!(
                                                        "please try starting the client again after ensuring the policy has been added for all accounts registered with satounki"
                                                    );
                                                }
                                            } else {
                                                log::error!(
                                                    "aws credentials were not provided in the configuration"
                                                );

                                                log::error!(
                                                    "please try starting the client again after providing aws credentials for all accounts registered with satounki"
                                                );

                                                break
                                            }
                                        },
                                        ServerMessage::CloudflareAccounts { data } => {
                                            if SKIP_CHECKS {
                                                continue;
                                            }

                                           if let Some(credentials) = &self.configuration.cloudflare {
                                                let mut failed = vec![];

                                                for a in data {
                                                    if let Some(creds) = credentials.get(&a.account) {
                                                        let mut cloudflare = Cloudflare::new(&creds.access_token, &creds.account_id)?;
                                                        if !cloudflare.memberships_writer().await? {
                                                            failed.push(a.account);
                                                        }
                                                    }
                                                }

                                                if !failed.is_empty() {
                                                    log::error!(
                                                        "the provided access token does not have the \"Memerships Read\" and \"Memberships Write\" permissions on the following accounts: {:?}",
                                                        failed,
                                                    );

                                                    log::error!(
                                                        "please try starting the client again after ensuring these permissions have been given to the access token"
                                                    );
                                                }
                                            } else {
                                                log::error!(
                                                    "cloudflare credentials were not provided in the configuration"
                                                );

                                                log::error!(
                                                    "please try starting the client again after providing cloudflare credentials for all accounts registered with satounki"
                                                );

                                                break
                                            }
                                        },

                                    }
                                }
                                Message::Ping(ping) => {
                                    self.websocket_stream.send(Message::Pong(ping)).await?;
                                }
                                Message::Binary(_)
                                | Message::Pong(_)
                                | Message::Frame(_) => {}
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }
}
