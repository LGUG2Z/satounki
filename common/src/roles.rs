use std::fmt::Display;
use std::fmt::Formatter;

use derive_more::Deref;
use diesel_derive_enum::DbEnum;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

pub trait ManagedRoleValidator: std::ops::Deref
where
    String: PartialEq<<Self as std::ops::Deref>::Target>,
{
    fn is_valid(&self, valid_roles: &Vec<String>) -> bool {
        for role in valid_roles {
            if role == &**self {
                return true;
            }
        }

        false
    }
}

/// Google Cloud Platform roles
#[derive(Debug, Clone, Deref, Eq, PartialEq, Serialize, Deserialize, JsonSchema, ToSchema)]
#[schema(example = "roles/compute.admin")]
pub struct GcpRole(pub String);
impl ManagedRoleValidator for GcpRole {}

/// Amazon Web Service policy ARN
#[derive(Debug, Clone, Deref, Eq, PartialEq, Serialize, Deserialize, JsonSchema, ToSchema)]
#[schema(example = "arn:aws:iam::aws:policy/AmazonEC2FullAccess")]
pub struct AwsPolicy(pub String);
impl ManagedRoleValidator for AwsPolicy {}

/// Cloudflare role
#[derive(
    Debug, Copy, Clone, Eq, PartialEq, Hash, Serialize, Deserialize, DbEnum, JsonSchema, ToSchema,
)]
pub enum CloudflareRole {
    Administrator,
    AdministratorReadOnly,
    Analytics,
    AuditLogsViewer,
    Billing,
    CachePurge,
    CloudflareAccess,
    CloudflareGateway,
    CloudflareImages,
    CloudflareStream,
    CloudflareWorkersAdmin,
    CloudflareZeroTrust,
    CloudflareZeroTrustPii,
    CloudflareZeroTrustReadOnly,
    CloudflareZeroTrustReporting,
    Dns,
    Firewall,
    LoadBalancer,
    LogShare,
    LogShareReader,
}

impl Display for CloudflareRole {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Administrator => write!(f, "Administrator"),
            Self::AdministratorReadOnly => write!(f, "Administrator Read Only"),
            Self::Analytics => write!(f, "Analytics"),
            Self::AuditLogsViewer => write!(f, "Audit Logs Viewer"),
            Self::Billing => write!(f, "Billing"),
            Self::CachePurge => write!(f, "Cache Purge"),
            Self::CloudflareAccess => write!(f, "Cloudflare Access"),
            Self::CloudflareGateway => write!(f, "Cloudflare Gateway"),
            Self::CloudflareImages => write!(f, "Cloudflare Images"),
            Self::CloudflareStream => write!(f, "Cloudflare Stream"),
            Self::CloudflareWorkersAdmin => write!(f, "Cloudflare Workers Admin"),
            Self::CloudflareZeroTrust => write!(f, "Cloudflare Zero Trust"),
            Self::CloudflareZeroTrustPii => write!(f, "Cloudflare Zero Trust PII"),
            Self::CloudflareZeroTrustReadOnly => write!(f, "Cloudflare Zero Trust Read Only"),
            Self::CloudflareZeroTrustReporting => write!(f, "Cloudflare Zero Trust Reporting"),
            Self::Dns => write!(f, "DNS"),
            Self::Firewall => write!(f, "Firewall"),
            Self::LoadBalancer => write!(f, "Load Balancer"),
            Self::LogShare => write!(f, "Log Share"),
            Self::LogShareReader => write!(f, "Log Share Reader"),
        }
    }
}
