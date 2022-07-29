#![warn(clippy::all, clippy::nursery, clippy::pedantic)]
#![allow(clippy::missing_errors_doc, clippy::use_self)]

pub use cloudflare::Cloudflare;
use member::ListMembersResponse;
use member::Member as UpdateMemberRequest;
use member::Member;
use member::UpdateMemberResponse;
use serde::de::DeserializeOwned;
use serde::Serialize;

mod cloudflare;
mod member;
mod tokens;

type Result<T> = core::result::Result<T, reqwest::Error>;

trait CloudflareRequest: Serialize + Sync + Send {}
trait CloudflareResponse: DeserializeOwned + Sync + Send {}
