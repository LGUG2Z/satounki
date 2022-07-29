#![allow(clippy::extra_unused_lifetimes)]

use diesel::prelude::*;
use serde::Deserialize;
use serde::Serialize;

use crate::schema::users_companies;
use crate::Result;

#[derive(Debug, Clone, Serialize, Deserialize, Insertable, Queryable)]
#[diesel(table_name = users_companies)]
pub struct UserCompany {
    pub user_id: i32,
    pub company_id: i32,
}

impl UserCompany {
    pub fn create(connection: &mut SqliteConnection, user_company: Self) -> Result<Self> {
        diesel::insert_into(users_companies::table)
            .values(user_company)
            .get_result(connection)
    }

    pub fn delete(
        connection: &mut SqliteConnection,
        user_id: i32,
        company_id: i32,
    ) -> Result<usize> {
        use users_companies::dsl;

        diesel::delete(
            dsl::users_companies
                .filter(dsl::user_id.eq(user_id))
                .filter(dsl::company_id.eq(company_id)),
        )
        .execute(connection)
    }
}
