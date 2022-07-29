use std::ops::Deref;

use actix::fut::ready;
use actix::fut::Ready;
use actix_web::dev::Payload;
use actix_web::web;
use actix_web::FromRequest;
use actix_web::HttpRequest;
use color_eyre::eyre::anyhow;
use database::Pool;

use crate::auth::access_role::AccessRole;
use crate::auth::authenticated::Authenticated;
use crate::error;

pub struct UserWithAccessRole<R: AccessRole> {
    pub authenticated: Authenticated,
    phantom_data: std::marker::PhantomData<R>,
}

impl<R: AccessRole> Deref for UserWithAccessRole<R> {
    type Target = Authenticated;

    fn deref(&self) -> &Self::Target {
        &self.authenticated
    }
}

impl<R: AccessRole> FromRequest for UserWithAccessRole<R> {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        let mut connection = match req.app_data::<web::Data<Pool>>() {
            None => return ready(Err(error::Api::Other(anyhow!("database error")).into())),
            Some(pool) => match pool.get() {
                Ok(connection) => connection,
                Err(error) => return ready(Err(error::Api::DatabaseConnection(error).into())),
            },
        };

        let connection = &mut *connection;

        let company_access_role = R::as_enum();

        match Authenticated::from_request(req, payload).into_inner() {
            Err(error) => ready(Err(error)),
            Ok(auth) => {
                let mut decision =
                    ready(Err(error::Api::UnauthorizedUserCredentialsRequired.into()));

                if matches!(
                    auth,
                    Authenticated::Cookie { .. } | Authenticated::UserToken { .. }
                ) {
                    if let Ok(user) = auth.user(connection) {
                        if let Ok(roles) = user.roles(connection) {
                            if roles.contains(&company_access_role) {
                                decision = ready(Ok(Self {
                                    authenticated: auth,
                                    phantom_data: std::marker::PhantomData,
                                }));
                            } else {
                                decision = ready(Err(error::Api::UnauthorizedAccessRoleRequired(
                                    R::as_str(),
                                )
                                .into()));
                            };
                        }
                    }
                }

                decision
            }
        }
    }
}
