use actix::fut::ready;
use actix::fut::Ready;
use actix_web::dev::Payload;
use actix_web::web;
use actix_web::FromRequest;
use actix_web::HttpMessage;
use actix_web::HttpRequest;
use actix_web_httpauth::extractors::bearer::BearerAuth;
use color_eyre::eyre::anyhow;
use database::PlatformToken;
use database::Pool;

use crate::auth::platform_token_scope::PlatformTokenScope;
use crate::error;

pub struct PlatformTokenWithScope<S: PlatformTokenScope> {
    phantom_data: std::marker::PhantomData<S>,
}

impl<S: PlatformTokenScope> FromRequest for PlatformTokenWithScope<S> {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let token = match req.extensions().get::<BearerAuth>() {
            None => return ready(Err(error::Api::UnauthorizedPlatformTokenRequired.into())),
            Some(credentials) => credentials.token().to_string(),
        };

        let mut connection = match req.app_data::<web::Data<Pool>>() {
            None => return ready(Err(error::Api::Other(anyhow!("database error")).into())),
            Some(pool) => match pool.get() {
                Ok(connection) => connection,
                Err(error) => return ready(Err(error::Api::DatabaseConnection(error).into())),
            },
        };

        if let Ok(platform_token) = PlatformToken::read(&mut connection, &token) {
            if platform_token.scope == S::as_enum()
                || platform_token.scope == common_platform::PlatformTokenScope::Write
            {
                return ready(Ok(Self {
                    phantom_data: std::marker::PhantomData,
                }));
            }
        }

        ready(Err(error::Api::UnauthorizedPlatformScopeRequired(
            S::as_str(),
        )
        .into()))
    }
}
