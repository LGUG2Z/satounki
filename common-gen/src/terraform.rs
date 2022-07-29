use common::AccessRole;
use common::AwsAccount;
use common::CloudflareAccount;
use common::GcpProject;
use common::Policy;
use common::UserAliases;
use common_platform::Company;

pub struct Resource<'a> {
    pub name: &'a str,
    pub doc: &'a str,
    pub members: Vec<String>,
}

pub fn platform_resources<'a>() -> Vec<Resource<'a>> {
    vec![Resource {
        name: "company",
        doc: Company::terraform_resource_members().0,
        members: Company::terraform_resource_members().1,
    }]
}

pub fn resources<'a>() -> Vec<Resource<'a>> {
    vec![
        Resource {
            name: "policy",
            doc: Policy::terraform_resource_members().0,
            members: Policy::terraform_resource_members().1,
        },
        Resource {
            name: "aws_account",
            doc: AwsAccount::terraform_resource_members().0,
            members: AwsAccount::terraform_resource_members().1,
        },
        Resource {
            name: "cloudflare_account",
            doc: CloudflareAccount::terraform_resource_members().0,
            members: CloudflareAccount::terraform_resource_members().1,
        },
        Resource {
            name: "gcp_project",
            doc: GcpProject::terraform_resource_members().0,
            members: GcpProject::terraform_resource_members().1,
        },
        Resource {
            name: "user_aliases",
            doc: UserAliases::terraform_resource_members().0,
            members: UserAliases::terraform_resource_members().1,
        },
        Resource {
            name: "user_roles",
            doc: AccessRole::terraform_resource_members().0,
            members: AccessRole::terraform_resource_members().1,
        },
    ]
}
