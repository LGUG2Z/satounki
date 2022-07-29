use serde::Deserialize;
use serde::Serialize;
use serde_with::skip_serializing_none;

use crate::SlackResponse;

impl SlackResponse for SlackApiUsersProfileGetResponse {}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct SlackApiUsersProfileGetResponse {
    pub ok: bool,
    pub error: Option<String>,
    pub profile: SlackUserProfile,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct SlackUserProfile {
    pub email: String,
}
