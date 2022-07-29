use common_macros::body;
use common_macros::response;

use crate::Schema;
use crate::Terraform;

/// Service-specific username aliases
#[apply(Terraform!)]
#[apply(Schema!)]
#[serde(rename_all = "snake_case")]
pub struct UserAliases {
    /// Username on Amazon Web Services, may not be an email address
    #[schema(example = "Samir")]
    pub aws: Option<String>,
    /// Email address registered with Cloudflare
    #[schema(example = "samir@cool-company.com")]
    pub cloudflare: Option<String>,
    /// Email address registered with Google Cloud Platform
    #[schema(example = "samir@cool-company.com")]
    pub gcp: Option<String>,
}

body! {
    #[Post] UserAliases -> UserAliases,
    #[Put] UserAliases -> UserAliases,
}

response! {
    #[Get] UserAliases -> UserAliases,
    #[Post] UserAliases -> UserAliases,
    #[Put] UserAliases -> UserAliases,
}
