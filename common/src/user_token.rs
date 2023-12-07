use common_macros::route_request_response;

use crate::Schema;

/// User API token for personal use
#[apply(Schema!)]
pub struct UserToken {
    /// Token
    #[schema(example = "super-duper-secret-user-token")]
    pub token: String,
}

route_request_response! {
    #[Get] UserToken() -> UserToken,
    #[Put] UserToken() -> UserToken,
}
