use serde::Deserialize;
use serde::Serialize;
use serde_with::skip_serializing_none;
use slack_blocks::Block;

use crate::SlackRequest;
use crate::SlackResponse;

impl SlackRequest for SlackApiChatPostMessageRequest<'_> {}
impl SlackRequest for SlackApiChatUpdateRequest<'_> {}

impl SlackResponse for SlackApiChatPostMessageResponse<'_> {}
impl SlackResponse for SlackApiChatUpdateResponse<'_> {}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct SlackApiChatPostMessageRequest<'a> {
    pub channel: String,
    #[serde(flatten)]
    pub content: SlackMessageContent<'a>,
    pub as_user: Option<bool>,
    pub icon_emoji: Option<String>,
    pub icon_url: Option<String>,
    pub link_names: Option<bool>,
    pub parse: Option<String>,
    pub thread_ts: Option<String>,
    pub username: Option<String>,
    pub reply_broadcast: Option<bool>,
    pub unfurl_links: Option<bool>,
    pub unfurl_media: Option<bool>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct Attachment<'a> {
    pub blocks: Vec<Block<'a>>,
    pub color: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct SlackMessageContent<'a> {
    pub text: Option<String>,
    pub blocks: Option<Vec<Block<'a>>>,
    pub attachments: Option<Vec<Attachment<'a>>>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct SlackApiChatPostMessageResponse<'a> {
    pub ok: bool,
    pub error: Option<String>,
    pub channel: String,
    pub ts: String,
    pub message: SlackMessage<'a>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct SlackMessage<'a> {
    #[serde(flatten)]
    pub origin: SlackMessageOrigin,
    #[serde(flatten)]
    pub content: SlackMessageContent<'a>,
    #[serde(flatten)]
    pub sender: SlackMessageSender,
    #[serde(flatten)]
    pub parent: SlackParentMessageParams,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct SlackMessageOrigin {
    pub ts: String,
    pub channel: Option<String>,
    pub channel_type: Option<String>,
    pub thread_ts: Option<String>,
    pub client_msg_id: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct SlackMessageSender {
    pub user: Option<String>,
    pub bot_id: Option<String>,
    pub username: Option<String>,
    pub display_as_bot: Option<bool>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct SlackParentMessageParams {
    pub reply_count: Option<usize>,
    pub reply_users_count: Option<usize>,
    pub latest_reply: Option<String>,
    pub reply_users: Option<Vec<String>>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct SlackApiChatUpdateRequest<'a> {
    pub channel: String,
    #[serde(flatten)]
    pub content: SlackMessageContent<'a>,
    pub ts: String,
    pub as_user: Option<bool>,
    pub link_names: Option<bool>,
    pub parse: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct SlackApiChatUpdateResponse<'a> {
    pub ok: bool,
    pub error: Option<String>,
    pub channel: String,
    pub ts: String,
    pub thread_ts: Option<String>,
    pub message: SlackUpdatedMessage<'a>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct SlackUpdatedMessage<'a> {
    #[serde(flatten)]
    pub sender: SlackMessageSender,
    #[serde(flatten)]
    pub content: SlackMessageContent<'a>,
    pub edited: Option<SlackMessageEdited>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct SlackMessageEdited {
    pub user: String,
    pub ts: String,
}
