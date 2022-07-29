use actix::fut::ready;
use actix::fut::Ready;
use actix_web::dev::Payload;
use actix_web::Either;
use actix_web::FromRequest;
use actix_web::HttpRequest;
use oauth2::http::StatusCode;

use crate::auth;
use crate::auth::access_role::AccessRole;
use crate::auth::authenticated::Authenticated;
use crate::auth::user_with_access_role::UserWithAccessRole;
use crate::error;

#[derive(derive_more::Deref)]
pub struct ApiTokenOrUserWithAccessRole<R: AccessRole>(
    Either<auth::ApiToken, UserWithAccessRole<R>>,
);

impl<R: AccessRole> ApiTokenOrUserWithAccessRole<R> {
    pub const fn information(&self) -> &Authenticated {
        match &self.0 {
            Either::Left(token) => &token.0,
            Either::Right(user) => &user.authenticated,
        }
    }
}

impl<R: AccessRole> FromRequest for ApiTokenOrUserWithAccessRole<R> {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        let mut decision = Err(error::Api::UnauthorizedApiTokenOrRoleRequired(R::as_str()).into());

        match auth::ApiToken::from_request(req, payload).into_inner() {
            Ok(auth) => {
                decision = Ok(Self(Either::Left(auth)));
            }
            Err(error) => {
                if error.error_response().status() == StatusCode::FORBIDDEN {
                    decision = Err(error);
                }
            }
        }

        if decision.is_ok() {
            return ready(decision);
        }

        match UserWithAccessRole::<R>::from_request(req, payload).into_inner() {
            Ok(auth) => {
                decision = Ok(Self(Either::Right(auth)));
            }
            Err(error) => {
                if error.error_response().status() == StatusCode::FORBIDDEN {
                    decision = Err(error);
                }
            }
        }

        ready(decision)
    }
}
