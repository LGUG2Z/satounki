use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

use crate::CloudflareRequest;
use crate::CloudflareResponse;

impl CloudflareResponse for ListMembersResponse {}
impl CloudflareResponse for UpdateMemberResponse {}
impl CloudflareResponse for ListRolesResponse {}
impl CloudflareResponse for Role {}
impl CloudflareRequest for Member {}

#[derive(Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct ListRolesResponse {
    pub success: bool,
    pub errors: Vec<Value>,
    pub messages: Vec<Value>,
    pub result: Vec<Role>,
}

#[derive(Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct ListMembersResponse {
    pub success: bool,
    pub errors: Vec<Value>,
    pub messages: Vec<Value>,
    pub result: Vec<Member>,
}

#[derive(Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct UpdateMemberResponse {
    pub success: bool,
    pub errors: Vec<Value>,
    pub messages: Vec<Value>,
    pub result: Member,
}

#[derive(Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Member {
    pub id: String,
    pub code: Option<String>,
    pub user: User,
    pub status: String,
    pub roles: Vec<Role>,
}

#[derive(Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: String,
    pub two_factor_authentication_enabled: bool,
}

#[derive(Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Role {
    pub id: String,
    pub name: String,
    pub description: String,
    pub permissions: Permissions,
}

#[derive(Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Permissions {
    pub analytics: Option<Analytics>,
    pub billing: Option<Billing>,
    pub cache_purge: Option<CachePurge>,
    pub dns: Option<Dns>,
    pub dns_records: Option<DnsRecords>,
    pub lb: Option<Lb>,
    pub logs: Option<Logs>,
    pub organization: Option<Organization>,
    pub ssl: Option<Ssl>,
    pub waf: Option<Waf>,
    pub zones: Option<Zones>,
    pub zone_settings: Option<ZoneSettings>,
}

#[derive(Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Analytics {
    pub read: bool,
    pub edit: bool,
}

#[derive(Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Billing {
    pub read: bool,
    pub edit: bool,
}

#[derive(Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct CachePurge {
    pub read: bool,
    pub edit: bool,
}

#[derive(Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Dns {
    pub read: bool,
    pub edit: bool,
}

#[derive(Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct DnsRecords {
    pub read: bool,
    pub edit: bool,
}

#[derive(Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Lb {
    pub read: bool,
    pub edit: bool,
}

#[derive(Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Logs {
    pub read: bool,
    pub edit: bool,
}

#[derive(Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Organization {
    pub read: bool,
    pub edit: bool,
}

#[derive(Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Ssl {
    pub read: bool,
    pub edit: bool,
}

#[derive(Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Waf {
    pub read: bool,
    pub edit: bool,
}

#[derive(Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Zones {
    pub read: bool,
    pub edit: bool,
}

#[derive(Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct ZoneSettings {
    pub read: bool,
    pub edit: bool,
}
