use common_macros::body;
use common_macros::response;

use crate::Schema;

/// Access request for policy permissions
#[apply(Schema!)]
pub struct PolicyRequest {
    /// Duration of the request in minutes
    #[schema(example = 60)]
    pub minutes: i32,
    /// Reason for the request
    #[schema(example = "to investigate incident #4321 on production")]
    pub justification: String,
    /// AWS account to grant permissions on, if the policy includes AWS policy ARNs
    #[schema(example = "cool-company-production")]
    pub aws_account: Option<String>,
    /// Cloudflare account to grant permissions on, if the policy includes Cloudflare roles
    #[schema(example = "cool-company.com")]
    pub cloudflare_account: Option<String>,
    /// GCP project to grant permissions on, if the policy includes GCP roles
    #[schema(example = "cool-company-production")]
    pub gcp_project: Option<String>,
}

/// Access request confirmation
#[apply(Schema!)]
pub struct PolicyRequestConfirmation {
    /// UUID generated by Satounki
    #[schema(example = "5da0230c-4eeb-4840-9ffa-d97f45a12182")]
    pub request_id: String,
    /// Human-friendly alias generated by Satounki
    #[schema(example = "samir-crazy-skyline-fish")]
    pub request_alias: String,
}

body! {
    #[Post] RequestPolicy -> PolicyRequest
}

response! {
    #[Post] RequestPolicy -> PolicyRequestConfirmation
}
