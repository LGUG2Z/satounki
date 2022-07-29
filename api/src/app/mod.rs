use actix::Addr;
use actix_web::get;
use actix_web::web;
use actix_web::HttpResponse;
use common::AccessRequestState;
use common::AccessRole;
use common::NewPolicy;
use common::Policy;
use common::Request;
use database::AccessRequest;
use database::Company;
use database::CompanyAwsAccount;
use database::CompanyCloudflareAccount;
use database::CompanyGcpProject;
use database::CompanyPolicy;
use database::Pool;
use database::RequestWrapper;
use database::User;
use database::UserWithRoles;
use serde::Deserialize;
use serde::Serialize;
use tera::Context;
use tera::Tera;

use crate::auth::IndividualUser;
use crate::error;
use crate::worker::CheckConnection;
use crate::Result;
use crate::Server;

#[derive(Serialize)]
struct RequestsTemplate {
    requests: Vec<Request>,
}

#[derive(Deserialize)]
pub struct RequestsQuery {
    state: Option<AccessRequestState>,
}

#[get("/requests")]
pub async fn requests(
    pool: web::Data<Pool>,
    authenticated: IndividualUser,
    query: web::Query<RequestsQuery>,
    tera: web::Data<Tera>,
) -> Result<HttpResponse> {
    let connection = &mut *pool.get()?;

    let company = authenticated.company(connection)?;
    let ids = company.access_request_ids(
        connection,
        query.state.unwrap_or(AccessRequestState::Pending),
        20,
    )?;

    let requests = RequestWrapper::read_all(connection, &ids)?;

    let context = RequestsTemplate { requests };

    let rendered = tera
        .render("requests.html", &Context::from_serialize(context).unwrap())
        .unwrap();

    Ok(HttpResponse::Ok().body(rendered))
}

#[get("/request/{alias}")]
pub async fn request(
    pool: web::Data<Pool>,
    authenticated: IndividualUser,
    alias: web::Path<String>,
    tera: web::Data<Tera>,
) -> Result<HttpResponse> {
    let connection = &mut *pool.get()?;
    let company_id = authenticated.company_id(connection)?;
    let access_request = AccessRequest::read_from_alias(connection, &alias, company_id)?;
    let request = RequestWrapper::read(connection, &access_request.id)?;

    if request.company_id != company_id {
        return Err(error::Api::UnauthorizedNotFound);
    }
    let context = request;

    let rendered = tera
        .render("request.html", &Context::from_serialize(context).unwrap())
        .unwrap();

    Ok(HttpResponse::Ok().body(rendered))
}

#[derive(Serialize)]
struct PoliciesTemplate {
    policies: Vec<CompanyPolicy>,
}

#[get("/policies")]
pub async fn policies(
    pool: web::Data<Pool>,
    authenticated: IndividualUser,
    tera: web::Data<Tera>,
) -> Result<HttpResponse> {
    let connection = &mut *pool.get()?;

    let company = authenticated.company(connection)?;
    let policies = company.policies(connection)?;

    let context = PoliciesTemplate { policies };

    let rendered = tera
        .render("policies.html", &Context::from_serialize(context).unwrap())
        .unwrap();

    Ok(HttpResponse::Ok().body(rendered))
}

#[get("/policy/{name}")]
pub async fn policy(
    pool: web::Data<Pool>,
    authenticated: IndividualUser,
    name: web::Path<String>,
    tera: web::Data<Tera>,
) -> Result<HttpResponse> {
    let connection = &mut *pool.get()?;
    let company_id = authenticated.company_id(connection)?;
    let policy = CompanyPolicy::read_by_name(connection, &name, company_id)?;

    if policy.company_id != company_id {
        return Err(error::Api::UnauthorizedNotFound);
    }

    let partially_parsed: NewPolicy = serde_json::from_str(&policy.policy)?;

    let context = Policy {
        id: policy.id,
        name: policy.name,
        description: policy.description,
        gcp: partially_parsed.gcp,
        aws: partially_parsed.aws,
        cloudflare: partially_parsed.cloudflare,
    };

    let rendered = tera
        .render("policy.html", &Context::from_serialize(context).unwrap())
        .unwrap();

    Ok(HttpResponse::Ok().body(rendered))
}

#[derive(Serialize)]
struct UsersTemplate {
    users: Vec<UserWithRoles>,
}

#[derive(Deserialize)]
pub struct UsersQuery {
    role: Option<AccessRole>,
    active: Option<bool>,
}

#[get("/users")]
pub async fn users(
    pool: web::Data<Pool>,
    authenticated: IndividualUser,
    query: web::Query<UsersQuery>,
    tera: web::Data<Tera>,
) -> Result<HttpResponse> {
    let connection = &mut *pool.get()?;

    let company = authenticated.company(connection)?;
    let mut users = company.users_with_roles(connection)?;

    if let Some(role) = &query.role {
        users.retain(|user: &UserWithRoles| user.roles.contains(role));
    }

    if let Some(active) = &query.active {
        users.retain(|user: &UserWithRoles| user.active == *active);
    }

    let context = UsersTemplate { users };

    let rendered = tera
        .render("users.html", &Context::from_serialize(context).unwrap())
        .unwrap();

    Ok(HttpResponse::Ok().body(rendered))
}

#[derive(Serialize)]
struct ServicesTemplate {
    aws: Vec<CompanyAwsAccount>,
    cloudflare: Vec<CompanyCloudflareAccount>,
    gcp: Vec<CompanyGcpProject>,
}

#[get("/services")]
pub async fn services(
    pool: web::Data<Pool>,
    authenticated: IndividualUser,
    tera: web::Data<Tera>,
) -> Result<HttpResponse> {
    let connection = &mut *pool.get()?;

    let company = authenticated.company(connection)?;
    let aws = company.aws_accounts(connection)?;
    let cloudflare = company.cloudflare_accounts(connection)?;
    let gcp = company.gcp_projects(connection)?;

    let context = ServicesTemplate {
        aws,
        cloudflare,
        gcp,
    };

    let rendered = tera
        .render("services.html", &Context::from_serialize(context).unwrap())
        .unwrap();

    Ok(HttpResponse::Ok().body(rendered))
}

#[derive(Serialize)]
struct DashboardTemplate {
    company: Company,
    user: User,
    roles: Vec<AccessRole>,
    policies: usize,
    users: usize,
    services: usize,
    worker_connected: bool,
}

#[get("/")]
pub async fn dashboard(
    pool: web::Data<Pool>,
    authenticated: IndividualUser,
    websocket: web::Data<Addr<Server>>,
    tera: web::Data<Tera>,
) -> Result<HttpResponse> {
    let connection = &mut *pool.get()?;
    let user = authenticated.user(connection)?;
    let company = authenticated.company(connection)?;
    let company_policies = company.policies(connection)?;

    let aws = company.aws_accounts(connection)?;
    let cloudflare = company.cloudflare_accounts(connection)?;
    let gcp = company.gcp_projects(connection)?;
    let company_users = company.users(connection)?;
    let roles = user.roles(connection)?;

    let worker_connected = websocket
        .send(CheckConnection {
            company_domain: company.domain.clone(),
        })
        .await
        .unwrap_or(false);

    let context = DashboardTemplate {
        company,
        user,
        roles,
        policies: company_policies.len(),
        services: aws.len() + cloudflare.len() + gcp.len(),
        users: company_users.len(),
        worker_connected,
    };

    let rendered = tera
        .render("dashboard.html", &Context::from_serialize(context).unwrap())
        .unwrap();

    Ok(HttpResponse::Ok().body(rendered))
}

#[derive(Serialize)]
struct SettingsTemplate {
    company: Company,
    user_token: String,
    api_token: Option<String>,
}

#[get("/settings")]
pub async fn settings(
    pool: web::Data<Pool>,
    authenticated: IndividualUser,
    tera: web::Data<Tera>,
) -> Result<HttpResponse> {
    let connection = &mut *pool.get()?;
    let company = authenticated.company(connection)?;
    let user = authenticated.user(connection)?;

    let api_token = if user.is_administrator(connection)? {
        Option::from(company.api_token(connection)?.token)
    } else {
        None
    };

    let user_token = user.user_token(connection)?.token;

    let context = SettingsTemplate {
        company,
        user_token,
        api_token,
    };

    let rendered = tera
        .render("settings.html", &Context::from_serialize(context).unwrap())
        .unwrap();

    Ok(HttpResponse::Ok().body(rendered))
}
