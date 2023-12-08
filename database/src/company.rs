#![allow(clippy::extra_unused_lifetimes)]

use common::AccessRequestState;
use common::AccessRole;
use diesel::prelude::*;
use diesel_autoincrement_new_struct::apply;
use serde::Deserialize;
use serde::Serialize;
pub use NewCompany as New;

use crate::api_token::ApiToken;
use crate::company_cloudflare_account::CompanyCloudflareAccount;
use crate::company_role::CompanyRole;
use crate::schema::access_requests;
use crate::schema::api_tokens;
use crate::schema::companies;
use crate::schema::company_aws_accounts;
use crate::schema::company_cloudflare_accounts;
use crate::schema::company_gcp_projects;
use crate::schema::company_policies;
use crate::schema::users;
use crate::schema::users_companies;
use crate::schema::users_companies_roles;
use crate::user_company::UserCompany;
use crate::user_company_role::UserCompanyRole;
use crate::worker_key::WorkerKey;
use crate::CompanyAwsAccount;
use crate::CompanyGcpProject;
use crate::CompanyPolicy;
use crate::Result;
use crate::User;
use crate::UserWithRoles;

#[derive(Identifiable)]
#[apply(NewInsertable!)]
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, AsChangeset)]
#[diesel(table_name = companies)]
pub struct Company {
    pub id: i32,
    pub name: String,
    pub domain: String,
    pub root_user: String,
}

impl From<Company> for common_platform::Company {
    fn from(c: Company) -> Self {
        Self {
            id: c.id,
            name: c.name,
            domain: c.domain,
            root_user_email: c.root_user,
            root_user_first_name: None,
            root_user_last_name: None,
            api_token: None,
            worker_key: None,
        }
    }
}

impl From<common_platform::NewCompany> for New {
    fn from(c: common_platform::NewCompany) -> Self {
        Self {
            name: c.name,
            domain: c.domain,
            root_user: c.root_user_email,
        }
    }
}

impl Company {
    pub fn create(connection: &mut SqliteConnection, company: &New) -> Result<Self> {
        let company: Self = diesel::insert_into(companies::table)
            .values(company)
            .get_result(connection)?;

        WorkerKey::create(connection, company.id)?;
        ApiToken::create(connection, company.id)?;

        Ok(company)
    }

    pub fn all(connection: &mut SqliteConnection) -> Result<Vec<Self>> {
        companies::table.get_results(connection)
    }

    pub fn read(connection: &mut SqliteConnection, id: i32) -> Result<Self> {
        companies::table
            .filter(companies::dsl::id.eq(id))
            .first(connection)
    }

    pub fn to_api_response(
        &self,
        connection: &mut SqliteConnection,
    ) -> Result<common_platform::Company> {
        let root_user: User = User::read_by_email(connection, &self.root_user)?;
        let company = common_platform::Company {
            id: self.id,
            name: self.name.clone(),
            domain: self.domain.clone(),
            root_user_email: self.root_user.clone(),
            root_user_first_name: Option::from(root_user.first_name),
            root_user_last_name: Option::from(root_user.last_name),
            api_token: ApiToken::read_by_company_id(connection, self.id)
                .ok()
                .map(|t| t.token),
            worker_key: WorkerKey::read_by_company_id(connection, self.id)
                .ok()
                .map(|k| k.key),
        };

        Ok(company)
    }

    pub fn update(&self, connection: &mut SqliteConnection) -> Result<Self> {
        self.save_changes(connection)
    }

    pub fn read_by_domain(connection: &mut SqliteConnection, domain: &str) -> Result<Option<Self>> {
        companies::table
            .filter(companies::dsl::domain.eq(domain))
            .first(connection)
            .optional()
    }

    pub fn delete(connection: &mut SqliteConnection, id: i32) -> Result<usize> {
        use companies::dsl;

        diesel::delete(dsl::companies.filter(dsl::id.eq(id))).execute(connection)
    }
}

impl Company {
    pub fn api_token(&self, connection: &mut SqliteConnection) -> Result<ApiToken> {
        api_tokens::table
            .filter(api_tokens::dsl::company_id.eq(self.id))
            .first(connection)
    }

    pub fn access_request_ids(
        &self,
        connection: &mut SqliteConnection,
        state: AccessRequestState,
        limit: i64,
    ) -> Result<Vec<String>> {
        access_requests::table
            .filter(access_requests::dsl::company_id.eq(self.id))
            .filter(access_requests::dsl::state.eq(state))
            .order_by(access_requests::dsl::timestamp.desc())
            .limit(limit)
            .select(access_requests::dsl::id)
            .load::<_>(connection)
    }

    pub fn policies(&self, connection: &mut SqliteConnection) -> Result<Vec<CompanyPolicy>> {
        companies::table
            .inner_join(company_policies::table)
            .filter(company_policies::dsl::company_id.eq(self.id))
            .select((
                company_policies::dsl::id,
                company_policies::dsl::company_id,
                company_policies::dsl::name,
                company_policies::dsl::policy,
                company_policies::dsl::description,
            ))
            .load::<_>(connection)
    }

    pub fn aws_accounts(
        &self,
        connection: &mut SqliteConnection,
    ) -> Result<Vec<CompanyAwsAccount>> {
        companies::table
            .inner_join(company_aws_accounts::table)
            .filter(company_aws_accounts::dsl::company_id.eq(self.id))
            .select((
                company_aws_accounts::dsl::id,
                company_aws_accounts::dsl::company_id,
                company_aws_accounts::dsl::aws_account_alias,
                company_aws_accounts::dsl::approvals_required,
                company_aws_accounts::dsl::admin_approval_required,
            ))
            .load::<_>(connection)
    }

    pub fn cloudflare_accounts(
        &self,
        connection: &mut SqliteConnection,
    ) -> Result<Vec<CompanyCloudflareAccount>> {
        companies::table
            .inner_join(company_cloudflare_accounts::table)
            .filter(company_cloudflare_accounts::dsl::company_id.eq(self.id))
            .select((
                company_cloudflare_accounts::dsl::id,
                company_cloudflare_accounts::dsl::company_id,
                company_cloudflare_accounts::dsl::cloudflare_account_alias,
                company_cloudflare_accounts::dsl::approvals_required,
                company_cloudflare_accounts::dsl::admin_approval_required,
            ))
            .load::<_>(connection)
    }

    pub fn gcp_projects(
        &self,
        connection: &mut SqliteConnection,
    ) -> Result<Vec<CompanyGcpProject>> {
        companies::table
            .inner_join(company_gcp_projects::table)
            .filter(company_gcp_projects::dsl::company_id.eq(self.id))
            .select((
                company_gcp_projects::dsl::id,
                company_gcp_projects::dsl::company_id,
                company_gcp_projects::dsl::gcp_project,
                company_gcp_projects::dsl::approvals_required,
                company_gcp_projects::dsl::admin_approval_required,
            ))
            .load::<_>(connection)
    }

    pub fn users(&self, connection: &mut SqliteConnection) -> Result<Vec<User>> {
        users::table
            .inner_join(users_companies::table)
            .filter(users_companies::dsl::company_id.eq(self.id))
            .select((
                users::dsl::id,
                users::dsl::email,
                users::dsl::first_name,
                users::dsl::last_name,
                users::dsl::active,
            ))
            .load::<_>(connection)
    }

    pub fn users_with_roles(
        &self,
        connection: &mut SqliteConnection,
    ) -> Result<Vec<UserWithRoles>> {
        let users = self.users(connection)?;
        let mut users_with_roles = vec![];
        for user in users {
            let roles = user.roles(connection)?;
            users_with_roles.push(UserWithRoles {
                id: user.id,
                email: user.email,
                first_name: user.first_name,
                last_name: user.last_name,
                active: user.active,
                roles,
            });
        }

        Ok(users_with_roles)
    }

    pub fn has_user(&self, connection: &mut SqliteConnection, user: &User) -> Result<bool> {
        let users = self.users(connection)?;

        for u in &users {
            if u.email == user.email {
                return Ok(true);
            }
        }

        Ok(false)
    }

    pub fn add_user(&self, connection: &mut SqliteConnection, user: &User) -> Result<UserCompany> {
        diesel::insert_into(users_companies::table)
            .values(UserCompany {
                user_id: user.id,
                company_id: self.id,
            })
            .get_result(connection)
    }

    pub fn update_roles(
        &self,
        connection: &mut SqliteConnection,
        user: &User,
        roles: &Vec<AccessRole>,
    ) -> Result<Vec<AccessRole>> {
        UserCompanyRole::delete_for_user(connection, self.id, user.id)?;

        for role in roles {
            self.assign_role(connection, user, role)?;
        }

        user.roles(connection)
    }

    pub fn assign_role(
        &self,
        connection: &mut SqliteConnection,
        user: &User,
        role: &AccessRole,
    ) -> Result<UserCompanyRole> {
        let role = CompanyRole::read_by_name(connection, role)?;

        diesel::insert_into(users_companies_roles::table)
            .values(UserCompanyRole {
                user_id: user.id,
                role_id: role.id,
                company_id: self.id,
            })
            .get_result(connection)
    }

    pub fn remove_role(
        &self,
        connection: &mut SqliteConnection,
        user: &User,
        role: &AccessRole,
    ) -> Result<usize> {
        use users_companies_roles::dsl;
        let role = CompanyRole::read_by_name(connection, role)?;

        diesel::delete(
            dsl::users_companies_roles
                .filter(dsl::user_id.eq(user.id))
                .filter(dsl::company_id.eq(self.id))
                .filter(dsl::role_id.eq(role.id)),
        )
        .execute(connection)
    }
}
