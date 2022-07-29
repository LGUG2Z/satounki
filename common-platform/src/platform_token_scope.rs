use diesel_derive_enum::DbEnum;
use serde::Deserialize;
use serde::Serialize;
use strum::EnumString;

/// Scopes of a Platform token
#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize, DbEnum, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum PlatformTokenScope {
    /// View Platform resources
    Read,
    /// Edit Platform resources
    Write,
}
