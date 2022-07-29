use crate::Schema;

/// Error returned by the Satounki API
#[apply(Schema!)]
pub struct ErrorResponse {
    /// HTTP error code
    pub code: u16,
    /// User-friendly error message
    pub error: String,
}
