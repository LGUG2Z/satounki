use common_macros::response;
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

response! {
    #[Get] Requests -> Vec<Request>
}
