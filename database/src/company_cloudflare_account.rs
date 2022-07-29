#![allow(clippy::extra_unused_lifetimes, clippy::use_self)]

use common::CloudflareAccount;
use diesel::prelude::*;
use serde::Deserialize;
use serde::Serialize;

use crate::schema::company_cloudflare_accounts;
use crate::Result;

#[derive(
    Debug, Clone, Serialize, Deserialize, Identifiable, Insertable, Queryable, AsChangeset,
)]
#[diesel(table_name = company_cloudflare_accounts)]
pub struct CompanyCloudflareAccount {
    pub id: String,
    pub company_id: i32,
    pub cloudflare_account_alias: String,
    pub approvals_required: i32,
    pub admin_approval_required: bool,
}

impl From<CompanyCloudflareAccount> for CloudflareAccount {
    fn from(a: CompanyCloudflareAccount) -> Self {
        Self {
            id: a.id,
            account: a.cloudflare_account_alias,
            approvals_required: a.approvals_required,
            admin_approval_required: a.admin_approval_required,
        }
    }
}

impl CompanyCloudflareAccount {
    pub fn create(connection: &mut SqliteConnection, account: Self) -> Result<Self> {
        diesel::insert_into(company_cloudflare_accounts::table)
            .values(account)
            .on_conflict_do_nothing()
            .get_result(connection)
    }

    pub fn read(connection: &mut SqliteConnection, id: &str) -> Result<Self> {
        company_cloudflare_accounts::dsl::company_cloudflare_accounts
            .find(id)
            .first(connection)
    }

    pub fn read_by_alias(
        connection: &mut SqliteConnection,
        name: &str,
        company_id: i32,
    ) -> Result<Self> {
        company_cloudflare_accounts::table
            .filter(company_cloudflare_accounts::dsl::company_id.eq(company_id))
            .filter(company_cloudflare_accounts::dsl::cloudflare_account_alias.eq(name))
            .first(connection)
    }

    pub fn update(&self, connection: &mut SqliteConnection) -> Result<Self> {
        self.save_changes(connection)
    }

    pub fn delete(&self, connection: &mut SqliteConnection) -> Result<usize> {
        use company_cloudflare_accounts::dsl;

        diesel::delete(dsl::company_cloudflare_accounts.filter(dsl::id.eq(&self.id)))
            .execute(connection)
    }
}
