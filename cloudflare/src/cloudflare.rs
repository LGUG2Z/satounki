use common::CloudflareRole;
use reqwest::header::HeaderMap;
use reqwest::header::HeaderValue;
use reqwest::header::AUTHORIZATION;
use reqwest::header::CONTENT_TYPE;

use crate::member::ListRolesResponse;
use crate::tokens::TokenDetailsResponse;
use crate::tokens::VerifyTokenResponse;
use crate::CloudflareRequest;
use crate::CloudflareResponse;
use crate::ListMembersResponse;
use crate::Member;
use crate::Result;
use crate::UpdateMemberRequest;
use crate::UpdateMemberResponse;

const BASE_URL: &str = "https://api.cloudflare.com/client/v4";
const MEMBERSHIPS_READ: &str = "Memberships Read";
const MEMBERSHIPS_WRITE: &str = "Memberships Write";

pub struct Cloudflare {
    client: reqwest::Client,
    account_id: String,
}

impl Cloudflare {
    #[allow(clippy::missing_panics_doc)]
    pub fn new(access_token: &str, account_id: &str) -> Result<Self> {
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

        Ok(Self {
            client,
            account_id: account_id.to_string(),
        })
    }

    pub async fn add_roles_to_user(
        &mut self,
        user_email: &str,
        roles: &Vec<CloudflareRole>,
    ) -> Result<Option<UpdateMemberResponse>> {
        Ok(match self.member(user_email).await? {
            None => None,
            Some(mut member) => {
                let mut new_role_names = vec![];
                for role in roles {
                    new_role_names.push(role.to_string());
                }

                let roles_response = self.list_roles().await?;

                let new_role_definitions: Vec<_> = roles_response
                    .result
                    .iter()
                    .filter(|r| new_role_names.contains(&r.name))
                    .collect();

                for role in new_role_definitions {
                    if !member.roles.contains(role) {
                        member.roles.push(role.clone());
                    }
                }

                Option::from(self.update_member(&member.id, &member).await?)
            }
        })
    }

    pub async fn remove_roles_from_user(
        &mut self,
        user_email: &str,
        roles: &Vec<CloudflareRole>,
    ) -> Result<Option<UpdateMemberResponse>> {
        Ok(match self.member(user_email).await? {
            None => None,
            Some(mut member) => {
                let mut expired_role_names = vec![];
                for role in roles {
                    expired_role_names.push(role.to_string());
                }

                member
                    .roles
                    .retain(|r| !expired_role_names.contains(&r.name));

                Option::from(self.update_member(&member.id, &member).await?)
            }
        })
    }

    async fn member(&mut self, user_email: &str) -> Result<Option<Member>> {
        for member in self.list_members().await?.result {
            if member.user.email == user_email {
                return Ok(Some(member));
            }
        }

        Ok(None)
    }

    async fn update_member(
        &mut self,
        membership_id: &str,
        body: &UpdateMemberRequest,
    ) -> Result<UpdateMemberResponse> {
        self.put::<UpdateMemberRequest, UpdateMemberResponse>(
            &format!("accounts/{}/members/{membership_id}", self.account_id),
            body,
        )
        .await
    }

    async fn list_roles(&mut self) -> Result<ListRolesResponse> {
        self.get::<ListRolesResponse>(&format!("accounts/{}/roles", self.account_id), &[])
            .await
    }

    async fn verify_token(&mut self) -> Result<VerifyTokenResponse> {
        self.get::<VerifyTokenResponse>("user/tokens/verify", &[])
            .await
    }

    pub async fn memberships_writer(&mut self) -> Result<bool> {
        let mut is = false;
        let mut matches = 0;
        let verified = self.verify_token().await?;
        let token_id = verified.result.id;
        let token_details = self
            .get::<TokenDetailsResponse>(&format!("user/tokens/{token_id}"), &[])
            .await?;

        for policy in &token_details.result.policies {
            for group in &policy.permission_groups {
                if group.name == MEMBERSHIPS_READ {
                    matches += 1;
                }

                if group.name == MEMBERSHIPS_WRITE {
                    matches += 1;
                }
            }
        }

        if matches == 2 {
            is = true;
        }

        Ok(is)
    }

    async fn list_members(&mut self) -> Result<ListMembersResponse> {
        let page = 1;
        let per_page = 50;

        let mut response = self
            .get::<ListMembersResponse>(
                &format!("accounts/{}/members", self.account_id),
                &[
                    ("page", &page.to_string()),
                    ("per_page", &per_page.to_string()),
                ],
            )
            .await?;

        let mut received = response.result.len();

        if received == per_page {
            while received == per_page {
                let mut next = self
                    .get::<ListMembersResponse>(
                        &format!("accounts/{}/members", self.account_id),
                        &[
                            ("page", &page.to_string()),
                            ("per_page", &per_page.to_string()),
                        ],
                    )
                    .await?;

                received = next.result.len();

                response.result.append(&mut next.result);
            }
        }

        Ok(response)
    }

    async fn get<Resp: CloudflareResponse>(
        &mut self,
        uri: &str,
        queries: &[(&str, &str)],
    ) -> Result<Resp> {
        let response = self
            .client
            .get(format!("{BASE_URL}/{uri}"))
            .query(queries)
            .send()
            .await?;

        response.json::<Resp>().await
    }

    async fn put<Body: CloudflareRequest, Resp: CloudflareResponse>(
        &mut self,
        uri: &str,
        body: &Body,
    ) -> Result<Resp> {
        let response = self
            .client
            .put(format!("{BASE_URL}/{uri}"))
            .json(body)
            .send()
            .await?;

        response.json::<Resp>().await
    }
}
