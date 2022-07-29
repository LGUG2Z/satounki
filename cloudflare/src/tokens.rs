use std::collections::HashMap;

use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

use crate::CloudflareResponse;

impl CloudflareResponse for VerifyTokenResponse {}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VerifyTokenResponse {
    pub result: Result,
    pub success: bool,
    pub errors: Vec<Value>,
    pub messages: Vec<Message>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Result {
    pub id: String,
    pub status: String,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Message {
    pub code: i64,
    pub message: String,
    #[serde(rename = "type")]
    pub type_field: Value,
}

impl CloudflareResponse for TokenDetailsResponse {}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TokenDetailsResponse {
    pub success: bool,
    pub errors: Vec<Value>,
    pub messages: Vec<Value>,
    pub result: ResultField,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResultField {
    pub id: String,
    pub name: String,
    pub status: String,
    pub issued_on: String,
    pub modified_on: String,
    pub not_before: String,
    pub expires_on: String,
    pub policies: Vec<Policy>,
    pub condition: HashMap<String, Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Policy {
    pub id: String,
    pub effect: String,
    pub resources: HashMap<String, String>,
    pub permission_groups: Vec<PermissionGroup>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PermissionGroup {
    pub id: String,
    pub name: String,
}
