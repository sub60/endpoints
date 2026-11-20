//! The `DELETE /user` endpoint.
//!
//! Deletes the user the request's token acts as, along with their personal
//! account and its caches.

use http::Method;
use sub60_endpoints_common::{Json, NoContent, WithMessage, error};

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
    sub60_endpoints_common::Response,
)]
#[serde(tag = "code", rename_all = "snake_case")]
pub enum Error {
    Unauthorized(error::Unauthorized),
    /// The user is the only owner of some org accounts, which would be left
    /// without one.
    #[display("the user is the only owner of {} org accounts", accounts.len())]
    #[status(CONFLICT)]
    LastOwner {
        /// The org accounts the user is the only owner of.
        accounts: Vec<AccountSummary>,
    },
    Internal(error::Internal),
}

impl sub60_endpoints_common::Endpoint for Endpoint {
    const METHOD: Method = Method::DELETE;

    type Path = Path;
    type Request = ();
    type Response = NoContent;
    type Error = Json<WithMessage<Error>>;
}
