use std::path::Path;
use std::process::Command;
use std::process::Output;

use common::PermissionsAction;
use derive_more::Deref;
use derive_more::DerefMut;
use serde::Deserialize;
use serde::Serialize;

use crate::Error;
use crate::IamPolicy;
use crate::IamPolicyWrapper;

#[derive(Debug, Serialize, Deserialize)]
pub struct Inputs {
    pub action: PermissionsAction,
    pub user_email: String,
    pub project: String,
    pub policy: IamPolicy,
    pub roles: Vec<String>,
}

#[derive(Deref, DerefMut)]
pub struct Gcloud(pub Command);

impl Gcloud {
    pub fn execute(&mut self) -> Result<Output, Error> {
        Ok(self.output()?)
    }

    pub fn prepare_command(wrapper: &IamPolicyWrapper, keyfile: &Path) -> Result<Vec<Self>, Error> {
        let mut all = vec![Self::auth().activate_service_account(keyfile)];
        all.push(
            Self::projects()
                .set_iam_policy()
                .project(&wrapper.project)
                .policy_file(&wrapper.policy.write()?),
        );

        Ok(all)
    }

    #[must_use]
    pub fn auth() -> Self {
        let mut command = Command::new("gcloud");

        command.arg("auth");

        Self(command)
    }

    #[must_use]
    pub fn activate_service_account(mut self, keyfile: &Path) -> Self {
        self.arg("activate-service-account");
        self.arg("--keyfile");
        self.arg(&*keyfile.to_string_lossy());

        self
    }

    #[must_use]
    pub fn projects() -> Self {
        let mut command = Command::new("gcloud");

        command.arg("projects");

        Self(command)
    }

    #[must_use]
    pub fn iam() -> Self {
        let mut command = Command::new("gcloud");

        command.arg("iam");

        Self(command)
    }

    #[must_use]
    pub fn project(mut self, project: &str) -> Self {
        self.arg(project);

        self
    }

    #[must_use]
    pub fn roles(mut self) -> Self {
        self.arg("roles");

        self
    }

    #[must_use]
    pub fn list(mut self) -> Self {
        self.arg("list");

        self
    }

    #[must_use]
    pub fn format(mut self, format: &str) -> Self {
        self.arg("--format");
        self.arg(format);

        self
    }

    #[must_use]
    pub fn policy_file(mut self, path: &Path) -> Self {
        self.arg(&*path.to_string_lossy());

        self
    }

    #[must_use]
    pub fn get_iam_policy(mut self) -> Self {
        self.arg("get-iam-policy");

        self
    }

    #[must_use]
    pub fn set_iam_policy(mut self) -> Self {
        self.arg("set-iam-policy");

        self
    }

    #[must_use]
    pub fn add_iam_policy_binding(mut self) -> Self {
        self.arg("add-iam-policy-binding");

        self
    }

    #[must_use]
    pub fn remove_iam_policy_binding(mut self) -> Self {
        self.arg("remove-iam-policy-binding");

        self
    }

    #[must_use]
    pub fn member(mut self, principal: &str) -> Self {
        self.arg("--member");
        self.arg(principal);

        self
    }

    #[must_use]
    pub fn role(mut self, role: &str) -> Self {
        self.arg("--role");
        self.arg(role);

        self
    }

    #[must_use]
    pub fn condition(mut self, condition: Option<String>) -> Self {
        if let Some(condition) = condition {
            self.arg("--condition");
            self.arg(condition);
        }

        self
    }

    #[must_use]
    pub fn condition_from_file(mut self, condition_from_file: &Path) -> Self {
        self.arg("--condition-from-file");
        self.arg(&*condition_from_file.to_string_lossy());

        self
    }
}
