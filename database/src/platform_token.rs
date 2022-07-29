#![allow(clippy::extra_unused_lifetimes)]

use common_platform::PlatformTokenScope;
use diesel::prelude::*;
use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

use crate::schema::platform_tokens;
use crate::Result;

#[derive(Debug, Clone, Serialize, Deserialize, Insertable, Queryable, AsChangeset)]
#[diesel(table_name = platform_tokens)]
pub struct PlatformToken {
    pub token: String,
    pub scope: PlatformTokenScope,
}

impl PlatformToken {
    pub fn create(connection: &mut SqliteConnection, scope: PlatformTokenScope) -> Result<Self> {
        diesel::insert_into(platform_tokens::table)
            .values(Self {
                token: Uuid::new_v4().to_string(),
                scope,
            })
            .get_result(connection)
    }

    pub fn create_from_env(
        connection: &mut SqliteConnection,
        scope: PlatformTokenScope,
        token: &str,
    ) -> Result<Self> {
        diesel::insert_into(platform_tokens::table)
            .values(Self {
                token: token.to_string(),
                scope,
            })
            .get_result(connection)
    }

    pub fn read(connection: &mut SqliteConnection, token: &str) -> Result<Self> {
        platform_tokens::table.find(token).first(connection)
    }

    pub fn read_by_scope(
        connection: &mut SqliteConnection,
        scope: PlatformTokenScope,
    ) -> Result<Self> {
        use platform_tokens::dsl;

        platform_tokens::table
            .filter(dsl::scope.eq(scope))
            .first(connection)
    }

    pub fn delete(connection: &mut SqliteConnection, scope: PlatformTokenScope) -> Result<usize> {
        use platform_tokens::dsl;

        diesel::delete(dsl::platform_tokens.filter(dsl::scope.eq(scope))).execute(connection)
    }

    pub fn replace(connection: &mut SqliteConnection, scope: PlatformTokenScope) -> Result<Self> {
        Self::delete(connection, scope)?;
        Self::create(connection, scope)
    }
}
