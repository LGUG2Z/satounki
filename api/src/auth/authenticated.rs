use actix::fut::ready;
use actix::fut::Ready;
use actix_session::SessionExt;
use actix_web::dev::Payload;
use actix_web::web;
use actix_web::FromRequest;
use actix_web::HttpMessage;
use actix_web::HttpRequest;
use actix_web_httpauth::extractors::bearer::BearerAuth;
use color_eyre::eyre::anyhow;
use database::ApiToken;
use database::Company;
use database::Pool;
use database::SqliteConnection;
use database::User;
use database::UserToken;

use crate::error;

#[derive(Clone)]
pub enum Authenticated {
    Cookie { user_id: i32 },
    UserToken { token: UserToken },
    ApiToken { token: ApiToken },
}

impl Authenticated {
    pub fn company_id(&self, connection: &mut SqliteConnection) -> crate::Result<i32> {
        Ok(match self {
            Authenticated::Cookie { user_id } => {
                User::read(connection, *user_id)?.company(connection)?.id
            }
            Authenticated::UserToken { token } => {
                User::read(connection, token.user_id)?
                    .company(connection)?
                    .id
            }
            Authenticated::ApiToken { token } => token.company_id,
        })
    }

    pub fn company(&self, connection: &mut SqliteConnection) -> crate::Result<Company> {
        Ok(match self {
            Authenticated::Cookie { user_id } => {
                User::read(connection, *user_id)?.company(connection)?
            }
            Authenticated::UserToken { token } => {
                User::read(connection, token.user_id)?.company(connection)?
            }
            Authenticated::ApiToken { token } => Company::read(connection, token.company_id)?,
        })
    }

    pub fn user(&self, connection: &mut SqliteConnection) -> crate::Result<User> {
        Ok(match self {
            Authenticated::Cookie { user_id } => User::read(connection, *user_id)?,
            Authenticated::UserToken { token } => User::read(connection, token.user_id)?,
            Authenticated::ApiToken { .. } => {
                return Err(error::Api::UnauthorizedUserCredentialsRequired)
            }
        })
    }

    #[allow(dead_code)]
    pub const fn user_id(&self) -> crate::Result<i32> {
        Ok(match self {
            Authenticated::Cookie { user_id } => *user_id,
            Authenticated::UserToken { token } => token.user_id,
            Authenticated::ApiToken { .. } => {
                return Err(error::Api::UnauthorizedUserCredentialsRequired)
            }
        })
    }
}

impl FromRequest for Authenticated {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Self::Error>>;

    #[allow(clippy::too_many_lines)]
    #[allow(clippy::cognitive_complexity)]
    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let session = req.get_session();
        let mut connection = match req.app_data::<web::Data<Pool>>() {
            None => return ready(Err(error::Api::Other(anyhow!("database error")).into())),
            Some(pool) => match pool.get() {
                Ok(connection) => connection,
                Err(error) => return ready(Err(error::Api::DatabaseConnection(error).into())),
            },
        };

        let connection = &mut *connection;

        if let Ok(Some(user_id)) = session.get::<i32>("user_id") {
            if let Ok(user) = User::read(connection, user_id) {
                log::info!("user found from cookie");
                if !user.active {
                    return ready(Err(error::Api::ForbiddenInactiveUser.into()));
                }

                log::info!("user is active");

                if user.company(connection).is_ok() {
                    log::info!("company found for user");
                    return ready(Ok(Self::Cookie { user_id }));
                }
            }
        }

        let token = match req.extensions().get::<BearerAuth>() {
            None => return ready(Err(error::Api::UnauthorizedNoValidCredentials.into())),
            Some(credentials) => {
                log::info!("bearer auth found");
                credentials.token().to_string()
            }
        };

        if let Ok(api_token) = ApiToken::read(connection, &token) {
            log::info!("api token found");
            if Company::read(connection, api_token.company_id).is_ok() {
                log::info!("company found for api token");
                return ready(Ok(Self::ApiToken { token: api_token }));
            }
        }

        if let Ok(user_token) = UserToken::read(connection, &token) {
            log::info!("user token found");
            if let Ok(user) = User::read(connection, user_token.user_id) {
                log::info!("user found for user_token");
                if !user.active {
                    return ready(Err(error::Api::ForbiddenInactiveUser.into()));
                }

                if user.company(connection).is_ok() {
                    log::info!("company found for user");
                    return ready(Ok(Self::UserToken { token: user_token }));
                }
            }
        }

        log::info!("no valid credentials found");
        ready(Err(error::Api::UnauthorizedNoValidCredentials.into()))
    }
}
