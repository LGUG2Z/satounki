#![warn(clippy::all, clippy::nursery, clippy::pedantic)]
#![allow(
    unknown_lints,
    clippy::missing_errors_doc,
    clippy::use_self,
    clippy::unused_async,
    clippy::explicit_auto_deref,
    clippy::future_not_send
)]

use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;

use actix::Actor;
use actix::Recipient;
use actix_session::config::BrowserSession;
use actix_session::config::CookieContentSecurity;
use actix_session::storage::CookieSessionStore;
use actix_session::SessionMiddleware;
use actix_web::cookie::Key;
use actix_web::cookie::SameSite;
use actix_web::middleware::Logger;
use actix_web::web;
use actix_web::App;
use actix_web::HttpResponse;
use actix_web::HttpServer;
use actix_web_httpauth::middleware::HttpAuthentication;
use common_platform::PlatformTokenScope;
use database::DieselError;
use database::PlatformToken;
use diesel_migrations::embed_migrations;
use diesel_migrations::EmbeddedMigrations;
use diesel_migrations::MigrationHarness;
use dotenv::dotenv;
use lazy_static::lazy_static;
use log::info;
use login::redirect_slack;
use oauth2::reqwest::async_http_client;
use oauth2::ClientId;
use oauth2::ClientSecret;
use oauth2::RedirectUrl;
use openidconnect::core::CoreClient;
use openidconnect::core::CoreProviderMetadata;
use openidconnect::IssuerUrl;
use parking_lot::Mutex;
use rolescraper::Aws as RoleScraperAws;
use rolescraper::Gcp as RoleScraperGcp;
use tera::Tera;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use worker::Message;
use worker::Server;

use crate::company::company_delete;
use crate::company::company_get;
use crate::company::company_post;
use crate::company::company_put;
use crate::login::login_google;
use crate::login::redirect_google;
use crate::platform_doc::PlatformDoc;
use crate::platform_token::platform_token_get;
use crate::platform_token::platform_token_put;
use crate::policies::policies_get;
use crate::policy::policy_delete;
use crate::policy::policy_get;
use crate::policy::policy_name_get;
use crate::policy::policy_post;
use crate::policy::policy_put;
use crate::public_doc::PublicDoc;
use crate::request_alias::request_alias_get;
use crate::request_alias::request_alias_patch;
use crate::request_policy::request_policy_post;
use crate::requests::requests_get;
use crate::settings_aws_account::settings_aws_account_delete;
use crate::settings_aws_account::settings_aws_account_get;
use crate::settings_aws_account::settings_aws_account_post;
use crate::settings_aws_account::settings_aws_account_put;
use crate::settings_aws_accounts::settings_aws_accounts_get;
use crate::settings_cf_account::settings_cf_account_delete;
use crate::settings_cf_account::settings_cf_account_get;
use crate::settings_cf_account::settings_cf_account_post;
use crate::settings_cf_account::settings_cf_account_put;
use crate::settings_cf_accounts::settings_cf_accounts_get;
use crate::settings_gcp_project::settings_gcp_project_delete;
use crate::settings_gcp_project::settings_gcp_project_get;
use crate::settings_gcp_project::settings_gcp_project_post;
use crate::settings_gcp_project::settings_gcp_project_put;
use crate::settings_gcp_projects::settings_gcp_projects_get;
use crate::settings_token::settings_token_get;
use crate::settings_token::settings_token_put;
use crate::socket::status;
use crate::socket::ws_worker;
use crate::token_validator::validator;
use crate::user::user_aliases_delete;
use crate::user::user_aliases_get;
use crate::user::user_aliases_post;
use crate::user::user_aliases_put;
use crate::user::user_disable_patch;
use crate::user::user_enable_patch;
use crate::user::user_roles_get;
use crate::user::user_roles_post;
use crate::user::user_roles_put;
use crate::user::user_status_get;
use crate::user_token::user_token_get;
use crate::user_token::user_token_put;

mod app;
mod auth;
mod company;
mod error;
mod login;
mod platform_doc;
mod platform_token;
mod policies;
mod policy;
mod public_doc;
mod request_alias;
mod request_policy;
mod requests;
mod rolescraper;
mod settings_aws_account;
mod settings_aws_accounts;
mod settings_cf_account;
mod settings_cf_accounts;
mod settings_gcp_project;
mod settings_gcp_projects;
mod settings_token;
mod slack_integration;
mod socket;
mod token_validator;
mod user;
mod user_token;
mod worker;

lazy_static! {
    static ref WORKER_SESSIONS: Arc<Mutex<HashMap<String, Recipient<Message>>>> =
        Arc::new(Mutex::new(HashMap::new()));
    static ref AWS_ROLES: Vec<RoleScraperAws> = {
        let scraped = std::fs::read_to_string("rolescraper_aws.json").unwrap();
        let parsed: Vec<RoleScraperAws> = serde_json::from_str(&scraped).unwrap();
        parsed
    };
    static ref GCP_ROLES: Vec<RoleScraperGcp> = {
        let scraped = std::fs::read_to_string("rolescraper_gcp.json").unwrap();
        let parsed: Vec<RoleScraperGcp> = serde_json::from_str(&scraped).unwrap();
        parsed
    };
    static ref SATOUNKI_URL: String =
        std::env::var("SATOUNKI_URL").unwrap_or_else(|_| String::from("http://localhost:8080"));
    static ref DATABASE_URL: String =
        std::env::var("DATABASE_URL").unwrap_or_else(|_| String::from("dev.db"));
}

type Result<T> = std::result::Result<T, error::Api>;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

#[actix_web::main]
async fn main() -> color_eyre::Result<()> {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
        std::env::set_var("RUST_BACKTRACE", "1");
    }

    dotenv()?;
    env_logger::init();
    color_eyre::install()?;

    let v1 = PublicDoc::openapi();
    let platform = PlatformDoc::openapi();

    let pool = database::new_pool(&DATABASE_URL)?;

    {
        let connection = &mut *pool.get()?;
        connection.run_pending_migrations(MIGRATIONS).unwrap();

        if PlatformToken::read_by_scope(connection, PlatformTokenScope::Write).is_err() {
            let platform_token =
                if let Ok(platform_token) = std::env::var("SATOUNKIPLATFORM_API_TOKEN") {
                    PlatformToken::create_from_env(
                        connection,
                        PlatformTokenScope::Write,
                        &platform_token,
                    )?
                    .token
                } else {
                    PlatformToken::create(connection, PlatformTokenScope::Write)?.token
                };

            info!("created a new platform api token with the 'write' scope: {platform_token}");
        }
    }

    let sessions = WORKER_SESSIONS.clone();
    let ws_server = Server { sessions }.start();

    let google_client_id = ClientId::new(std::env::var("GOOGLE_CLIENT_ID")?);
    let google_client_secret = ClientSecret::new(std::env::var("GOOGLE_CLIENT_SECRET")?);
    let google_redirect_url = format!("{}/redirect/google", *SATOUNKI_URL);

    let issuer_url = IssuerUrl::new("https://accounts.google.com".to_string())?;
    let provider_metadata =
        CoreProviderMetadata::discover_async(issuer_url, async_http_client).await?;

    let client: CoreClient = CoreClient::from_provider_metadata(
        provider_metadata,
        google_client_id,
        Some(google_client_secret),
    )
    .set_redirect_uri(RedirectUrl::new(google_redirect_url)?);

    let secret_key = Key::from(std::env::var("SECRET_KEY")?.as_bytes());

    let templates_dir = if Path::new("/templates").is_dir() {
        "/templates/**/*"
    } else {
        concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")
    };

    let tera = Tera::new(templates_dir)?;
    let api_token_auth = HttpAuthentication::bearer(validator);
    Ok(HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(tera.clone()))
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(ws_server.clone()))
            .app_data(web::Data::new(client.clone()))
            .service(
                SwaggerUi::new("/doc/platform/swagger-ui/{_:.*}")
                    .url("/api-doc/platform.json", platform.clone()),
            )
            .service(
                SwaggerUi::new("/doc/v1/swagger-ui/{_:.*}").url("/api-doc/v1.json", v1.clone()),
            )
            .route(
                "/health",
                web::get().to(|| async { HttpResponse::Ok().finish() }),
            )
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), secret_key.clone())
                    .cookie_name("satounki_session_id".to_string())
                    .cookie_secure(true)
                    .session_lifecycle(BrowserSession::default())
                    .cookie_same_site(SameSite::Lax)
                    .cookie_path("/".to_string())
                    .cookie_content_security(CookieContentSecurity::Private)
                    .build(),
            )
            .service(slack_integration::slack_post)
            .service(app::dashboard)
            .service(app::settings)
            .service(app::requests)
            .service(app::request)
            .service(app::policies)
            .service(app::policy)
            .service(app::users)
            .service(app::services)
            .service(redirect_google)
            .service(redirect_slack)
            .service(login_google)
            .service(
                web::scope("/platform")
                    .wrap(api_token_auth.clone())
                    .service(platform_token_get)
                    .service(platform_token_put)
                    .service(company_post)
                    .service(company_put)
                    .service(company_get)
                    .service(company_delete),
            )
            .service(web::scope("/socket").service(ws_worker).service(status))
            .service(
                web::scope("/v1")
                    .wrap(api_token_auth.clone())
                    .service(requests_get)
                    .service(
                        web::scope("/request")
                            .service(
                                web::scope("/alias")
                                    .service(request_alias_get)
                                    .service(request_alias_patch),
                            )
                            .service(web::scope("/policy").service(request_policy_post)),
                    )
                    .service(policies_get)
                    .service(
                        web::scope("/settings")
                            .service(settings_token_get)
                            .service(settings_token_put)
                            .service(settings_aws_accounts_get)
                            .service(settings_aws_account_post)
                            .service(settings_aws_account_get)
                            .service(settings_aws_account_put)
                            .service(settings_aws_account_delete)
                            .service(settings_cf_accounts_get)
                            .service(settings_cf_account_post)
                            .service(settings_cf_account_get)
                            .service(settings_cf_account_put)
                            .service(settings_cf_account_delete)
                            .service(settings_gcp_projects_get)
                            .service(settings_gcp_project_post)
                            .service(settings_gcp_project_get)
                            .service(settings_gcp_project_put)
                            .service(settings_gcp_project_delete),
                    )
                    .service(
                        web::scope("/policy")
                            .service(policy_get)
                            .service(policy_name_get)
                            .service(policy_post)
                            .service(policy_delete)
                            .service(policy_put),
                    )
                    .service(
                        web::scope("/user")
                            .service(user_status_get)
                            .service(user_disable_patch)
                            .service(user_enable_patch)
                            .service(user_token_get)
                            .service(user_token_put)
                            .service(user_aliases_put)
                            .service(user_aliases_post)
                            .service(user_aliases_put)
                            .service(user_aliases_get)
                            .service(user_aliases_delete)
                            .service(user_roles_post)
                            .service(user_roles_put)
                            .service(user_roles_get),
                    ),
            )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await?)
}
