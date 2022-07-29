#![warn(clippy::all, clippy::nursery, clippy::pedantic)]
#![allow(clippy::missing_errors_doc, clippy::wildcard_imports)]

#[macro_use]
extern crate diesel_autoincrement_new_struct;

use std::time::Duration;

pub use access_request::AccessRequest;
pub use api_token::ApiToken;
pub use approval::Approval;
pub use aws_request::AwsRequest;
pub use cancellation::Cancellation;
pub use cloudflare_request::CloudflareRequest;
pub use company::Company;
pub use company::New as NewCompany;
pub use company_aws_account::CompanyAwsAccount;
pub use company_cloudflare_account::CompanyCloudflareAccount;
pub use company_gcp_project::CompanyGcpProject;
pub use company_policy::CompanyPolicy;
pub use company_slack::CompanySlack;
use diesel::connection::SimpleConnection;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::CustomizeConnection;
pub use diesel::r2d2::PoolError;
pub use diesel::result::Error as DieselError;
pub use diesel::SqliteConnection;
pub use extension::Extension;
pub use gcp_request::GcpRequest;
pub use justification::Justification;
pub use platform_token::PlatformToken;
pub use rejection::Rejection;
pub use request::NewFromPolicy as NewRequestFromPolicy;
pub use request::Wrapper as RequestWrapper;
pub use request_slack::RequestSlack;
pub use revocation::Revocation;
pub use user::New as NewUser;
pub use user::User;
pub use user::WithRoles as UserWithRoles;
pub use user_alias::UserAlias;
pub use user_token::UserToken;
pub use worker_key::WorkerKey;

mod access_request;
mod api_token;
mod approval;
mod aws_request;
mod cancellation;
mod cloudflare_request;
mod company;
mod company_aws_account;
mod company_cloudflare_account;
mod company_gcp_project;
mod company_policy;
mod company_role;
mod company_slack;
mod extension;
mod gcp_request;
mod justification;
mod platform_token;
mod rejection;
mod request;
mod request_slack;
mod revocation;
mod schema;
mod user;
mod user_alias;
mod user_company;
mod user_company_role;
mod user_token;
mod worker_key;

pub type Pool = diesel::r2d2::Pool<ConnectionManager<SqliteConnection>>;
type Result<T> = core::result::Result<T, diesel::result::Error>;

#[derive(Debug)]
pub struct ConnectionOptions {
    pub enable_wal: bool,
    pub enable_foreign_keys: bool,
    pub busy_timeout: Option<Duration>,
}

impl CustomizeConnection<SqliteConnection, diesel::r2d2::Error> for ConnectionOptions {
    fn on_acquire(
        &self,
        conn: &mut SqliteConnection,
    ) -> std::result::Result<(), diesel::r2d2::Error> {
        (|| {
            if self.enable_wal {
                conn.batch_execute("PRAGMA journal_mode = WAL; PRAGMA synchronous = NORMAL;")?;
            }
            if self.enable_foreign_keys {
                conn.batch_execute("PRAGMA foreign_keys = ON;")?;
            }
            if let Some(d) = self.busy_timeout {
                conn.batch_execute(&format!("PRAGMA busy_timeout = {};", d.as_millis()))?;
            }
            Ok(())
        })()
        .map_err(diesel::r2d2::Error::QueryError)
    }
}

pub fn new_pool(db_url: &str) -> color_eyre::Result<Pool> {
    Ok(Pool::builder()
        .max_size(8)
        .connection_customizer(Box::new(ConnectionOptions {
            enable_wal: true,
            enable_foreign_keys: true,
            busy_timeout: Some(Duration::from_secs(30)),
        }))
        .build(ConnectionManager::<SqliteConnection>::new(db_url))?)
}
