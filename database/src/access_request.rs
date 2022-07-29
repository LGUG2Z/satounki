#![allow(
    clippy::module_name_repetitions,
    clippy::use_self,
    clippy::extra_unused_lifetimes
)]

use chrono::DateTime;
use chrono::Utc;
use common::AccessRequestState;
use diesel::prelude::*;
use serde::Deserialize;
use serde::Serialize;

use crate::schema::access_requests;
use crate::Result;

#[derive(
    Debug, Clone, Serialize, Deserialize, Identifiable, Insertable, Queryable, AsChangeset,
)]
#[diesel(table_name = access_requests)]
pub struct AccessRequest {
    pub id: String,
    pub company_id: i32,
    pub requester: String,
    pub timestamp: DateTime<Utc>,
    pub duration: i32,
    pub approved: bool,
    pub access_expiry: Option<DateTime<Utc>>,
    pub state: AccessRequestState,
    pub modified: DateTime<Utc>,
    pub req_alias: String,
    pub policy: String,
}

impl AccessRequest {
    pub fn create(connection: &mut SqliteConnection, new_access_request: &Self) -> Result<Self> {
        diesel::insert_into(access_requests::table)
            .values(new_access_request)
            .get_result(connection)
    }

    pub fn read(connection: &mut SqliteConnection, id: &str) -> Result<Self> {
        use access_requests::dsl::access_requests;
        access_requests.find(id).first(connection)
    }

    pub fn read_from_alias(
        connection: &mut SqliteConnection,
        alias: &str,
        company_id: i32,
    ) -> Result<Self> {
        access_requests::table
            .filter(access_requests::dsl::req_alias.eq(alias))
            .filter(access_requests::dsl::company_id.eq(company_id))
            .first(connection)
    }

    pub fn update(&self, connection: &mut SqliteConnection) -> Result<Self> {
        self.save_changes(connection)
    }

    pub fn expired_and_active(
        connection: &mut SqliteConnection,
        company_id: i32,
    ) -> Result<Vec<Self>> {
        let now = Utc::now();

        access_requests::table
            .filter(access_requests::dsl::company_id.eq(company_id))
            .filter(access_requests::dsl::access_expiry.le(now))
            .filter(access_requests::dsl::state.eq(AccessRequestState::Active))
            .load::<_>(connection)
    }

    pub fn unexpired_and_revoked(
        connection: &mut SqliteConnection,
        company_id: i32,
    ) -> Result<Vec<Self>> {
        let now = Utc::now();

        access_requests::table
            .filter(access_requests::dsl::company_id.eq(company_id))
            .filter(access_requests::dsl::access_expiry.ge(now))
            .filter(access_requests::dsl::state.eq(AccessRequestState::Revoked))
            .load::<_>(connection)
    }

    pub fn unexpired_and_completed(
        connection: &mut SqliteConnection,
        company_id: i32,
    ) -> Result<Vec<Self>> {
        let now = Utc::now();

        access_requests::table
            .filter(access_requests::dsl::company_id.eq(company_id))
            .filter(access_requests::dsl::access_expiry.gt(now))
            .filter(access_requests::dsl::state.eq(AccessRequestState::Completed))
            .load::<_>(connection)
    }

    pub fn approved_and_pending(
        connection: &mut SqliteConnection,
        company_id: i32,
    ) -> Result<Vec<Self>> {
        access_requests::table
            .filter(access_requests::dsl::company_id.eq(company_id))
            .filter(access_requests::dsl::approved.eq(true))
            .filter(access_requests::dsl::state.eq(AccessRequestState::Pending))
            .load::<_>(connection)
    }
}
