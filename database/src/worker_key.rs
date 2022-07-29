#![allow(clippy::extra_unused_lifetimes)]

use diesel::prelude::*;
use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

use crate::schema::worker_keys;
use crate::Result;

#[derive(Debug, Clone, Serialize, Deserialize, Insertable, Queryable, AsChangeset)]
#[diesel(table_name = worker_keys)]
pub struct WorkerKey {
    pub company_id: i32,
    pub key: String,
}

impl WorkerKey {
    pub fn create(connection: &mut SqliteConnection, company_id: i32) -> Result<Self> {
        diesel::insert_into(worker_keys::table)
            .values(Self {
                company_id,
                key: format!("swk-{}", Uuid::new_v4()),
            })
            .get_result(connection)
    }

    pub fn read(connection: &mut SqliteConnection, company_id: i32) -> Result<Option<Self>> {
        worker_keys::table
            .filter(worker_keys::dsl::company_id.eq(company_id))
            .first(connection)
            .optional()
    }

    pub fn delete(connection: &mut SqliteConnection, company_id: i32) -> Result<usize> {
        use worker_keys::dsl;

        diesel::delete(dsl::worker_keys.filter(dsl::company_id.eq(company_id))).execute(connection)
    }

    pub fn replace(connection: &mut SqliteConnection, company_id: i32) -> Result<Self> {
        Self::delete(connection, company_id)?;
        Self::create(connection, company_id)
    }
}
