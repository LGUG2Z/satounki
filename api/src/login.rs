use std::collections::HashMap;

use actix_session::Session;
use actix_web::get;
use actix_web::http::header::LOCATION;
use actix_web::web;
use actix_web::HttpResponse;
use common::AccessRole;
use database::Company;
use database::CompanySlack;
use database::NewCompany;
use database::NewUser;
use database::Pool;
use database::User;
use openidconnect::core::CoreClient;
use openidconnect::core::CoreResponseType;
use openidconnect::reqwest::async_http_client;
use openidconnect::AuthenticationFlow;
use openidconnect::AuthorizationCode;
use openidconnect::CsrfToken;
use openidconnect::Nonce;
use openidconnect::PkceCodeChallenge;
use openidconnect::Scope;
use serde::Deserialize;
use serde::Serialize;

use crate::error;
use crate::Result;

#[get("/login/google")]
pub async fn login_google(
    session: Session,
    oidc_client: web::Data<CoreClient>,
) -> Result<HttpResponse> {
    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

    let (authorize_url, csrf_state, nonce) = &oidc_client
        .authorize_url(
            AuthenticationFlow::<CoreResponseType>::AuthorizationCode,
            CsrfToken::new_random,
            Nonce::new_random,
        )
        .add_scope(Scope::new("openid".to_string()))
        .add_scope(Scope::new("profile".to_string()))
        .add_scope(Scope::new("email".to_string()))
        // .add_scope(Scope::new(
        //     "https://www.googleapis.com/auth/admin.directory.group.readonly".to_string(),
        // ))
        .set_pkce_challenge(pkce_challenge)
        .url();

    session.insert("csrf_state", csrf_state)?;
    session.insert("nonce", nonce)?;
    session.insert("pkce_verifier", pkce_verifier)?;

    Ok(HttpResponse::Found()
        .append_header((LOCATION, authorize_url.as_str()))
        .finish())
}

#[derive(Deserialize)]
pub struct AuthRequest {
    code: String,
    state: String,
    scope: String,
}

#[derive(Debug, Deserialize)]
pub struct SlackAuthRequest {
    code: String,
    state: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SlackAuthResponse {
    pub ok: bool,
    pub app_id: String,
    pub authed_user: AuthedUser,
    pub scope: String,
    pub token_type: String,
    pub access_token: String,
    pub bot_user_id: String,
    pub team: Team,
    pub is_enterprise_install: bool,
    pub incoming_webhook: IncomingWebhook,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthedUser {
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Team {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IncomingWebhook {
    pub channel: String,
    pub channel_id: String,
    pub configuration_url: String,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SlackUserResponse {
    pub ok: bool,
    pub profile: Profile,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Profile {
    pub email: String,
}

#[get("/redirect/slack")]
pub async fn redirect_slack(
    pool: web::Data<Pool>,
    params: web::Query<SlackAuthRequest>,
) -> Result<HttpResponse> {
    let code = AuthorizationCode::new(params.code.clone());
    let _state = CsrfToken::new(params.state.clone());

    let mut params = HashMap::new();
    params.insert("code", code.secret().to_string());
    params.insert("client_id", std::env::var("SLACK_CLIENT_ID").unwrap());
    params.insert(
        "client_secret",
        std::env::var("SLACK_CLIENT_SECRET").unwrap(),
    );

    let client = reqwest::Client::new();

    let auth_response = client
        .post("https://slack.com/api/oauth.v2.access")
        .form(&params)
        .send()
        .await?
        .json::<SlackAuthResponse>()
        .await?;

    let profile_response = client
        .get("https://slack.com/api/users.profile.get")
        .bearer_auth(&auth_response.access_token)
        .query(&[("user", &auth_response.authed_user.id)])
        .send()
        .await?
        .json::<SlackUserResponse>()
        .await?;

    let split: Vec<_> = profile_response.profile.email.split('@').collect();
    let domain = split[1];

    let connection = &mut *pool.get()?;

    if let Some(company) = Company::read_by_domain(connection, domain)? {
        CompanySlack::create(
            connection,
            CompanySlack {
                company_id: company.id,
                access_token: auth_response.access_token,
                team_id: auth_response.team.id,
                team_name: auth_response.team.name,
                channel_id: auth_response.incoming_webhook.channel_id,
                incoming_webhook: auth_response.incoming_webhook.url,
            },
        )?;
    }

    Ok(HttpResponse::Found()
        .append_header((LOCATION, "/".to_string()))
        .finish())
}

#[allow(clippy::too_many_lines)]
#[get("/redirect/google")]
pub async fn redirect_google(
    pool: web::Data<Pool>,
    session: Session,
    oidc_client: web::Data<CoreClient>,
    params: web::Query<AuthRequest>,
) -> Result<HttpResponse> {
    let code = AuthorizationCode::new(params.code.clone());
    let state = CsrfToken::new(params.state.clone());
    let _scope = Scope::new(params.scope.clone());

    let token = &oidc_client
        .exchange_code(code)
        .set_pkce_verifier(session.get("pkce_verifier")?.ok_or_else(|| {
            error::Api::Login("pkce_verifier not found in session cookie".to_string())
        })?)
        .request_async(async_http_client)
        .await
        .map_err(|error| error::Api::Login(error.to_string()))?;

    let id_token = token
        .extra_fields()
        .id_token()
        .ok_or_else(|| error::Api::Login("no id_token received from oidc".to_string()))?;

    let csrf: CsrfToken = session
        .get("csrf_state")?
        .ok_or_else(|| error::Api::Login("csrf_state not found in session cookie".to_string()))?;

    if state.secret() != csrf.secret() {
        return Err(error::Api::Login("csrf tokens do not match".to_string()));
    }

    let claims = id_token
        .claims(
            &oidc_client.id_token_verifier(),
            &session.get("nonce")?.unwrap_or_else(Nonce::new_random),
        )
        .map_err(|error| error::Api::Login(error.to_string()))?;

    let first_name: String = (*claims.given_name().unwrap().get(None).unwrap()).to_string();
    let last_name: String = (*claims.family_name().unwrap().get(None).unwrap()).to_string();
    let email: String = (*claims.email().unwrap()).to_string();

    log::info!(
        "received user claims: email: {email}, given_name: {first_name}, family_name: {last_name}"
    );

    let split: Vec<_> = email.split('@').collect();
    let domain = split[1];

    log::info!("company domain identified from email: {domain}");

    let connection = &mut *pool.get()?;

    #[allow(clippy::single_match_else)]
    let (company, user) = match Company::read_by_domain(connection, domain)? {
        Some(company) => {
            log::info!(
                "company identified from domain: {}",
                serde_json::to_string(&company)?
            );

            let users = company.users(connection)?;

            log::info!("company user count: {}", users.len());

            let user = if let Ok(mut user) = User::read_by_email(connection, &email) {
                log::info!(
                    "user identified from email: {}",
                    serde_json::to_string(&user)?
                );

                if user.first_name != first_name || user.last_name != last_name {
                    user.first_name = first_name;
                    user.last_name = last_name;
                    user.update(connection)?;
                }

                user
            } else {
                log::info!("could not identify user from email, creating new user");
                let user = User::create(
                    connection,
                    &NewUser {
                        email: email.clone(),
                        first_name,
                        last_name,
                        active: true,
                    },
                )?;

                log::info!(
                    "user identified from email: {}",
                    serde_json::to_string(&user)?
                );

                user
            };

            let mut is_assigned_to_company = false;

            for u in &users {
                if u.email == user.email {
                    is_assigned_to_company = true;
                }
            }

            if is_assigned_to_company {
                log::info!("user is assigned to a company, logging in and setting session cookie");
            } else {
                log::info!(
                    "user is not assigned to a company, assigning to company with domain: {domain}"
                );

                company.add_user(connection, &user)?;

                if users.is_empty() {
                    log::info!(
                        "company has no users, assigning user, approver and administrator roles"
                    );
                    company.assign_role(connection, &user, &AccessRole::Administrator)?;
                    company.assign_role(connection, &user, &AccessRole::Approver)?;
                    company.assign_role(connection, &user, &AccessRole::User)?;
                } else {
                    log::info!("granting 'user' role");
                    company.assign_role(connection, &user, &AccessRole::User)?;
                }
            }

            (company, user)
        }
        None => {
            log::info!("could not identify company, creating new user");
            let user = User::create(
                connection,
                &NewUser {
                    email: email.clone(),
                    first_name,
                    last_name,
                    active: true,
                },
            )?;

            log::info!("creating company for domain {domain}");
            let company = Company::create(
                connection,
                &NewCompany {
                    name: domain.to_string(),
                    domain: domain.to_string(),
                    root_user: user.email.clone(),
                },
            )?;

            company.add_user(connection, &user)?;
            company.assign_role(connection, &user, &AccessRole::Administrator)?;
            company.assign_role(connection, &user, &AccessRole::Approver)?;
            company.assign_role(connection, &user, &AccessRole::User)?;

            (company, user)
        }
    };

    session.renew();
    session.insert("user_id", user.id)?;
    session.insert("company_id", company.id)?;

    log::info!("redirecting user");

    Ok(HttpResponse::Found()
        .append_header((LOCATION, "/".to_string()))
        .finish())
}
