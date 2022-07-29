#![allow(clippy::extra_unused_lifetimes, clippy::use_self)]

use common::GcpProject;
use diesel::prelude::*;
use serde::Deserialize;
use serde::Serialize;

use crate::schema::company_gcp_projects;
use crate::Result;

#[derive(
    Debug, Clone, Serialize, Deserialize, Insertable, Identifiable, Queryable, AsChangeset,
)]
#[diesel(table_name = company_gcp_projects)]
pub struct CompanyGcpProject {
    pub id: String,
    pub company_id: i32,
    pub gcp_project: String,
    pub approvals_required: i32,
    pub admin_approval_required: bool,
}

impl From<CompanyGcpProject> for GcpProject {
    fn from(p: CompanyGcpProject) -> Self {
        GcpProject {
            id: p.id,
            project: p.gcp_project,
            approvals_required: p.approvals_required,
            admin_approval_required: p.admin_approval_required,
        }
    }
}

impl CompanyGcpProject {
    pub fn create(connection: &mut SqliteConnection, project: Self) -> Result<Self> {
        diesel::insert_into(company_gcp_projects::table)
            .values(project)
            .on_conflict_do_nothing()
            .get_result(connection)
    }

    pub fn read(connection: &mut SqliteConnection, id: &str) -> Result<Self> {
        company_gcp_projects::dsl::company_gcp_projects
            .find(id)
            .first(connection)
    }

    pub fn read_by_project(
        connection: &mut SqliteConnection,
        name: &str,
        company_id: i32,
    ) -> Result<Self> {
        company_gcp_projects::table
            .filter(company_gcp_projects::dsl::company_id.eq(company_id))
            .filter(company_gcp_projects::dsl::gcp_project.eq(name))
            .first(connection)
    }

    pub fn update(&self, connection: &mut SqliteConnection) -> Result<Self> {
        self.save_changes(connection)
    }

    pub fn delete(&self, connection: &mut SqliteConnection) -> Result<usize> {
        use company_gcp_projects::dsl;

        diesel::delete(dsl::company_gcp_projects.filter(dsl::id.eq(&self.id))).execute(connection)
    }
}
