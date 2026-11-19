//! The `DELETE /user` endpoint.
//!
//! Deletes the user the request's token acts as, along with their personal
//! account and its caches.

use http::{Method, StatusCode};
use sub60_endpoints_common::{JsonError, NoContent};

use crate::{AccountSummary, user_get};

#[expect(missing_docs)]
pub type Path = user_get::Path;

#[derive(Debug, Copy, Clone)]
#[expect(missing_docs)]
pub struct Endpoint;

#[derive(
    Debug, derive_more::Display, serde::Serialize, schemars::JsonSchema,
)]
#[serde(tag = "code", content = "details", rename_all = "snake_case")]
#[schemars(rename = "DeleteUserError")]
#[expect(missing_docs)]
pub enum Error {
    /// The request has no user token, or its token is invalid/expired.
    #[display("the request needs a valid user token")]
    Unauthorized,

    /// The user is the only owner of some org accounts, which would be left
    /// without one.
    #[display("the user is the only owner of {} org accounts", accounts.len())]
    LastOwner {
        /// The org accounts the user is the only owner of.
        accounts: Vec<AccountSummary>,
    },

    /// The server failed to delete the user.
    #[display("the server failed to delete the user")]
    Internal,
}

impl sub60_endpoints_common::Endpoint for Endpoint {
    const METHOD: Method = Method::DELETE;

    type Error = JsonError<Error>;
    type Path = Path;
    type Request = ();
    type Response = NoContent;
}

impl From<Error> for StatusCode {
    fn from(error: Error) -> Self {
        match error {
            Error::Unauthorized => Self::UNAUTHORIZED,
            Error::LastOwner { .. } => Self::CONFLICT,
            Error::Internal => Self::INTERNAL_SERVER_ERROR,
        }
    }
}
