use std::collections::hash_map::DefaultHasher;
use std::env::temp_dir;
use std::hash::Hash;
use std::hash::Hasher;
use std::path::PathBuf;

use common::PermissionsAction;
use serde::Deserialize;
use serde::Serialize;

use crate::Error;
use crate::GcloudInputData;

const PROJECT_IAM_ADMIN: &str = "roles/resourcemanager.projectIamAdmin";

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct Wrapper {
    pub project: String,
    pub policy: IamPolicy,
}

impl Wrapper {
    pub fn calculate(inputs: &mut GcloudInputData) -> Result<Self, Error> {
        match inputs.action {
            PermissionsAction::Add => {
                inputs.policy.add_updates(&inputs.user_email, &inputs.roles);
            }
            PermissionsAction::Remove => {
                inputs
                    .policy
                    .remove_updates(&inputs.user_email, &inputs.roles);
            }
        }

        Ok(Self {
            project: inputs.project.to_string(),
            policy: inputs.policy.clone(),
        })
    }
}

#[derive(Default, Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct IamPolicy {
    pub bindings: Vec<Binding>,
    pub etag: String,
    pub version: i64,
}

#[derive(Default, Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct Binding {
    pub members: Vec<String>,
    pub role: String,
}

impl IamPolicy {
    #[must_use]
    pub fn service_account_is_project_iam_admin(&self, service_account: &str) -> bool {
        let mut is = false;

        for binding in &self.bindings {
            if binding.role == PROJECT_IAM_ADMIN
                && binding
                    .members
                    .contains(&format!("serviceAccount:{service_account}"))
            {
                is = true;
                break;
            }
        }

        is
    }

    pub fn write(&self) -> Result<PathBuf, Error> {
        let mut s = DefaultHasher::new();
        self.hash(&mut s);
        let hash = s.finish();
        let path_buf = temp_dir().join(format!("{hash}.yaml"));

        let content = serde_yaml::to_string(&self)?;
        std::fs::write(path_buf.iter(), content)?;

        Ok(path_buf)
    }

    pub fn add_updates(&mut self, user: &str, roles: &Vec<String>) {
        for role in roles {
            let mut role_exists = false;
            for binding in &mut self.bindings {
                if *role == binding.role {
                    role_exists = true;
                    let principal = format!("user:{user}");
                    if !binding.members.contains(&principal) {
                        binding.members.push(principal);
                    }
                }
            }

            if !role_exists {
                self.bindings.push(Binding {
                    members: vec![format!("user:{}", user)],
                    role: role.to_string(),
                });
            }
        }
    }

    pub fn remove_updates(&mut self, user: &str, roles: &Vec<String>) {
        for role in roles {
            for binding in &mut self.bindings {
                if *role == binding.role {
                    let principal = format!("user:{user}");
                    if let Some(idx) = binding.members.iter().position(|m| m == &principal) {
                        binding.members.remove(idx);
                    }
                }
            }
        }
    }
}
