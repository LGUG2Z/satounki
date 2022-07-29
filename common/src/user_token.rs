use common_macros::response;

use crate::Schema;

/// User API token for personal use
#[apply(Schema!)]
pub struct UserToken {
    /// Token
    #[schema(example = "super-duper-secret-user-token")]
    pub token: String,
}

response! {
    #[Get] UserToken -> UserToken,
    #[Put] UserToken -> UserToken,
}
