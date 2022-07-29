#![allow(clippy::extra_unused_lifetimes)]

use diesel::prelude::*;
use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

use crate::schema::api_tokens;
use crate::Result;

#[derive(Debug, Clone, Serialize, Deserialize, Insertable, Queryable, AsChangeset)]
#[diesel(table_name = api_tokens)]
pub struct ApiToken {
    pub token: String,
    pub company_id: i32,
}

impl ApiToken {
    pub fn create(connection: &mut SqliteConnection, company_id: i32) -> Result<Self> {
        diesel::insert_into(api_tokens::table)
            .values(Self {
                token: Uuid::new_v4().to_string(),
                company_id,
            })
            .get_result(connection)
    }

    pub fn read(connection: &mut SqliteConnection, token: &str) -> Result<Self> {
        api_tokens::table.find(token).first(connection)
    }

    pub fn delete(connection: &mut SqliteConnection, company_id: i32) -> Result<usize> {
        use api_tokens::dsl;

        diesel::delete(dsl::api_tokens.filter(dsl::company_id.eq(company_id))).execute(connection)
    }

    pub fn replace(connection: &mut SqliteConnection, company_id: i32) -> Result<Self> {
        Self::delete(connection, company_id)?;
        Self::create(connection, company_id)
    }
}
