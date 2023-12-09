use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum Ops {
    GetId,
    Get,
    Post,
    PostId,
    Put,
    PutId,
    PutIdBody,
    PatchBody,
    PatchId,
    Delete,
}

#[derive(Debug, Default, Serialize)]
pub struct Context<'a> {
    pub name: &'a str,
    pub post: Option<&'a str>,
    pub get: Option<&'a str>,
    pub put: Option<&'a str>,
    pub patch: Option<&'a str>,
    pub delete: Option<&'a str>,
}

#[derive(Debug, Serialize)]
pub struct Route<'a> {
    pub context: Context<'a>,
    pub ops: Vec<Ops>,
}

impl Route<'_> {
    pub fn is_valid(&self) -> bool {
        let mut valid = true;

        for op in &self.ops {
            match op {
                Ops::GetId => valid = self.context.get.is_some(),
                Ops::Get => valid = self.context.get.is_some(),
                Ops::Post => valid = self.context.post.is_some(),
                Ops::PostId => valid = self.context.post.is_some(),
                Ops::Put => valid = self.context.put.is_some(),
                Ops::PutIdBody => valid = self.context.put.is_some(),
                Ops::PatchBody => valid = self.context.patch.is_some(),
                Ops::PatchId => valid = self.context.patch.is_some(),
                Ops::Delete => valid = self.context.delete.is_some(),
                Ops::PutId => valid = self.context.put.is_some(),
            }

            if !valid {
                return false;
            }
        }

        valid
    }
}

pub fn platform_routes<'a>() -> Vec<Route<'a>> {
    vec![
        // Companies
        Route {
            ops: vec![Ops::Get],
            context: Context {
                name: "Companies",
                get: Option::from("/companies"),
                ..Default::default()
            },
        },
        Route {
            ops: vec![Ops::GetId, Ops::PutIdBody, Ops::Post, Ops::Delete],
            context: Context {
                name: "Company",
                get: Option::from("/company/%s"),
                post: Option::from("/company"),
                put: Option::from("/company/%s"),
                delete: Option::from("/company/%s"),
                ..Default::default()
            },
        },
        Route {
            ops: vec![Ops::GetId, Ops::PutId],
            context: Context {
                name: "PlatformToken",
                get: Option::from("/token/%s"),
                put: Option::from("/token/%s"),
                ..Default::default()
            },
        },
    ]
}

pub fn public_routes<'a>() -> Vec<Route<'a>> {
    vec![
        // Policies
        Route {
            ops: vec![Ops::Get],
            context: Context {
                name: "Policies",
                get: Option::from("/policies"),
                ..Default::default()
            },
        },
        Route {
            ops: vec![Ops::GetId],
            context: Context {
                name: "PolicyName",
                get: Option::from("/policy/name/%s"),
                ..Default::default()
            },
        },
        Route {
            ops: vec![Ops::Post, Ops::PutIdBody, Ops::GetId, Ops::Delete],
            context: Context {
                name: "Policy",
                post: Option::from("/policy"),
                get: Option::from("/policy/%s"),
                put: Option::from("/policy/%s"),
                delete: Option::from("/policy/%s"),
                ..Default::default()
            },
        },
        // Settings
        Route {
            ops: vec![Ops::Get],
            context: Context {
                name: "SettingsAwsAccounts",
                get: Option::from("/settings/aws-accounts"),
                ..Default::default()
            },
        },
        Route {
            ops: vec![Ops::Get],
            context: Context {
                name: "SettingsCloudflareAccounts",
                get: Option::from("/settings/cloudflare-accounts"),
                ..Default::default()
            },
        },
        Route {
            ops: vec![Ops::Get],
            context: Context {
                name: "SettingsGcpProjects",
                get: Option::from("/settings/gcp-projects"),
                ..Default::default()
            },
        },
        Route {
            ops: vec![Ops::Post, Ops::PutIdBody, Ops::GetId, Ops::Delete],
            context: Context {
                name: "SettingsAwsAccount",
                post: Option::from("/settings/aws-account"),
                get: Option::from("/settings/aws-account/%s"),
                put: Option::from("/settings/aws-account/%s"),
                delete: Option::from("/settings/aws-account/%s"),
                ..Default::default()
            },
        },
        Route {
            ops: vec![Ops::Post, Ops::PutIdBody, Ops::GetId, Ops::Delete],
            context: Context {
                name: "SettingsCloudflareAccount",
                post: Option::from("/settings/cloudflare-account"),
                get: Option::from("/settings/cloudflare-account/%s"),
                put: Option::from("/settings/cloudflare-account/%s"),
                delete: Option::from("/settings/cloudflare-account/%s"),
                ..Default::default()
            },
        },
        Route {
            ops: vec![Ops::Post, Ops::PutIdBody, Ops::GetId, Ops::Delete],
            context: Context {
                name: "SettingsGcpProject",
                post: Option::from("/settings/gcp-project"),
                get: Option::from("/settings/gcp-project/%s"),
                put: Option::from("/settings/gcp-project/%s"),
                delete: Option::from("/settings/gcp-project/%s"),
                ..Default::default()
            },
        },
        // Users
        Route {
            ops: vec![Ops::PostId, Ops::PutIdBody, Ops::GetId, Ops::Delete],
            context: Context {
                name: "UserAliases",
                post: Option::from("/user/%s/aliases"),
                get: Option::from("/user/%s/aliases"),
                put: Option::from("/user/%s/aliases"),
                delete: Option::from("/user/%s/aliases"),
                ..Default::default()
            },
        },
        Route {
            ops: vec![Ops::PostId, Ops::PutIdBody, Ops::GetId, Ops::Delete],
            context: Context {
                name: "UserRoles",
                post: Option::from("/user/%s/roles"),
                get: Option::from("/user/%s/roles"),
                put: Option::from("/user/%s/roles"),
                delete: Option::from("/user/%s/roles"),
                ..Default::default()
            },
        },
        Route {
            ops: vec![Ops::Get, Ops::Put],
            context: Context {
                name: "UserToken",
                get: Option::from("/user/token"),
                put: Option::from("/user/token"),
                ..Default::default()
            },
        },
        Route {
            ops: vec![Ops::PatchId],
            context: Context {
                name: "UserEnable",
                patch: Option::from("/user/%s/enable"),
                ..Default::default()
            },
        },
        Route {
            ops: vec![Ops::PatchId],
            context: Context {
                name: "UserDisable",
                patch: Option::from("/user/%s/enable"),
                ..Default::default()
            },
        },
        // Requests
        Route {
            ops: vec![Ops::GetId, Ops::PatchBody],
            context: Context {
                name: "RequestAlias",
                get: Option::from("/request/alias/%s"),
                patch: Option::from("/request/alias/%s"),
                ..Default::default()
            },
        },
        Route {
            ops: vec![Ops::PostId],
            context: Context {
                name: "RequestPolicy",
                post: Option::from("/request/policy/%s"),
                ..Default::default()
            },
        },
        Route {
            ops: vec![Ops::Get],
            context: Context {
                name: "Requests",
                get: Option::from("/requests"),
                ..Default::default()
            },
        },
    ]
}
