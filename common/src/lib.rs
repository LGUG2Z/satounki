#[macro_use]
extern crate macro_rules_attribute;

pub use access_request_state::*;
pub use error_response::*;
pub use permissions_action::*;
pub use policy::*;
pub use request_alias::*;
pub use request_policy::*;
pub use requests::*;
pub use roles::*;
pub use settings::*;
pub use user_aliases::*;
pub use user_roles::*;
pub use user_status::*;
pub use user_token::*;
pub use websocket::*;

attribute_alias! {
    #[apply(Schema!)] = #[derive(Debug, Clone, serde::Serialize, serde::Deserialize, display_json::DisplayAsJsonPretty, schemars::JsonSchema, utoipa::ToSchema)];
    #[apply(Terraform!)] = #[macro_rules_derive(common_macros::terraform_resource!)];
    #[apply(New!)] = #[macro_rules_derive(common_macros::new_resource!)];
}

mod access_request_state;
mod error_response;
mod permissions_action;
mod policy;
mod request_alias;
mod request_policy;
mod requests;
mod roles;
mod settings;
mod user_aliases;
mod user_roles;
mod user_status;
mod user_token;
mod websocket;
