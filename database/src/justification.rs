#![allow(clippy::extra_unused_lifetimes)]

use diesel::prelude::*;
use serde::Deserialize;
use serde::Serialize;

use crate::schema::justifications;
use crate::Result;

#[derive(Debug, Clone, Serialize, Deserialize, Insertable, Queryable)]
#[diesel(table_name = justifications)]
pub struct Justification {
    pub access_request_id: String,
    pub justification: String,
}

impl Justification {
    pub fn create(connection: &mut SqliteConnection, justification: Self) -> Result<Self> {
        diesel::insert_into(justifications::table)
            .values(justification)
            .get_result(connection)
    }

    pub fn read(connection: &mut SqliteConnection, access_request_id: &str) -> Result<Self> {
        use justifications::dsl::justifications;
        justifications.find(access_request_id).first(connection)
    }
}
