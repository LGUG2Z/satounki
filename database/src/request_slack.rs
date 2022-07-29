#![allow(clippy::extra_unused_lifetimes)]

use diesel::prelude::*;
use serde::Deserialize;
use serde::Serialize;

use crate::schema::requests_slack;
use crate::Result;

#[derive(Debug, Clone, Serialize, Deserialize, Insertable, Queryable)]
#[diesel(table_name = requests_slack)]
pub struct RequestSlack {
    pub access_request_id: String,
    pub company_id: i32,
    pub channel_id: String,
    pub ts: String,
}

impl RequestSlack {
    pub fn create(connection: &mut SqliteConnection, request: Self) -> Result<Self> {
        diesel::insert_into(requests_slack::table)
            .values(request)
            .get_result(connection)
    }

    pub fn read(connection: &mut SqliteConnection, access_request_id: &str) -> Result<Self> {
        use requests_slack::dsl::requests_slack;
        requests_slack.find(access_request_id).first(connection)
    }
}
