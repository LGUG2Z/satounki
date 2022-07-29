#![allow(clippy::use_self)]

use chrono::Duration;
use chrono::Utc;
use common::AccessRequestState;
use common::AccessRole;
use common::AwsPolicy;
use common::CloudflareRole;
use common::GcpRole;
use common::Request;
use common::UserAliases;
use common::UserInteraction;
use derive_more::Deref;
use display_json::DisplayAsJson;
use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

use crate::AccessRequest;
use crate::Approval;
use crate::AwsRequest;
use crate::Cancellation;
use crate::CloudflareRequest;
use crate::CompanyAwsAccount;
use crate::CompanyGcpProject;
use crate::Extension;
use crate::GcpRequest;
use crate::Justification;
use crate::Rejection;
use crate::Result;
use crate::Revocation;
use crate::SqliteConnection;
use crate::User;
use crate::UserAlias;

#[derive(Debug, Serialize, Deserialize)]
pub struct NewFromPolicy {
    pub policy: String,
    pub duration: i32,
    pub gcp_project: Option<String>,
    pub gcp: Option<Vec<GcpRole>>,
    pub cloudflare_account: Option<String>,
    pub cloudflare: Option<Vec<CloudflareRole>>,
    pub aws_account: Option<String>,
    pub aws: Option<Vec<AwsPolicy>>,
    pub justification: String,
}

#[derive(Debug, Clone, Deref, Serialize, Deserialize, DisplayAsJson)]
pub struct Wrapper(pub Request);

impl Wrapper {
    #[allow(clippy::missing_panics_doc)]
    #[allow(clippy::cast_possible_truncation)]
    pub fn create_from_policy(
        connection: &mut SqliteConnection,
        user: &User,
        company_id: i32,
        is_pre_approved: bool,
        request: &NewFromPolicy,
    ) -> Result<Request> {
        let now = Utc::now();
        let inserted = AccessRequest::create(
            connection,
            &AccessRequest {
                id: Uuid::new_v4().to_string(),
                requester: user.email.clone(),
                company_id,
                timestamp: now,
                duration: request.duration,
                approved: is_pre_approved,
                access_expiry: None,
                state: AccessRequestState::Pending,
                modified: now,
                req_alias: format!(
                    "{}-{}-{}",
                    &user.first_name.to_lowercase(),
                    &user.last_name.to_lowercase(),
                    memorable_wordlist::kebab_case(33)
                ),
                policy: request.policy.clone(),
            },
        )?;

        let mut all_gcp_requests = vec![];
        let mut all_cf_requests = vec![];
        let mut all_aws_requests = vec![];

        if let Some(gcp_request) = &request.gcp {
            for role in gcp_request {
                all_gcp_requests.push(GcpRequest {
                    access_request_id: inserted.id.clone(),
                    company_id,
                    user: user.email.clone(),
                    // FIXME: Can we do this expect?
                    project: request
                        .gcp_project
                        .as_ref()
                        .expect("gcp_project not provided")
                        .clone(),
                    role: (**role).clone(),
                });
            }
        }

        if let Some(cf_request) = &request.cloudflare {
            for role in cf_request {
                all_cf_requests.push(CloudflareRequest {
                    access_request_id: inserted.id.clone(),
                    company_id,
                    user: user.email.clone(),
                    account_alias: request
                        .cloudflare_account
                        .as_ref()
                        .expect("cloudflare account not provided")
                        .clone(),
                    role: *role,
                });
            }
        }

        if let Some(aws_request) = &request.aws {
            for role in aws_request {
                all_aws_requests.push(AwsRequest {
                    access_request_id: inserted.id.clone(),
                    company_id,
                    user: user.email.clone(),
                    account_alias: request
                        .aws_account
                        .as_ref()
                        .expect("gcp_project not provided")
                        .clone(),
                    role: (**role).clone(),
                });
            }
        }

        if !all_gcp_requests.is_empty() {
            GcpRequest::create(connection, &all_gcp_requests)?;
        }

        if !all_cf_requests.is_empty() {
            CloudflareRequest::create(connection, &all_cf_requests)?;
        }

        if !all_aws_requests.is_empty() {
            AwsRequest::create(connection, &all_aws_requests)?;
        }

        Justification::create(
            connection,
            Justification {
                access_request_id: inserted.id.clone(),
                justification: request.justification.to_string(),
            },
        )?;

        Self::read(connection, &inserted.id)
    }

    pub fn read_all(
        connection: &mut SqliteConnection,
        matching: &Vec<String>,
    ) -> Result<Vec<Request>> {
        let mut all = vec![];

        for id in matching {
            all.push(Self::read(connection, id)?);
        }

        Ok(all)
    }

    #[allow(clippy::too_many_lines)]
    pub fn read(connection: &mut SqliteConnection, id: &str) -> Result<Request> {
        let access_request = AccessRequest::read(connection, id)?;
        let user = User::read_by_email(connection, &access_request.requester)?;
        let user_aliases = user.aliases(connection).unwrap_or(UserAlias {
            user_id: user.id,
            ..UserAlias::default()
        });
        let gcp_requests = GcpRequest::read(connection, id)?;
        let cf_requests = CloudflareRequest::read(connection, id)?;
        let aws_requests = AwsRequest::read(connection, id)?;
        let approvals = Approval::read(connection, id)?;
        let cancellation = Cancellation::read(connection, id).ok();
        let rejection = Rejection::read(connection, id).ok();
        let extensions = Extension::read(connection, id)?;
        let justification = Justification::read(connection, id)?;
        let gcp_project = gcp_requests.first().map(|r| r.project.clone());
        let aws_account = aws_requests.first().map(|r| r.account_alias.clone());
        let cloudflare_account = cf_requests.first().map(|r| r.account_alias.clone());

        let aws_requests: Vec<String> = aws_requests.iter().map(|r| r.role.clone()).collect();
        let gcp_requests: Vec<String> = gcp_requests.iter().map(|r| r.role.clone()).collect();
        let cf_requests: Vec<CloudflareRole> = cf_requests.iter().map(|r| r.role).collect();

        let mut approvals_required = 0;
        let mut admin_approval_required = false;
        if let Some(gcp_project) = &gcp_project {
            let project = CompanyGcpProject::read_by_project(
                connection,
                gcp_project,
                access_request.company_id,
            )?;

            approvals_required = project.approvals_required;
            admin_approval_required = project.admin_approval_required;
        }

        if let Some(aws_account) = &aws_account {
            let account = CompanyAwsAccount::read_by_alias(
                connection,
                aws_account,
                access_request.company_id,
            )?;

            if account.approvals_required > approvals_required {
                approvals_required = account.approvals_required;
            }

            if !admin_approval_required {
                admin_approval_required = account.admin_approval_required;
            }
        }

        let approvals: Vec<_> = approvals
            .iter()
            .map(|r| UserInteraction {
                timestamp: r.timestamp,
                user: r.user.clone(),
            })
            .collect();

        let extensions: Vec<_> = extensions
            .iter()
            .map(|r| UserInteraction {
                timestamp: r.timestamp,
                user: r.user.clone(),
            })
            .collect();

        let requester_aliases = UserAliases {
            aws: user_aliases.aws,
            cloudflare: user_aliases.cloudflare,
            gcp: user_aliases.gcp,
        };

        Ok(Request {
            id: access_request.id,
            company_id: access_request.company_id,
            alias: access_request.req_alias,
            requester: access_request.requester,
            requester_aliases,
            approvals_required,
            admin_approval_required,
            policy: access_request.policy,
            timestamp: access_request.timestamp,
            justification: justification.justification,
            minutes: access_request.duration,
            aws: {
                if aws_requests.is_empty() {
                    None
                } else {
                    Some(aws_requests)
                }
            },
            cloudflare: {
                if cf_requests.is_empty() {
                    None
                } else {
                    Some(cf_requests)
                }
            },
            cloudflare_account,
            gcp: {
                if gcp_requests.is_empty() {
                    None
                } else {
                    Some(gcp_requests)
                }
            },
            gcp_project,
            aws_account,
            approved: access_request.approved,
            state: access_request.state,
            access_expiry: access_request.access_expiry,
            approvals: {
                if approvals.is_empty() {
                    None
                } else {
                    Some(approvals)
                }
            },
            extensions: {
                if extensions.is_empty() {
                    None
                } else {
                    Some(extensions)
                }
            },
            rejection: rejection.map(|r| UserInteraction {
                timestamp: r.timestamp,
                user: r.user,
            }),
            cancellation: cancellation.map(|c| UserInteraction {
                timestamp: c.timestamp,
                user: c.user,
            }),
        })
    }

    pub fn unexpired_and_completed(
        connection: &mut SqliteConnection,
        company_id: i32,
    ) -> Result<Vec<Request>> {
        let mut all = vec![];

        for expired in AccessRequest::unexpired_and_completed(connection, company_id)? {
            all.push(Self::read(connection, &expired.id)?);
        }

        Ok(all)
    }

    pub fn expired_and_active(
        connection: &mut SqliteConnection,
        company_id: i32,
    ) -> Result<Vec<Request>> {
        let mut all = vec![];

        for expired in AccessRequest::expired_and_active(connection, company_id)? {
            all.push(Self::read(connection, &expired.id)?);
        }

        Ok(all)
    }

    pub fn unexpired_and_revoked(
        connection: &mut SqliteConnection,
        company_id: i32,
    ) -> Result<Vec<Request>> {
        let mut all = vec![];

        for expired in AccessRequest::unexpired_and_revoked(connection, company_id)? {
            all.push(Self::read(connection, &expired.id)?);
        }

        Ok(all)
    }

    pub fn approved_and_pending(
        connection: &mut SqliteConnection,
        company_id: i32,
    ) -> Result<Vec<Request>> {
        let mut all = vec![];

        for expired in AccessRequest::approved_and_pending(connection, company_id)? {
            all.push(Self::read(connection, &expired.id)?);
        }

        Ok(all)
    }

    pub fn approve(
        &mut self,
        connection: &mut SqliteConnection,
        user: &User,
        company_id: i32,
    ) -> Result<Request> {
        let now = Utc::now();
        Approval::create(
            connection,
            Approval {
                access_request_id: self.id.clone(),
                user: user.email.clone(),
                timestamp: now,
            },
        )?;

        let mut access_request = AccessRequest::read(connection, &self.id)?;

        let mut approvals_required = 0;
        let mut admin_approval_required = false;
        if let Some(gcp_project) = &self.gcp_project {
            let project = CompanyGcpProject::read_by_project(connection, gcp_project, company_id)?;
            approvals_required = project.approvals_required;
            admin_approval_required = project.admin_approval_required;
        }

        if let Some(aws_account) = &self.aws_account {
            let account = CompanyAwsAccount::read_by_alias(connection, aws_account, company_id)?;
            if account.approvals_required > approvals_required {
                approvals_required = account.approvals_required;
            }

            if !admin_approval_required {
                admin_approval_required = account.admin_approval_required;
            }
        }

        let mut has_met_approvals = false;
        let mut has_met_admin_approval = !admin_approval_required;

        #[allow(clippy::cast_sign_loss)]
        let count = if let Some(approvals) = &self.approvals {
            for approval in approvals {
                if User::read_by_email(connection, &approval.user)?
                    .roles(connection)?
                    .contains(&AccessRole::Administrator)
                {
                    has_met_admin_approval = true;
                }
            }

            approvals.len() + 1
        } else {
            if User::read_by_email(connection, &user.email)?
                .roles(connection)?
                .contains(&AccessRole::Administrator)
            {
                has_met_admin_approval = true;
            }

            1
        };

        #[allow(clippy::cast_sign_loss)]
        if count >= approvals_required as usize {
            has_met_approvals = true;
        }

        if has_met_approvals && has_met_admin_approval {
            access_request.approved = true;
        }

        access_request.modified = now;
        access_request.update(connection)?;

        Self::read(connection, &self.id)
    }

    pub fn extend(
        &mut self,
        connection: &mut SqliteConnection,
        user: &str,
        duration: i32,
    ) -> Result<Request> {
        let now = Utc::now();
        Extension::create(
            connection,
            Extension {
                access_request_id: self.id.clone(),
                user: user.to_string(),
                timestamp: now,
                duration,
            },
        )?;

        let mut access_request = AccessRequest::read(connection, &self.id)?;

        if let Some(expiry) = access_request.access_expiry {
            let new_expiry = expiry + Duration::minutes(i64::from(duration));
            access_request.access_expiry = Option::from(new_expiry);
            access_request.modified = now;
        }

        access_request.update(connection)?;

        Self::read(connection, &self.id)
    }

    pub fn reject(&mut self, connection: &mut SqliteConnection, user: &str) -> Result<Request> {
        let now = Utc::now();

        Rejection::create(
            connection,
            Rejection {
                access_request_id: self.id.clone(),
                user: user.to_string(),
                timestamp: now,
            },
        )?;

        let mut access_request = AccessRequest::read(connection, &self.id)?;

        access_request.modified = now;
        access_request.state = AccessRequestState::Rejected;
        access_request.update(connection)?;

        Self::read(connection, &self.id)
    }

    pub fn cancel(&mut self, connection: &mut SqliteConnection, user: &str) -> Result<Request> {
        let now = Utc::now();

        Cancellation::create(
            connection,
            Cancellation {
                access_request_id: self.id.clone(),
                user: user.to_string(),
                timestamp: now,
            },
        )?;

        let mut access_request = AccessRequest::read(connection, &self.id)?;

        access_request.modified = now;
        access_request.state = AccessRequestState::Cancelled;
        access_request.update(connection)?;

        Self::read(connection, &self.id)
    }

    pub fn complete(&mut self, connection: &mut SqliteConnection) -> Result<Request> {
        let mut access_request = AccessRequest::read(connection, &self.id)?;

        access_request.modified = Utc::now();
        access_request.state = AccessRequestState::Completed;
        access_request.update(connection)?;

        Self::read(connection, &self.id)
    }

    pub fn activate(&mut self, connection: &mut SqliteConnection) -> Result<Request> {
        let mut access_request = AccessRequest::read(connection, &self.id)?;

        let now = Utc::now();
        access_request.modified = now;
        access_request.state = AccessRequestState::Active;
        access_request.access_expiry =
            Option::from(now + Duration::minutes(i64::from(access_request.duration)));

        access_request.update(connection)?;

        Self::read(connection, &self.id)
    }

    pub fn revoke(&mut self, connection: &mut SqliteConnection, user: &str) -> Result<Request> {
        let now = Utc::now();

        Revocation::create(
            connection,
            Revocation {
                access_request_id: self.id.clone(),
                user: user.to_string(),
                timestamp: now,
            },
        )?;

        let mut access_request = AccessRequest::read(connection, &self.id)?;

        access_request.modified = now;
        access_request.state = AccessRequestState::Revoked;
        access_request.update(connection)?;

        Self::read(connection, &self.id)
    }
}
