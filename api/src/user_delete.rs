//! The `DELETE /user` endpoint.
//!
//! Deletes the user the request's token acts as, along with their personal
//! account and its caches.

use http::{Method, StatusCode};
use sub60_endpoints_common::body::{Json, WithMessage};
use sub60_endpoints_common::error;
use sub60_endpoints_common::response::{NoContent, Status, WithStatusOf};

use crate::{AccountSummary, user_get};

pub type Path = user_get::Path;

#[derive(Debug, Copy, Clone)]
pub struct Endpoint;

#[derive(
    Debug,
    derive_more::Display,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    sub60_endpoints_common::Statuses,
)]
#[serde(tag = "code", rename_all = "snake_case")]
pub enum Error {
    Unauthorized(error::Unauthorized),
    /// The user is the only owner of some org accounts. Deleting the user would
    /// leave those accounts without an owner.
    #[display("the user is the only owner of {} org accounts", _0.accounts.len())]
    LastOwner(OwnedAccounts),
    Internal(error::Internal),
}

/// The org accounts a user is the only owner of.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
)]
pub struct OwnedAccounts {
    pub accounts: Vec<AccountSummary>,
}

impl sub60_endpoints_common::Endpoint for Endpoint {
    const METHOD: Method = Method::DELETE;

    type Path = Path;
    type Request = ();
    type Response = NoContent;
    type Error = WithStatusOf<Json<WithMessage<Error>>>;
}

impl Status for OwnedAccounts {
    const STATUS: StatusCode = StatusCode::CONFLICT;
}
