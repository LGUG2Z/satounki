#![allow(clippy::extra_unused_lifetimes)]

use chrono::DateTime;
use chrono::Utc;
use diesel::prelude::*;
use serde::Deserialize;
use serde::Serialize;

use crate::schema::approvals;
use crate::Result;

#[derive(Debug, Clone, Serialize, Deserialize, Insertable, Queryable)]
#[diesel(table_name = approvals)]
pub struct Approval {
    pub access_request_id: String,
    pub user: String,
    pub timestamp: DateTime<Utc>,
}

impl Approval {
    pub fn create(connection: &mut SqliteConnection, approval: Self) -> Result<Self> {
        diesel::insert_into(approvals::table)
            .values(approval)
            .get_result(connection)
    }

    pub fn read(connection: &mut SqliteConnection, access_request_id: &str) -> Result<Vec<Self>> {
        approvals::table
            .filter(approvals::dsl::access_request_id.eq(access_request_id))
            .load::<_>(connection)
    }
}
