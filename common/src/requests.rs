use common_macros::route_request_response;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use utoipa::IntoParams;
use utoipa::ToSchema;

use crate::AccessRequestState;
use crate::Request;

/// Query parameters for the GET /v1/requests endpoint
#[derive(Serialize, Deserialize, JsonSchema, ToSchema, IntoParams)]
pub struct RequestsGetQueryParams {
    /// State of the requests
    pub state: AccessRequestState,
    /// Number of requests to return
    pub count: i64,
}

route_request_response! {
    #[Get] Requests() -> Vec<Request>
}
