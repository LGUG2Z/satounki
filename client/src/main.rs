#![warn(clippy::all, clippy::nursery, clippy::pedantic)]
#![allow(clippy::missing_errors_doc)]

use clap::Parser;
use serde::Deserialize;

use crate::cli::Cli;
use crate::client::Client;
use crate::configuration::Configuration;

mod cli;
mod client;
mod configuration;
mod request;

pub const SKIP_CHECKS: bool = true;
pub const DRY_RUN: bool = true;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }

    env_logger::init();
    color_eyre::install()?;

    let cli = Cli::parse();
    let configuration: Configuration =
        serde_yaml::from_str(&std::fs::read_to_string(&cli.config)?)?;

    let company_domain =
        std::env::var("COMPANY_DOMAIN").unwrap_or_else(|_| "satounki.com".to_string());
    let company_worker_key = std::env::var("COMPANY_WORKER_KEY")
        .unwrap_or_else(|_| "swk-e0c43bd0-38a4-4e7b-9c0f-8bd5f47f20d2".to_string());

    let mut client = Client::new(company_domain, company_worker_key, configuration).await?;
    client.listen().await?;

    Ok(())
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct GcpServiceAccount {
    #[serde(rename = "type")]
    pub type_field: String,
    pub project_id: String,
    pub private_key_id: String,
    pub private_key: String,
    pub client_email: String,
    pub client_id: String,
    pub auth_uri: String,
    pub token_uri: String,
    pub auth_provider_x509_cert_url: String,
    pub client_x509_cert_url: String,
}
