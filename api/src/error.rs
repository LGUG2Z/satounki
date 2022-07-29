use std::env::VarError;

use actix_web::http::StatusCode;
use actix_web::HttpResponse;
use actix_web::ResponseError;
use common::ErrorResponse;
use database::PoolError;
use oauth2::url::ParseError;
use thiserror::Error;

use crate::DieselError;

#[derive(Debug, Error)]
pub enum Api {
    #[error(transparent)]
    DatabaseTransaction(#[from] DieselError),
    #[error(transparent)]
    DatabaseConnection(#[from] PoolError),
    #[error(transparent)]
    Serialization(#[from] serde_json::Error),
    #[error(transparent)]
    ExternalRequest(#[from] reqwest::Error),
    #[error(transparent)]
    Actix(#[from] actix_web::Error),
    #[error(transparent)]
    ActixSessionInsert(#[from] actix_session::SessionInsertError),
    #[error(transparent)]
    ActixSessionGet(#[from] actix_session::SessionGetError),
    #[error("internal: database connection pool not available")]
    Configuration(#[from] VarError),
    #[error("login: {0}")]
    Oauth2(#[from] ParseError),
    #[error("login: {0}")]
    Login(String),
    #[error("invalid roles {0:?}")]
    InvalidRoles(Vec<String>),
    #[error("a gcp project is required for this policy")]
    GcpProjectRequiredForPolicy,
    #[error("the gcp project \"{0}\" was not recognized for your company")]
    UnknownGcpProject(String),
    #[error("an aws account is required for this policy")]
    AwsAccountRequiredForPolicy,
    #[error("the aws account \"{0}\" was not recognized for your company")]
    UnknownAwsAccount(String),
    #[error("a cloudflare account is required for this policy")]
    CloudflareAccountRequiredForPolicy,
    #[error("the cloudflare account \"{0}\" was not recognized for your company")]
    UnknownCloudflareAccount(String),
    #[error("authentication: {0}")]
    Authentication(String),
    #[error("your user has been marked as disabled by an administrator")]
    ForbiddenInactiveUser,
    #[error("resource not found")]
    UnauthorizedNotFound,
    #[error("this operation requires an api token, user credentials cannot be used")]
    UnauthorizedApiTokenRequired,
    #[error("this operation requires user credentials (user token or cookie), an api token cannot be used")]
    UnauthorizedUserCredentialsRequired,
    #[error("this operation requires the \"{0}\" role for a user token or an api token")]
    UnauthorizedApiTokenOrRoleRequired(&'static str),
    #[error("this operation requires the \"{0}\" role")]
    UnauthorizedAccessRoleRequired(&'static str),
    #[error("this operation requires a platform token")]
    UnauthorizedPlatformTokenRequired,
    #[error("this operation requires the \"{0}\" role")]
    UnauthorizedPlatformScopeRequired(&'static str),
    #[error("this operation requires valid credentials (user token, api token or cookie)")]
    UnauthorizedNoValidCredentials,
    #[error("the \"user\" role is required to make requests")]
    UnauthorizedRequest,
    #[error("the \"approver\" role is required to approve requests")]
    UnauthorizedApproval,
    #[error("you cannot approve your own requests")]
    UnauthorizedSelfApproval,
    #[error("you cannot approve the same request multiple times")]
    UnauthorizedMultipleApprovals,
    #[error("the \"approver\" role is required to reject requests")]
    UnauthorizedRejection,
    #[error("you cannot reject your own requests; try cancelling it instead")]
    UnauthorizedSelfRejection,
    #[error("the \"approver\" role is required to extend active requests")]
    UnauthorizedExtension,
    #[error("you cannot grant extensions to your own requests")]
    UnauthorizedSelfExtension,
    #[error("the \"administrator\" role is required to revoke active requests")]
    UnauthorizedRevocation,
    #[error("only the user who made the request can execute this action")]
    UnauthorizedRequesterRequired,
    #[error("this request is no longer pending; cannot approve")]
    RequestNotPendingCannotApprove,
    #[error("this request is no longer pending; cannot reject")]
    RequestNotPendingCannotReject,
    #[error("this request is no longer pending; cannot cancel")]
    RequestNotPendingCannotCancel,
    #[error("this request is not active; cannot complete")]
    RequestNotActiveCannotComplete,
    #[error("this request is not active; cannot extend")]
    RequestNotActiveCannotExtend,
    #[error("this request is not active; cannot revoke")]
    RequestNotActiveCannotRevoke,
    #[error("this request has already been approved")]
    RequestAlreadyApproved,
    #[error("the maximum number of requests that can be listed is {0}")]
    MaximumRequestListExceeded(i64),
    #[error("an unexpected value was found in the url")]
    BadPathFragment(#[from] strum::ParseError),
    #[error("only one worker can be connected at any one time")]
    TooManyWorkers,
    #[error(transparent)]
    Other(#[from] color_eyre::eyre::Error),
}

impl ResponseError for Api {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::DatabaseConnection(_)
            | Self::Configuration(_)
            | Self::Serialization(_)
            | Self::Actix(_)
            | Self::ActixSessionGet(_)
            | Self::ActixSessionInsert(_)
            | Self::Other(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::Login(_) | Self::Oauth2(_) | Self::ExternalRequest(_) => StatusCode::BAD_GATEWAY,
            Self::Authentication(_)
            | Self::UnauthorizedUserCredentialsRequired
            | Self::UnauthorizedNoValidCredentials
            | Self::UnauthorizedAccessRoleRequired(_)
            | Self::UnauthorizedApiTokenRequired
            | Self::UnauthorizedApiTokenOrRoleRequired(_)
            | Self::UnauthorizedRequest
            | Self::UnauthorizedRequesterRequired
            | Self::UnauthorizedPlatformTokenRequired
            | Self::UnauthorizedPlatformScopeRequired(_)
            | Self::UnauthorizedSelfApproval
            | Self::UnauthorizedRejection
            | Self::UnauthorizedExtension
            | Self::UnauthorizedRevocation
            | Self::UnauthorizedSelfExtension
            | Self::UnauthorizedSelfRejection
            | Self::UnauthorizedMultipleApprovals
            | Self::UnauthorizedApproval => StatusCode::UNAUTHORIZED,
            Self::UnauthorizedNotFound => StatusCode::NOT_FOUND,
            Self::TooManyWorkers => StatusCode::TOO_MANY_REQUESTS,
            Self::UnknownAwsAccount(_)
            | Self::UnknownGcpProject(_)
            | Self::UnknownCloudflareAccount(_)
            | Self::GcpProjectRequiredForPolicy
            | Self::CloudflareAccountRequiredForPolicy
            | Self::BadPathFragment(_)
            | Self::AwsAccountRequiredForPolicy => StatusCode::BAD_REQUEST,
            Self::DatabaseTransaction(error) => match error {
                DieselError::NotFound => StatusCode::NOT_FOUND,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            },
            Self::InvalidRoles(_)
            | Self::RequestAlreadyApproved
            | Self::RequestNotActiveCannotExtend
            | Self::RequestNotActiveCannotComplete
            | Self::RequestNotActiveCannotRevoke
            | Self::RequestNotPendingCannotCancel
            | Self::RequestNotPendingCannotReject
            | Self::MaximumRequestListExceeded(_)
            | Self::RequestNotPendingCannotApprove => StatusCode::UNPROCESSABLE_ENTITY,
            Self::ForbiddenInactiveUser => StatusCode::FORBIDDEN,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(ErrorResponse {
            code: self.status_code().as_u16(),
            error: self.to_string(),
        })
    }
}
