#![allow(clippy::extra_unused_lifetimes, clippy::use_self)]

use common::CloudflareRole;
use diesel::prelude::*;
use serde::Deserialize;
use serde::Serialize;

use crate::schema::cloudflare_requests;
use crate::Result;

#[derive(
    Debug, Clone, Serialize, Deserialize, Insertable, Identifiable, Queryable, AsChangeset,
)]
#[diesel(table_name = cloudflare_requests)]
#[diesel(primary_key(access_request_id, user, role))]
pub struct CloudflareRequest {
    pub access_request_id: String,
    pub company_id: i32,
    pub user: String,
    pub account_alias: String,
    pub role: CloudflareRole,
}

impl CloudflareRequest {
    pub fn create(connection: &mut SqliteConnection, batch: &Vec<Self>) -> Result<usize> {
        diesel::insert_into(cloudflare_requests::table)
            .values(batch)
            .execute(connection)
    }

    pub fn read(connection: &mut SqliteConnection, access_request_id: &str) -> Result<Vec<Self>> {
        cloudflare_requests::table
            .filter(cloudflare_requests::dsl::access_request_id.eq(access_request_id))
            .load::<_>(connection)
    }

    pub fn update(&self, connection: &mut SqliteConnection) -> Result<Self> {
        self.save_changes(connection)
    }

    pub fn delete(connection: &mut SqliteConnection, access_request_id: &str) -> Result<usize> {
        use cloudflare_requests::dsl;

        diesel::delete(
            dsl::cloudflare_requests.filter(dsl::access_request_id.eq(access_request_id)),
        )
        .execute(connection)
    }

    pub fn replace(
        connection: &mut SqliteConnection,
        access_request_id: &str,
        batch: &Vec<Self>,
    ) -> Result<usize> {
        Self::delete(connection, access_request_id)?;
        Self::create(connection, batch)
    }
}
