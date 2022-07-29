use std::ops::Deref;

use actix::fut::ready;
use actix::fut::Ready;
use actix_web::dev::Payload;
use actix_web::FromRequest;
use actix_web::HttpRequest;

use crate::auth::authenticated::Authenticated;
use crate::error;

#[derive(Clone)]
pub struct ApiToken(pub Authenticated);

impl Deref for ApiToken {
    type Target = Authenticated;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromRequest for ApiToken {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        match Authenticated::from_request(req, payload).into_inner() {
            Ok(auth) => {
                if matches!(auth, Authenticated::ApiToken { .. }) {
                    ready(Ok(Self(auth)))
                } else {
                    ready(Err(error::Api::UnauthorizedApiTokenRequired.into()))
                }
            }
            Err(error) => ready(Err(error)),
        }
    }
}
