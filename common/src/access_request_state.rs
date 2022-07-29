use derive_more::Display;
use diesel_derive_enum::DbEnum;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

/// State in the access request lifecycle
#[derive(Debug, Display, Copy, Clone, Serialize, Deserialize, DbEnum, JsonSchema, ToSchema)]
#[schema(example = "active")]
#[serde(rename_all = "snake_case")]
pub enum AccessRequestState {
    /// Request has been submitted and may or may not have met required approvals
    Pending,
    /// Request has been approved and the permissions associated with the policy have been granted
    Active,
    /// Request has expired or been marked as completed early by the requesting user
    Completed,
    /// Request has been cancelled before approval by the requesting user
    Cancelled,
    /// Request has been rejected by an Approver or an Administrator
    Rejected,
    /// Request was active, but revoked by an Administrator
    Revoked,
}
