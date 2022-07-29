use serde::Deserialize;
use serde::Serialize;

/// Action to be taken with a set of permissions
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum PermissionsAction {
    /// Grant permissions to the requesting user
    Add,
    /// Remove permissions from the requesting user
    Remove,
}
