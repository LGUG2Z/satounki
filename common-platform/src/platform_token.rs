use common_macros::response;

/// Platform API token used for automation
#[apply(crate::Schema!)]
pub struct PlatformToken {
    /// Token
    #[schema(example = "super-duper-secret-platform-token")]
    pub token: String,
}

response! {
    #[Get] PlatformToken -> PlatformToken,
    #[Put] PlatformToken -> PlatformToken,
}
