use reqwest::header::HeaderMap;
use reqwest::header::HeaderValue;
use reqwest::header::AUTHORIZATION;
use reqwest::header::CONTENT_TYPE;

use crate::chat::SlackApiChatPostMessageRequest;
use crate::chat::SlackApiChatPostMessageResponse;
use crate::chat::SlackApiChatUpdateRequest;
use crate::chat::SlackApiChatUpdateResponse;
use crate::users::SlackApiUsersProfileGetResponse;
use crate::Result;
use crate::SlackRequest;
use crate::SlackResponse;

const BASE_URL: &str = "https://slack.com/api/";

pub struct Slack {
    client: reqwest::Client,
}

impl Slack {
    #[allow(clippy::missing_panics_doc)]
    pub fn new(access_token: &str) -> Result<Self> {
        let mut default_headers = HeaderMap::new();
        let bearer_token = format!("Bearer {access_token}");

        // FIXME: Can we do this without expect?
        default_headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&bearer_token).expect("this is not a valid cloudflare token"),
        );
        default_headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        let client = reqwest::ClientBuilder::new()
            .default_headers(default_headers)
            .build()?;

        Ok(Self { client })
    }

    pub async fn chat_post_message(
        &self,
        body: &SlackApiChatPostMessageRequest<'_>,
    ) -> Result<SlackApiChatPostMessageResponse> {
        self.post::<SlackApiChatPostMessageRequest, SlackApiChatPostMessageResponse>(
            "chat.postMessage",
            body,
        )
        .await
    }

    pub async fn chat_update(
        &self,
        body: &SlackApiChatUpdateRequest<'_>,
    ) -> Result<SlackApiChatUpdateResponse> {
        self.post::<SlackApiChatUpdateRequest, SlackApiChatUpdateResponse>("chat.update", body)
            .await
    }

    pub async fn users_profile_get(
        &self,
        user_id: &str,
    ) -> Result<SlackApiUsersProfileGetResponse> {
        self.get::<SlackApiUsersProfileGetResponse>("users.profile.get", &[("user", user_id)])
            .await
    }

    async fn get<Resp: SlackResponse>(&self, uri: &str, queries: &[(&str, &str)]) -> Result<Resp> {
        let response = self
            .client
            .get(format!("{BASE_URL}/{uri}"))
            .query(queries)
            .send()
            .await?;

        response.json::<Resp>().await
    }

    async fn post<Body: SlackRequest, Resp: SlackResponse>(
        &self,
        uri: &str,
        body: &Body,
    ) -> Result<Resp> {
        let response = self
            .client
            .post(format!("{BASE_URL}/{uri}"))
            .json(body)
            .send()
            .await?;

        response.json::<Resp>().await
    }
}
