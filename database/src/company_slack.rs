#![allow(clippy::extra_unused_lifetimes, clippy::use_self)]

use diesel::prelude::*;
use serde::Deserialize;
use serde::Serialize;

use crate::schema::company_slack;
use crate::Result;

#[derive(
    Debug, Clone, Serialize, Deserialize, Identifiable, Insertable, Queryable, AsChangeset,
)]
#[diesel(table_name = company_slack)]
#[diesel(primary_key(company_id))]
pub struct CompanySlack {
    pub company_id: i32,
    pub team_id: String,
    pub team_name: String,
    pub channel_id: String,
    pub access_token: String,
    pub incoming_webhook: String,
}

impl CompanySlack {
    pub fn create(connection: &mut SqliteConnection, slack: Self) -> Result<Self> {
        diesel::insert_into(company_slack::table)
            .values(slack.clone())
            .on_conflict(company_slack::dsl::company_id)
            .do_update()
            .set(slack)
            .get_result(connection)
    }

    pub fn read(connection: &mut SqliteConnection, id: i32) -> Result<Self> {
        company_slack::dsl::company_slack.find(id).first(connection)
    }

    pub fn read_by_team_id(connection: &mut SqliteConnection, team_id: &str) -> Result<Self> {
        company_slack::dsl::company_slack
            .filter(company_slack::dsl::team_id.eq(team_id))
            .first(connection)
    }

    pub fn update(&self, connection: &mut SqliteConnection) -> Result<Self> {
        self.save_changes(connection)
    }

    pub fn delete(&self, connection: &mut SqliteConnection) -> Result<usize> {
        use company_slack::dsl;

        diesel::delete(dsl::company_slack.filter(dsl::company_id.eq(&self.company_id)))
            .execute(connection)
    }
}
