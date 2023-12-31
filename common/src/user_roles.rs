use common_macros::route_request_response;
use diesel_derive_enum::DbEnum;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

use crate::Terraform;

/// Satounki user access roles
#[apply(Terraform!)]
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, JsonSchema, ToSchema, DbEnum)]
#[serde(rename_all = "snake_case")]
pub enum AccessRole {
    /// View and make access requests
    User,
    /// Approve access requests
    Approver,
    /// Change user roles, grant administrator approval to access requests
    Administrator,
}

route_request_response! {
    #[Post] UserRoles(Vec<AccessRole>) -> Vec<AccessRole>,
    #[Put]  UserRoles(Vec<AccessRole>) -> Vec<AccessRole>,
    #[Get]  UserRoles() -> Vec<AccessRole>,
}
