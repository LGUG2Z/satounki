#![allow(clippy::extra_unused_lifetimes)]

use diesel::prelude::*;
use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

use crate::schema::user_tokens;
use crate::Result;

#[derive(Debug, Clone, Serialize, Deserialize, Insertable, Queryable, AsChangeset)]
#[diesel(table_name = user_tokens)]
pub struct UserToken {
    pub token: String,
    pub user_id: i32,
}

impl UserToken {
    pub fn create(connection: &mut SqliteConnection, user_id: i32) -> Result<Self> {
        diesel::insert_into(user_tokens::table)
            .values(Self {
                token: Uuid::new_v4().to_string(),
                user_id,
            })
            .get_result(connection)
    }

    pub fn read(connection: &mut SqliteConnection, token: &str) -> Result<Self> {
        user_tokens::table.find(token).first(connection)
    }

    pub fn delete(connection: &mut SqliteConnection, user_id: i32) -> Result<usize> {
        use user_tokens::dsl;

        diesel::delete(dsl::user_tokens.filter(dsl::user_id.eq(user_id))).execute(connection)
    }

    pub fn replace(connection: &mut SqliteConnection, user_id: i32) -> Result<Self> {
        Self::delete(connection, user_id)?;
        Self::create(connection, user_id)
    }
}
