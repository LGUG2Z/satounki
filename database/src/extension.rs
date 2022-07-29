#![allow(clippy::extra_unused_lifetimes)]

use chrono::DateTime;
use chrono::Utc;
use diesel::prelude::*;
use serde::Deserialize;
use serde::Serialize;

use crate::schema::extensions;
use crate::Result;

#[derive(Debug, Clone, Serialize, Deserialize, Insertable, Queryable)]
#[diesel(table_name = extensions)]
pub struct Extension {
    pub access_request_id: String,
    pub user: String,
    pub timestamp: DateTime<Utc>,
    pub duration: i32,
}

impl Extension {
    pub fn create(connection: &mut SqliteConnection, extension: Self) -> Result<Self> {
        diesel::insert_into(extensions::table)
            .values(extension)
            .get_result(connection)
    }

    pub fn read(connection: &mut SqliteConnection, access_request_id: &str) -> Result<Vec<Self>> {
        extensions::table
            .filter(extensions::dsl::access_request_id.eq(access_request_id))
            .load::<_>(connection)
    }
}
