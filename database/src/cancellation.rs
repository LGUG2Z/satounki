#![allow(clippy::extra_unused_lifetimes)]

use chrono::DateTime;
use chrono::Utc;
use diesel::prelude::*;
use serde::Deserialize;
use serde::Serialize;

use crate::schema::cancellations;
use crate::Result;

#[derive(Debug, Clone, Serialize, Deserialize, Insertable, Queryable)]
#[diesel(table_name = cancellations)]
pub struct Cancellation {
    pub access_request_id: String,
    pub user: String,
    pub timestamp: DateTime<Utc>,
}

impl Cancellation {
    pub fn create(connection: &mut SqliteConnection, cancellation: Self) -> Result<Self> {
        diesel::insert_into(cancellations::table)
            .values(cancellation)
            .on_conflict_do_nothing()
            .get_result(connection)
    }

    pub fn read(connection: &mut SqliteConnection, access_request_id: &str) -> Result<Self> {
        use cancellations::dsl::cancellations;
        cancellations.find(access_request_id).first(connection)
    }
}
