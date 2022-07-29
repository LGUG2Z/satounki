use std::fmt::Display;
use std::fmt::Formatter;

use chrono::DateTime;
use chrono::Local;
use clap::Parser;
use clap::ValueEnum;
use color_eyre::owo_colors::OwoColorize;
use common::AccessRequestState;
use common::ErrorResponse;
use common::PoliciesGetResponse;
use common::PolicyGetResponse;
use common::PolicyRequest;
use common::RequestAliasGetResponse;
use common::RequestAliasPatchBody;
use common::RequestOperation;
use common::RequestPolicyPostBody;
use common::RequestPolicyPostResponse;
use common::RequestsGetResponse;
use reqwest::StatusCode;
use serde::Serialize;
use tabled::settings::object::Columns;
use tabled::settings::object::Rows;
use tabled::settings::Disable;
use tabled::settings::Format;
use tabled::settings::Modify;
use tabled::settings::Style;
use tabled::Table;
use tabled::Tabled;
use termcolor::ColorChoice;
use termcolor::StandardStream;

use crate::reporters::Outcome;

mod reporters;

#[derive(Parser)]
#[clap(about, version)]
/// The Satounki command line interface
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Parser)]
struct DescribePolicy {
    /// Name of the policy to describe
    ///
    /// You can see all policies names with the 'list-policies' command
    policy: String,
}

#[derive(Parser)]
struct ApproveRequest {
    /// Alias of the request to approve
    request: String,
}

#[derive(Parser)]
struct DescribeRequest {
    /// Alias of the request to describe
    request: String,
}

#[derive(Parser)]
struct ExtendRequest {
    /// Alias of the request to extend
    request: String,
    /// Duration to extend the request by in minutes
    #[clap(short, long)]
    minutes: i32,
}

#[derive(Debug, Copy, Clone, ValueEnum, Serialize)]
enum RequestStatus {
    Pending,
    Active,
}

impl Display for RequestStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RequestStatus::Pending => write!(f, "pending"),
            RequestStatus::Active => write!(f, "active"),
        }
    }
}

#[derive(Parser)]
struct ListRequests {
    #[clap(value_enum, short, long, default_value_t = RequestStatus::Pending )]
    /// Status filter to apply to requests
    status: RequestStatus,
    /// Maximum number of matching requests to show
    #[clap(short, long, default_value = "10")]
    count: i32,
}

#[derive(Parser)]
struct CompleteRequest {
    /// Alias of the request to complete
    request: String,
}

#[derive(Parser)]
struct CancelRequest {
    /// Alias of the request to cancel
    request: String,
}

#[derive(Parser)]
struct RejectRequest {
    /// Alias of the request to reject
    request: String,
}

#[derive(Parser)]
struct RevokeRequest {
    /// Alias of the request to revoke
    request: String,
}

#[derive(Parser)]
struct Request {
    /// Name of the policy
    ///
    /// You can see all policies names with the 'list-policies' command
    policy: String,
    /// Duration of the request in minutes
    #[clap(short, long)]
    minutes: i32,
    /// Name of the AWS account (if policy contains AWS access)
    #[clap(short, long)]
    aws_account: Option<String>,
    /// Name of the Cloudflare account (if policy contains Cloudflare access)
    #[clap(short, long)]
    cloudflare_account: Option<String>,
    /// Name of the GCP project (if policy contains GCP access)
    #[clap(short, long)]
    gcp_project: Option<String>,
    /// Reason for the access request
    #[clap(short, long)]
    justification: Option<String>,
}

#[derive(Parser)]
enum SubCommand {
    /// List available policies
    ListPolicies,
    /// Describe a policy in detail
    ///
    /// Requires the "user" role
    #[clap(arg_required_else_help = true)]
    DescribePolicy(DescribePolicy),
    /// Submit an access request
    ///
    /// Requires the "user" role
    #[clap(arg_required_else_help = true)]
    Request(Request),
    /// List pending and active access requests
    ///
    /// Requires the "user" role
    ListRequests(ListRequests),
    /// Describe an access request in detail
    ///
    /// Requires the "user" role
    #[clap(arg_required_else_help = true)]
    DescribeRequest(DescribeRequest),
    /// Approve another user's pending access request
    ///
    /// Requires the "approver" role
    /// You cannot approve your own requests
    #[clap(arg_required_else_help = true)]
    ApproveRequest(ApproveRequest),
    /// Extend another user's active access request
    ///
    /// Requires the "approver" role
    /// You cannot extend your own requests
    #[clap(arg_required_else_help = true)]
    ExtendRequest(ExtendRequest),
    /// Reject another user's pending access request
    ///
    /// Requires the "approver" role
    /// You cannot reject your own requests
    #[clap(arg_required_else_help = true)]
    RejectRequest(RejectRequest),
    /// Complete one of your active access requests
    ///
    /// Requires the "user" role
    #[clap(arg_required_else_help = true)]
    CompleteRequest(CompleteRequest),
    /// Cancel one of your pending access requests
    ///
    /// Requires the "user" role
    #[clap(arg_required_else_help = true)]
    CancelRequest(CancelRequest),
    /// Revoke another user's active access request
    ///
    /// Requires the "administrator" role
    #[clap(arg_required_else_help = true)]
    RevokeRequest(RevokeRequest),
}

const BASE_URL: &str = "http://localhost:8080/v1";

#[derive(Tabled)]
struct PolicyListEntry {
    name: String,
    description: String,
}

#[derive(Tabled)]
struct RequestsListEntry {
    request: String,
    policy: String,
    timestamp: String,
    state: AccessRequestState,
}

fn main() -> color_eyre::Result<()> {
    let opts: Opts = Opts::parse();
    let token = std::env::var("SATOUNKI_USER_TOKEN").unwrap_or_else(|_| String::from("crj"));
    let client = reqwest::blocking::Client::new();
    let stdout = StandardStream::stdout(ColorChoice::Auto);
    match opts.subcmd {
        SubCommand::ListPolicies => {
            let response = client
                .get(format!("{BASE_URL}/policies"))
                .bearer_auth(token)
                .send()?;

            if response.status() == StatusCode::OK {
                let policies = response.json::<PoliciesGetResponse>()?;
                let mut entries = vec![];
                for policy in &*policies {
                    entries.push(PolicyListEntry {
                        name: policy.name.clone(),
                        description: policy.description.clone(),
                    });
                }

                println!(
                    "{}",
                    Table::new(entries)
                        .with(Style::psql())
                        .with(
                            Modify::new(Columns::single(0))
                                .with(Format::content(|s| s.bright_blue().to_string()))
                        )
                        .with(
                            Modify::new(Columns::single(1))
                                .with(Format::content(|s| s.green().to_string()))
                        )
                );
            } else {
                reporters::error(&[response.json::<ErrorResponse>()?.error.as_str()]);
            }
        }
        SubCommand::DescribePolicy(args) => {
            let response = client
                .get(format!("{BASE_URL}/policy/name/{}", args.policy))
                .bearer_auth(token)
                .send()?;

            if response.status() == StatusCode::OK {
                let policy = response.json::<PolicyGetResponse>()?;
                termcolor_json::to_writer(&mut stdout.lock(), &policy)?;
            } else {
                reporters::error(&[response.json::<ErrorResponse>()?.error.as_str()]);
            }
        }
        SubCommand::Request(args) => {
            let policy = client
                .get(format!("{BASE_URL}/policy/name/{}", args.policy))
                .bearer_auth(&token)
                .send()?
                .json::<PolicyGetResponse>()?;

            let mut errors = vec![];

            if policy.aws.is_some() && args.aws_account.is_none() {
                errors.push("requests for this policy require an aws account alias");
            }

            if policy.gcp.is_some() && args.gcp_project.is_none() {
                errors.push("requests for this policy require a gcp project")
            }

            if !errors.is_empty() {
                reporters::error(&errors);
            }

            let justification = if let Some(justification) = args.justification {
                justification
            } else {
                let instructions = [
                    "",
                    "# Please enter the justification for your request above.",
                    "# You can include links to tickets, issues, incidents, etc.",
                    "# Lines starting with '#' will be ignored, and an empty",
                    "# justification aborts the commit",
                    "",
                ]
                .join("\n");

                edit::edit(instructions)?.trim().to_string()
            };

            let mut split: Vec<_> = justification.split('\n').collect();
            split.retain(|l| !l.starts_with('#') && !l.is_empty());
            let justification = split.join("\n");

            if justification.is_empty() {
                reporters::error(&["a justification message is required when making requests"]);
            }

            let response = client
                .post(format!("{BASE_URL}/request/policy/{}", policy.id))
                .json(&RequestPolicyPostBody(PolicyRequest {
                    minutes: args.minutes,
                    justification,
                    cloudflare_account: args.cloudflare_account,
                    gcp_project: args.gcp_project,
                    aws_account: args.aws_account,
                }))
                .bearer_auth(&token)
                .send()?;

            if response.status() == StatusCode::OK {
                let response = response.json::<RequestPolicyPostResponse>()?;
                let data = vec![
                    (
                        "view details",
                        format!("satounki describe-request \"{}\"", &response.request_alias),
                    ),
                    (
                        "approve request",
                        format!("satounki approve-request \"{}\"", &response.request_alias),
                    ),
                    (
                        "cancel request",
                        format!("satounki cancel-request \"{}\"", &response.request_alias),
                    ),
                    (
                        "complete request",
                        format!("satounki complete-request \"{}\"", &response.request_alias),
                    ),
                ];

                println!(
                    "{}",
                    Table::new(data)
                        .with(Style::blank())
                        .with(Disable::row(Rows::new(..1)))
                        .with(
                            Modify::new(Columns::single(0))
                                .with(Format::content(|s| s.bright_blue().to_string()))
                        )
                        .with(
                            Modify::new(Columns::single(1))
                                .with(Format::content(|s| s.green().to_string()))
                        )
                );
            } else {
                reporters::error(&[response.json::<ErrorResponse>()?.error.as_str()]);
            }
        }
        SubCommand::ListRequests(args) => {
            let response = client
                .get(format!("{BASE_URL}/requests"))
                .query(&[
                    ("state", args.status.to_string()),
                    ("count", args.count.to_string()),
                ])
                .bearer_auth(&token)
                .send()?;

            if response.status() == StatusCode::OK {
                let requests = response.json::<RequestsGetResponse>()?;
                let mut entries = vec![];
                for request in &*requests {
                    let local: DateTime<Local> = DateTime::from(request.timestamp);
                    entries.push(RequestsListEntry {
                        request: request.alias.clone(),
                        policy: request.policy.clone(),
                        timestamp: local.to_rfc2822(),
                        state: request.state,
                    });
                }

                println!(
                    "{}",
                    Table::new(entries)
                        .with(Style::psql())
                        .with(
                            Modify::new(Columns::single(0))
                                .with(Format::content(|s| s.bright_blue().to_string()))
                        )
                        .with(
                            Modify::new(Columns::single(1))
                                .with(Format::content(|s| s.green().to_string()))
                        )
                        .with(
                            Modify::new(Columns::single(2))
                                .with(Format::content(|s| s.bright_magenta().to_string()))
                        )
                        .with(Modify::new(Columns::single(3)).with(Format::content(|s| {
                            match args.status {
                                RequestStatus::Pending => s.bright_yellow().to_string(),
                                RequestStatus::Active => s.bright_green().to_string(),
                            }
                        })))
                );

                if requests.is_empty() {
                    println!("No {} requests found", args.status)
                }
            } else {
                reporters::error(&[response.json::<ErrorResponse>()?.error.as_str()]);
            }
        }
        SubCommand::DescribeRequest(args) => {
            let response = client
                .get(format!("{BASE_URL}/request/alias/{}", args.request))
                .bearer_auth(&token)
                .send()?;

            if response.status() == StatusCode::OK {
                let request = response.json::<RequestAliasGetResponse>()?;
                termcolor_json::to_writer(&mut stdout.lock(), &request)?;
            } else {
                reporters::error(&[response.json::<ErrorResponse>()?.error.as_str()]);
            }
        }
        SubCommand::ApproveRequest(args) => {
            let response = client
                .patch(format!("{BASE_URL}/request/alias/{}", args.request))
                .json(&RequestAliasPatchBody(RequestOperation::Approve))
                .bearer_auth(&token)
                .send()?;

            if response.status() == StatusCode::OK {
                reporters::outcome(
                    Outcome::Positive,
                    &[("Approval added", args.request.as_str())],
                );
            } else {
                reporters::error(&[response.json::<ErrorResponse>()?.error.as_str()]);
            }
        }
        SubCommand::RejectRequest(args) => {
            let response = client
                .patch(format!("{BASE_URL}/request/alias/{}", args.request))
                .json(&RequestAliasPatchBody(RequestOperation::Reject))
                .bearer_auth(&token)
                .send()?;

            if response.status() == StatusCode::OK {
                reporters::outcome(
                    Outcome::Negative,
                    &[("Request rejected", args.request.as_str())],
                );
            } else {
                reporters::error(&[response.json::<ErrorResponse>()?.error.as_str()]);
            }
        }
        SubCommand::CompleteRequest(args) => {
            let response = client
                .patch(format!("{BASE_URL}/request/alias/{}", args.request))
                .json(&RequestAliasPatchBody(RequestOperation::Complete))
                .bearer_auth(&token)
                .send()?;

            if response.status() == StatusCode::OK {
                reporters::outcome(
                    Outcome::Positive,
                    &[("Request completed", args.request.as_str())],
                );
            } else {
                reporters::error(&[response.json::<ErrorResponse>()?.error.as_str()]);
            }
        }
        SubCommand::CancelRequest(args) => {
            let response = client
                .patch(format!("{BASE_URL}/request/alias/{}", args.request))
                .json(&RequestAliasPatchBody(RequestOperation::Cancel))
                .bearer_auth(&token)
                .send()?;

            if response.status() == StatusCode::OK {
                reporters::outcome(
                    Outcome::Negative,
                    &[("Request cancelled", args.request.as_str())],
                );
            } else {
                reporters::error(&[response.json::<ErrorResponse>()?.error.as_str()]);
            }
        }
        SubCommand::ExtendRequest(args) => {
            let response = client
                .patch(format!("{BASE_URL}/request/alias/{}", args.request))
                .json(&RequestAliasPatchBody(RequestOperation::Extend(
                    args.minutes,
                )))
                .bearer_auth(&token)
                .send()?;

            if response.status() == StatusCode::OK {
                reporters::outcome(
                    Outcome::Neutral,
                    &[("Request extended", args.request.as_str())],
                );
            } else {
                reporters::error(&[response.json::<ErrorResponse>()?.error.as_str()]);
            }
        }
        SubCommand::RevokeRequest(args) => {
            let response = client
                .patch(format!("{BASE_URL}/request/alias/{}", args.request.red()))
                .json(&RequestAliasPatchBody(RequestOperation::Revoke))
                .bearer_auth(&token)
                .send()?;

            if response.status() == StatusCode::OK {
                reporters::outcome(
                    Outcome::Negative,
                    &[("Request revoked", args.request.as_str())],
                );
            } else {
                reporters::error(&[response.json::<ErrorResponse>()?.error.as_str()]);
            }
        }
    }

    Ok(())
}
