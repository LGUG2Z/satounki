use std::collections::HashMap;
use std::path::PathBuf;

use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Configuration {
    pub aws: Option<HashMap<String, AwsCredentials>>,
    pub gcp: Option<PathBuf>,
    pub cloudflare: Option<HashMap<String, CfCredentials>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AwsCredentials {
    pub aws_access_key_id: String,
    pub aws_secret_access_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CfCredentials {
    pub account_id: String,
    pub access_token: String,
}
