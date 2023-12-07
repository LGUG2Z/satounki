use common_macros::route_request_response;

use crate::New;
use crate::Schema;
use crate::Terraform;

/// Company
#[apply(New!)]
#[apply(Terraform!)]
#[apply(Schema!)]
pub struct Company {
    /// Auto-incrementing integer
    pub id: i32,
    /// Name of the company
    pub name: String,
    /// Email domain of the company (G-Suite etc.)
    pub domain: String,
    /// Company root user's email address
    pub root_user_email: String,
    /// Company root user's first name
    pub root_user_first_name: Option<String>,
    /// Company root user's last name
    pub root_user_last_name: Option<String>,
}

route_request_response! {
    #[Put] Company(Company) -> Company,
    #[Post] Company(Company) -> Company,
    #[Get] Company() -> Company,
    #[Get] Companies() -> Vec<Company>,
}
