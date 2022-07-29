use display_json::DisplayAsJson;
use serde::Deserialize;
use serde::Serialize;

use crate::AccessRequestState;
use crate::AwsAccount;
use crate::CloudflareAccount;
use crate::GcpProject;
use crate::Request;

pub trait WebsocketMessage {}
impl WebsocketMessage for ClientMessage {}
impl WebsocketMessage for ServerMessage {}

/// Message sent to the Satounki Websocket Server by a worker client
#[derive(Debug, Serialize, Deserialize, DisplayAsJson)]
#[serde(rename_all = "snake_case")]
pub enum ClientMessage {
    /// Request Google Cloud Platform projects registered for the company
    GetGcpProjects,
    /// Request Amazon Web Services accounts registered for the company
    GetAwsAccounts,
    /// Request Cloudflare accounts registered for the company
    GetCloudflareAccounts,
    /// Request active access requests that have expired
    GetExpiredAccessRequests,
    /// Request approved access requests that have not yet been activated
    GetApprovedAccessRequests,
    /// Request active access requests that have been marked as completed early
    GetCompletedAccessRequests,
    /// Request active access requests that have been revoked
    GetRevokedAccessRequests,
    /// Update the state of an access request
    UpdateAccessRequest {
        id: String,
        new_state: AccessRequestState,
    },
    /// Report an error in handling permissions changes
    Error { id: i32, error: String },
}

/// Message sent to a worker client by the Satounki Websocket Server
#[derive(Debug, Clone, Serialize, Deserialize, DisplayAsJson)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum ServerMessage {
    /// Report an error in request updates
    Error { message: String },
    /// Send all active access requests that have expired
    ExpiredAccessRequests { data: Vec<Request> },
    /// Send all active access requests that have been marked as completed early
    CompletedAccessRequests { data: Vec<Request> },
    /// Send all active access requests that have been revoked
    RevokedAccessRequests { data: Vec<Request> },
    /// Send all approved access requests that have not yet been activated
    ApprovedAccessRequests { data: Vec<Request> },
    /// Send an access request that has just been approved
    AccessRequestApproved { data: Box<Request> },
    /// Send an access request that has just been completed
    AccessRequestCompleted { data: Box<Request> },
    /// Send an access request that has just been revoked
    AccessRequestRevoked { data: Box<Request> },
    /// Send all registered Google Cloud Platform projects for a company
    GcpProjects { data: Vec<GcpProject> },
    /// Send all registered Amazon Web Services accounts for a company
    AwsAccounts { data: Vec<AwsAccount> },
    /// Send all registered Cloudflare accounts for a company
    CloudflareAccounts { data: Vec<CloudflareAccount> },
}
