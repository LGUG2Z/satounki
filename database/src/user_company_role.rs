#![allow(clippy::extra_unused_lifetimes)]

use diesel::prelude::*;
use serde::Deserialize;
use serde::Serialize;

use crate::schema::users_companies_roles;
use crate::Result;

#[derive(Debug, Clone, Serialize, Deserialize, Insertable, Queryable)]
#[diesel(table_name = users_companies_roles)]
pub struct UserCompanyRole {
    pub user_id: i32,
    pub company_id: i32,
    pub role_id: i32,
}

impl UserCompanyRole {
    pub fn create(connection: &mut SqliteConnection, user_company_role: Self) -> Result<Self> {
        diesel::insert_into(users_companies_roles::table)
            .values(user_company_role)
            .get_result(connection)
    }

    pub fn delete_for_user(
        connection: &mut SqliteConnection,
        user_id: i32,
        company_id: i32,
    ) -> Result<usize> {
        use users_companies_roles::dsl;

        diesel::delete(
            dsl::users_companies_roles
                .filter(dsl::user_id.eq(user_id))
                .filter(dsl::company_id.eq(company_id)),
        )
        .execute(connection)
    }

    pub fn delete(
        connection: &mut SqliteConnection,
        user_id: i32,
        company_id: i32,
        role_id: i32,
    ) -> Result<usize> {
        use users_companies_roles::dsl;

        diesel::delete(
            dsl::users_companies_roles
                .filter(dsl::user_id.eq(user_id))
                .filter(dsl::role_id.eq(role_id))
                .filter(dsl::company_id.eq(company_id)),
        )
        .execute(connection)
    }
}
