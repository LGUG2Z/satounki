#[macro_use]
extern crate macro_rules_attribute;

attribute_alias! {
    #[apply(Schema!)] = #[derive(Debug, Clone, serde::Serialize, serde::Deserialize, display_json::DisplayAsJsonPretty, schemars::JsonSchema, utoipa::ToSchema)];
    #[apply(Terraform!)] = #[macro_rules_derive(common_macros::terraform_resource!)];
    #[apply(New!)] = #[macro_rules_derive(common_macros::new_resource!)];
}

pub use company::*;
pub use error_response::*;
pub use platform_token::*;
pub use platform_token_scope::*;

mod company;
mod error_response;
mod platform_token;
mod platform_token_scope;
