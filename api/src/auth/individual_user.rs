use actix::fut::ready;
use actix::fut::Ready;
use actix_web::dev::Payload;
use actix_web::FromRequest;
use actix_web::HttpRequest;
use derive_more::Deref;

use crate::auth::authenticated::Authenticated;
use crate::error;

#[derive(Deref)]
pub struct IndividualUser(pub Authenticated);

impl FromRequest for IndividualUser {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        if let Ok(auth) = Authenticated::from_request(req, payload).into_inner() {
            if matches!(
                auth,
                Authenticated::Cookie { .. } | Authenticated::UserToken { .. }
            ) {
                return ready(Ok(Self(auth)));
            }
        }

        ready(Err(error::Api::UnauthorizedUserCredentialsRequired.into()))
    }
}
