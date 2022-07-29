use actix_web::dev::ServiceRequest;
use actix_web::HttpMessage;
use actix_web_httpauth::extractors::bearer::BearerAuth;

pub async fn validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (actix_web::Error, ServiceRequest)> {
    req.extensions_mut().insert(credentials);
    Ok(req)
}
