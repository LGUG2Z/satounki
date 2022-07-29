#![allow(clippy::extra_unused_lifetimes)]

use chrono::DateTime;
use chrono::Utc;
use diesel::prelude::*;
use serde::Deserialize;
use serde::Serialize;

use crate::schema::rejections;
use crate::Result;

#[derive(Debug, Clone, Serialize, Deserialize, Insertable, Queryable)]
#[diesel(table_name = rejections)]
pub struct Rejection {
    pub access_request_id: String,
    pub user: String,
    pub timestamp: DateTime<Utc>,
}

impl Rejection {
    pub fn create(connection: &mut SqliteConnection, rejection: Self) -> Result<Self> {
        diesel::insert_into(rejections::table)
            .values(rejection)
            .get_result(connection)
    }

    pub fn read(connection: &mut SqliteConnection, access_request_id: &str) -> Result<Self> {
        use rejections::dsl::rejections;
        rejections.find(access_request_id).first(connection)
    }
}
