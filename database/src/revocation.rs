#![allow(clippy::extra_unused_lifetimes)]

use chrono::DateTime;
use chrono::Utc;
use diesel::prelude::*;
use serde::Deserialize;
use serde::Serialize;

use crate::schema::revocations;
use crate::Result;

#[derive(Debug, Clone, Serialize, Deserialize, Insertable, Queryable)]
#[diesel(table_name = revocations)]
pub struct Revocation {
    pub access_request_id: String,
    pub user: String,
    pub timestamp: DateTime<Utc>,
}

impl Revocation {
    pub fn create(connection: &mut SqliteConnection, revocation: Self) -> Result<Self> {
        diesel::insert_into(revocations::table)
            .values(revocation)
            .on_conflict_do_nothing()
            .get_result(connection)
    }

    pub fn read(connection: &mut SqliteConnection, access_request_id: &str) -> Result<Self> {
        use revocations::dsl::revocations;
        revocations.find(access_request_id).first(connection)
    }
}
