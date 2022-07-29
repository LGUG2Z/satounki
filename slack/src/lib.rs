#![warn(clippy::all, clippy::nursery, clippy::pedantic)]
#![allow(clippy::missing_errors_doc, clippy::use_self)]

pub use chat::Attachment;
pub use chat::SlackApiChatPostMessageRequest;
pub use chat::SlackApiChatUpdateRequest;
pub use chat::SlackMessageContent;
use serde::de::DeserializeOwned;
use serde::Serialize;
pub use slack::Slack;

mod chat;
mod slack;
mod users;

type Result<T> = core::result::Result<T, reqwest::Error>;

trait SlackRequest: Serialize + Sync + Send {}
trait SlackResponse: DeserializeOwned + Sync + Send {}
