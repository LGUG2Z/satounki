use std::collections::HashMap;
use std::path::PathBuf;

use aws::Aws;
use cloudflare::Cloudflare;
use color_eyre::Result;
use common::PermissionsAction;
use common::Request;
use derive_more::Deref;
use gcloud::Gcloud;
use gcloud::GcloudInputData;
use gcloud::IamPolicy;
use gcloud::IamPolicyWrapper;
use serde::Deserialize;
use serde::Serialize;

use crate::configuration::AwsCredentials;
use crate::configuration::CfCredentials;
use crate::configuration::Configuration;
use crate::DRY_RUN;

#[derive(Debug, Deref, Clone, Serialize, Deserialize)]
pub struct Executor(pub Request);

impl Executor {
    pub async fn execute(
        &self,
        action: PermissionsAction,
        configuration: &Configuration,
    ) -> Result<()> {
        if DRY_RUN {
            return Ok(());
        }

        self.execute_aws(action, &configuration.aws).await?;
        self.execute_gcp(action, &configuration.gcp)?;
        self.execute_cf(action, &configuration.cloudflare).await
    }

    async fn execute_aws(
        &self,
        action: PermissionsAction,
        credentials: &Option<HashMap<String, AwsCredentials>>,
    ) -> Result<()> {
        if let (Some(roles), Some(account), Some(credentials)) =
            (&self.aws, &self.aws_account, credentials)
        {
            if let Some(creds) = credentials.get(account) {
                let client = Aws::new(&creds.aws_access_key_id, &creds.aws_secret_access_key);

                let username = self
                    .requester_aliases
                    .aws
                    .as_ref()
                    .map_or(&self.requester, |alias| alias);

                match action {
                    PermissionsAction::Add => {
                        for role in roles {
                            client.attach_user_policy(username, role).await?;
                        }
                    }
                    PermissionsAction::Remove => {
                        for role in roles {
                            client.detach_user_policy(username, role).await?;
                        }
                    }
                }
            }
        }

        Ok(())
    }

    fn execute_gcp(&self, action: PermissionsAction, credentials: &Option<PathBuf>) -> Result<()> {
        if let (Some(roles), Some(project), Some(credentials)) =
            (&self.gcp, &self.gcp_project, credentials)
        {
            let output = Gcloud::projects()
                .get_iam_policy()
                .project(project)
                .execute()?;

            let project_policy: IamPolicy = serde_yaml::from_slice(&output.stdout)?;

            let user_email = self
                .requester_aliases
                .gcp
                .as_ref()
                .map_or(&self.requester, |alias| alias);

            let mut inputs = GcloudInputData {
                action,
                user_email: user_email.clone(),
                project: project.clone(),
                policy: project_policy,
                roles: roles.clone(),
            };

            let wrapper = IamPolicyWrapper::calculate(&mut inputs)?;

            let commands = Gcloud::prepare_command(&wrapper, credentials)?;
            for mut command in commands {
                command.output()?;
            }
        }

        Ok(())
    }

    async fn execute_cf(
        &self,
        action: PermissionsAction,
        credentials: &Option<HashMap<String, CfCredentials>>,
    ) -> Result<()> {
        if let (Some(roles), Some(account), Some(credentials)) =
            (&self.cloudflare, &self.cloudflare_account, credentials)
        {
            if let Some(creds) = credentials.get(account) {
                let mut client = Cloudflare::new(&creds.access_token, &creds.account_id)?;

                let user_email = self
                    .requester_aliases
                    .cloudflare
                    .as_ref()
                    .map_or(&self.requester, |alias| alias);

                match action {
                    PermissionsAction::Add => {
                        if client.add_roles_to_user(user_email, roles).await?.is_none() {
                            log::error!("{user_email} was not recognized by cloudflare");
                        }
                    }
                    PermissionsAction::Remove => {
                        if client
                            .remove_roles_from_user(user_email, roles)
                            .await?
                            .is_none()
                        {
                            log::error!("{user_email} was not recognized by cloudflare");
                        }
                    }
                }
            }
        }

        Ok(())
    }
}
