#![allow(clippy::extra_unused_lifetimes)]

use common::UserAliases;
use diesel::prelude::*;
use serde::Deserialize;
use serde::Serialize;

use crate::schema::user_aliases;
use crate::Result;

#[derive(
    Default, Debug, Clone, Serialize, Deserialize, Insertable, Identifiable, Queryable, AsChangeset,
)]
#[diesel(table_name = user_aliases)]
#[diesel(primary_key(user_id))]
#[diesel(treat_none_as_null = true)]
pub struct UserAlias {
    pub user_id: i32,
    pub aws: Option<String>,
    pub cloudflare: Option<String>,
    pub gcp: Option<String>,
}

impl From<UserAlias> for UserAliases {
    fn from(u: UserAlias) -> Self {
        Self {
            aws: u.aws,
            cloudflare: u.cloudflare,
            gcp: u.gcp,
        }
    }
}

impl UserAlias {
    pub fn create(connection: &mut SqliteConnection, user_alias: &Self) -> Result<Self> {
        diesel::insert_into(user_aliases::table)
            .values(user_alias)
            .on_conflict(user_aliases::dsl::user_id)
            .do_update()
            .set(user_alias)
            .get_result(connection)
    }

    pub fn read(connection: &mut SqliteConnection, user_id: i32) -> Result<Self> {
        user_aliases::dsl::user_aliases
            .find(user_id)
            .first(connection)
    }

    pub fn read_optional(connection: &mut SqliteConnection, user_id: i32) -> Result<Option<Self>> {
        user_aliases::dsl::user_aliases
            .find(user_id)
            .first(connection)
            .optional()
    }

    pub fn update(&self, connection: &mut SqliteConnection) -> Result<Self> {
        self.save_changes(connection)
    }

    pub fn delete(&self, connection: &mut SqliteConnection) -> Result<usize> {
        use user_aliases::dsl;

        diesel::delete(dsl::user_aliases.filter(dsl::user_id.eq(self.user_id))).execute(connection)
    }
}
