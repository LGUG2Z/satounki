#![warn(clippy::all, clippy::nursery, clippy::pedantic)]
#![allow(clippy::missing_errors_doc)]

use aws_sdk_iam::config::Credentials;
use aws_sdk_iam::config::Region;
use aws_sdk_iam::error::SdkError;
use aws_sdk_iam::operation::get_user::GetUserError;
use aws_sdk_iam::operation::get_user::GetUserOutput;
use aws_sdk_iam::operation::list_entities_for_policy::ListEntitiesForPolicyError;
use aws_sdk_iam::operation::list_entities_for_policy::ListEntitiesForPolicyOutput;
use aws_sdk_iam::operation::list_policies::ListPoliciesError;
use aws_sdk_iam::operation::list_policies::ListPoliciesOutput;
use aws_sdk_iam::Client;
use aws_sdk_iam::Config;
use aws_sdk_iam::Error;

const IAM_FULL_ACCESS: &str = "arn:aws:iam::aws:policy/IAMFullAccess";

pub struct Aws {
    client: Client,
}

impl Aws {
    #[must_use]
    pub fn new(aws_access_key_id: &str, aws_secret_access_key: &str) -> Self {
        let credentials = Credentials::new(
            aws_access_key_id,
            aws_secret_access_key,
            None,
            None,
            "satounki",
        );

        let config = Config::builder()
            .credentials_provider(credentials)
            .behavior_version_latest()
            .region(Region::new("us-east-1"))
            .build();

        let client = Client::from_conf(config);

        Self { client }
    }

    #[allow(clippy::missing_panics_doc)]
    pub async fn iam_full_access(&self) -> Result<bool, Error> {
        let get_user_output = self.get_user().await?;
        let list_entities_output = self.list_entities_for_policy(IAM_FULL_ACCESS).await?;
        // FIXME: Can we do this without expect?
        let user = get_user_output
            .user()
            .expect("aws did not recognize a user associated with the given credentials");

        if let Some(policy_users) = list_entities_output.policy_users {
            for policy_user in policy_users {
                if let Some(user_id) = policy_user.user_id {
                    if user_id == user.user_id {
                        return Ok(true);
                    }
                }
            }
        }

        Ok(false)
    }

    async fn get_user(&self) -> Result<GetUserOutput, SdkError<GetUserError>> {
        self.client.get_user().send().await
    }

    async fn list_entities_for_policy(
        &self,
        policy_arn: &str,
    ) -> Result<ListEntitiesForPolicyOutput, SdkError<ListEntitiesForPolicyError>> {
        self.client
            .list_entities_for_policy()
            .policy_arn(policy_arn)
            .send()
            .await
    }

    pub async fn attach_user_policy(&self, username: &str, policy_arn: &str) -> Result<(), Error> {
        self.client
            .attach_user_policy()
            .user_name(username)
            .policy_arn(policy_arn)
            .send()
            .await?;

        Ok(())
    }

    pub async fn detach_user_policy(&self, username: &str, policy_arn: &str) -> Result<(), Error> {
        self.client
            .detach_user_policy()
            .user_name(username)
            .policy_arn(policy_arn)
            .send()
            .await?;

        Ok(())
    }

    pub async fn list_policies(
        &self,
        path_prefix: Option<String>,
        marker: Option<String>,
        max_items: Option<i32>,
    ) -> Result<ListPoliciesOutput, SdkError<ListPoliciesError>> {
        let response = self
            .client
            .list_policies()
            .set_path_prefix(path_prefix)
            .set_marker(marker)
            .set_max_items(max_items)
            .send()
            .await?;

        Ok(response)
    }
}
