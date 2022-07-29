#![warn(clippy::all, clippy::nursery, clippy::pedantic)]
#![allow(clippy::missing_errors_doc, clippy::use_self)]

pub use error::Error;
pub use gcloud::Gcloud;
pub use gcloud::Inputs as GcloudInputData;
pub use iam_policy::IamPolicy;
pub use iam_policy::Wrapper as IamPolicyWrapper;

mod error;
mod gcloud;
mod iam_policy;
