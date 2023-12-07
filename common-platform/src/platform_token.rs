use common_macros::route_request_response;

/// Platform API token used for automation
#[apply(crate::Schema!)]
pub struct PlatformToken {
    /// Token
    #[schema(example = "super-duper-secret-platform-token")]
    pub token: String,
}

route_request_response! {
    #[Get] PlatformToken() -> PlatformToken,
    #[Put] PlatformToken() -> PlatformToken,
}
