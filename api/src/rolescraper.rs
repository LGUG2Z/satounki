use serde::Deserialize;
use serde::Serialize;

#[derive(Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Gcp {
    pub description: Option<String>,
    pub etag: String,
    pub name: String,
    pub stage: Option<String>,
    pub title: Option<String>,
}

#[derive(Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Aws {
    pub path: String,
    pub policy_name: String,
    pub policy_id: String,
    pub arn: String,
}
