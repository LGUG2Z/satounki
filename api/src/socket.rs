use actix::Addr;
use actix_web::get;
use actix_web::web;
use actix_web::HttpRequest;
use actix_web::HttpResponse;
use actix_web_actors::ws;
use database::Company;
use database::Pool;
use database::WorkerKey;
use serde::Deserialize;
use serde::Serialize;

use crate::error;
use crate::worker;
use crate::Result;
use crate::WORKER_SESSIONS;

#[derive(Deserialize)]
pub struct WorkerCredentials {
    company_domain: String,
    company_worker_key: String,
}

#[get("/worker")]
pub async fn ws_worker(
    pool: web::Data<Pool>,
    req: HttpRequest,
    stream: web::Payload,
    server_addr: web::Data<Addr<worker::Server>>,
    params: web::Query<WorkerCredentials>,
) -> Result<HttpResponse> {
    let company = Company::read_by_domain(&mut *pool.get()?, &params.company_domain)?
        .ok_or_else(|| error::Api::Authentication("company domain not found".to_string()))?;

    let worker_key = WorkerKey::read(&mut *pool.get()?, company.id)?.ok_or_else(|| {
        error::Api::Authentication("worker key not found for this company".to_string())
    })?;

    if worker_key.key != params.company_worker_key {
        log::info!("[{}] company worker key is invalid", params.company_domain);

        return Err(error::Api::Authentication(
            "company worker key is invalid".to_string(),
        ));
    }

    log::info!("[{}] company worker key is valid", params.company_domain);

    if WORKER_SESSIONS.lock().contains_key(&params.company_domain) {
        log::info!(
            "[{}] existing worker websocket connection found, rejecting request",
            params.company_domain
        );

        return Err(error::Api::TooManyWorkers);
    }

    log::info!(
        "[{}] no existing worker websocket connection found, permitting request",
        params.company_domain
    );

    log::info!(
        "[{}] starting worker websocket session",
        params.company_domain
    );

    let response: HttpResponse = ws::start(
        worker::Session::new(
            company,
            server_addr.get_ref().clone(),
            pool.get_ref().clone(),
        ),
        &req,
        stream,
    )?;

    Ok(response)
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkerStatus {
    Connected,
    Disconnected,
}

#[get("/status")]
pub async fn status(
    _pool: web::Data<Pool>,
    _websocket: web::Data<Addr<worker::Server>>,
) -> Result<web::Json<WorkerStatus>> {
    if WORKER_SESSIONS.lock().len() == 1 {
        Ok(web::Json(WorkerStatus::Connected))
    } else {
        Ok(web::Json(WorkerStatus::Disconnected))
    }
}
