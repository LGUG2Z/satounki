#![allow(clippy::extra_unused_lifetimes, clippy::use_self)]

use diesel::prelude::*;
use serde::Deserialize;
use serde::Serialize;

use crate::schema::company_policies;
use crate::Result;

#[derive(
    Debug, Clone, Serialize, Deserialize, Identifiable, Insertable, Queryable, AsChangeset,
)]
#[diesel(table_name = company_policies)]
pub struct CompanyPolicy {
    pub id: String,
    pub company_id: i32,
    pub name: String,
    pub policy: String,
    pub description: String,
}

impl CompanyPolicy {
    pub fn create(connection: &mut SqliteConnection, policy: &Self) -> Result<Self> {
        diesel::insert_into(company_policies::table)
            .values(policy)
            .get_result(connection)
    }

    pub fn read(connection: &mut SqliteConnection, id: &str) -> Result<Self> {
        company_policies::dsl::company_policies
            .find(id)
            .first(connection)
    }

    pub fn read_by_name(
        connection: &mut SqliteConnection,
        name: &str,
        company_id: i32,
    ) -> Result<Self> {
        company_policies::table
            .filter(company_policies::dsl::company_id.eq(company_id))
            .filter(company_policies::dsl::name.eq(name))
            .first(connection)
    }

    pub fn update(&self, connection: &mut SqliteConnection) -> Result<Self> {
        self.save_changes(connection)
    }

    pub fn delete(&self, connection: &mut SqliteConnection) -> Result<usize> {
        use company_policies::dsl;

        diesel::delete(dsl::company_policies.filter(dsl::id.eq(&self.id))).execute(connection)
    }
}
