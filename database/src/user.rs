#![allow(clippy::extra_unused_lifetimes)]

use common::AccessRole;
use diesel::prelude::*;
use diesel_autoincrement_new_struct::apply;
use diesel_autoincrement_new_struct::NewInsertable;
use serde::Deserialize;
use serde::Serialize;

use crate::company_role::CompanyRole;
use crate::schema::companies;
use crate::schema::company_roles;
use crate::schema::user_tokens;
use crate::schema::users;
use crate::schema::users_companies;
use crate::schema::users_companies_roles;
use crate::Company;
use crate::Result;
use crate::UserAlias;
use crate::UserToken;

#[derive(Identifiable)]
#[apply(NewInsertable!)]
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, AsChangeset)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub active: bool,
}

pub use NewUser as New;

#[derive(Debug, Clone, Serialize)]
pub struct WithRoles {
    pub id: i32,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub active: bool,
    pub roles: Vec<AccessRole>,
}

impl User {
    pub fn create(connection: &mut SqliteConnection, user: &New) -> Result<Self> {
        diesel::insert_into(users::table)
            .values(user)
            .on_conflict(users::dsl::email)
            .do_update()
            .set(user)
            .get_result(connection)
    }

    pub fn read(connection: &mut SqliteConnection, id: i32) -> Result<Self> {
        users::table.filter(users::dsl::id.eq(id)).first(connection)
    }

    pub fn update(&self, connection: &mut SqliteConnection) -> Result<Self> {
        self.save_changes(connection)
    }

    pub fn read_by_email(connection: &mut SqliteConnection, email: &str) -> Result<Self> {
        users::table
            .filter(users::dsl::email.eq(email))
            .first(connection)
    }
}

impl User {
    pub fn user_token(&self, connection: &mut SqliteConnection) -> Result<UserToken> {
        user_tokens::table
            .filter(user_tokens::dsl::user_id.eq(self.id))
            .first(connection)
    }
    pub fn company(&self, connection: &mut SqliteConnection) -> Result<Company> {
        users_companies::table
            .inner_join(companies::table)
            .filter(users_companies::dsl::user_id.eq(self.id))
            .select((
                companies::dsl::id,
                companies::dsl::name,
                companies::dsl::domain,
                companies::dsl::root_user,
            ))
            .first(connection)
    }

    pub fn belongs_to_company(
        &self,
        connection: &mut SqliteConnection,
        company_id: i32,
    ) -> Result<bool> {
        let users: Vec<String> = users::table
            .inner_join(users_companies::table)
            .filter(users_companies::dsl::user_id.eq(self.id))
            .filter(users_companies::dsl::company_id.eq(company_id))
            .select(users::dsl::email)
            .load::<_>(connection)?;

        Ok(!users.is_empty())
    }

    pub fn roles(&self, connection: &mut SqliteConnection) -> Result<Vec<AccessRole>> {
        let company_roles: Vec<CompanyRole> = company_roles::table
            .inner_join(users_companies_roles::table)
            .filter(users_companies_roles::dsl::user_id.eq(self.id))
            .select((company_roles::dsl::id, company_roles::dsl::name))
            .load::<_>(connection)?;

        Ok(company_roles.into_iter().map(|r| r.name).collect())
    }

    pub fn aliases(&self, connection: &mut SqliteConnection) -> Result<UserAlias> {
        UserAlias::read(connection, self.id)
    }

    pub fn can_make_requests(&self, connection: &mut SqliteConnection) -> Result<bool> {
        for company_role in self.roles(connection)? {
            if company_role == AccessRole::User {
                return Ok(true);
            }
        }

        Ok(false)
    }

    pub fn is_approver(&self, connection: &mut SqliteConnection) -> Result<bool> {
        for company_role in self.roles(connection)? {
            if company_role == AccessRole::Approver {
                return Ok(true);
            }
        }

        Ok(false)
    }

    pub fn is_administrator(&self, connection: &mut SqliteConnection) -> Result<bool> {
        for company_role in self.roles(connection)? {
            if company_role == AccessRole::Administrator {
                return Ok(true);
            }
        }

        Ok(false)
    }
}
