use common_macros::route_request_response;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

use crate::Terraform;

/// User status
#[apply(Terraform!)]
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, JsonSchema, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum UserStatus {
    /// User is enabled
    Enabled,
    /// User is disabled
    Disabled,
}

route_request_response! {
    #[Get] UserStatus() -> UserStatus,
}
