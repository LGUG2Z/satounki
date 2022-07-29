#![allow(clippy::extra_unused_lifetimes, clippy::use_self)]

use common::AccessRole;
use diesel::prelude::*;
use serde::Deserialize;
use serde::Serialize;

use crate::schema::company_roles;
use crate::Result;

#[derive(Debug, Clone, Serialize, Deserialize, Insertable, Queryable)]
#[diesel(table_name = company_roles)]
pub struct CompanyRole {
    pub id: i32,
    pub name: AccessRole,
}

impl CompanyRole {
    pub fn read(connection: &mut SqliteConnection, id: i32) -> Result<Self> {
        company_roles::table
            .filter(company_roles::dsl::id.eq(id))
            .first(connection)
    }

    pub fn read_by_name(connection: &mut SqliteConnection, role: &AccessRole) -> Result<Self> {
        company_roles::table
            .filter(company_roles::dsl::name.eq(role))
            .first(connection)
    }
}
