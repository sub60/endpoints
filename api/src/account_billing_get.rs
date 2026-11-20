//! The `GET /accounts/{account_id}/billing` endpoint.
//!
//! Gets an account's [`Billing`].

use http::{Method, StatusCode};
use sub60_endpoints_common::{Json, JsonError, Ok, PathParams, Permission};

use crate::{AccountId, Billing};

#[derive(Debug, Copy, Clone)]
#[expect(missing_docs)]
pub struct Endpoint;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PathParams)]
#[route("/accounts/{account_id}/billing")]
#[expect(missing_docs)]
pub struct Path {
    #[expect(missing_docs)]
    pub account_id: AccountId,
}

#[derive(
    Debug, derive_more::Display, serde::Serialize, schemars::JsonSchema,
)]
#[serde(tag = "code", content = "details", rename_all = "snake_case")]
#[schemars(rename = "GetBillingError")]
#[expect(missing_docs)]
pub enum Error {
    /// The request has no token, or its token is invalid or expired.
    #[display("the request needs a valid token")]
    Unauthorized,

    /// The principal has access to the account, but can't see its billing.
    #[display("the request's principal can't see the account's billing")]
    Forbidden {
        /// The permissions the principal is missing on the account.
        missing: Vec<Permission>,
    },

    /// No account has the ID, or the principal doesn't have access to it.
    #[display("there's no account with the ID")]
    NotFound,

    /// The server failed to get the account's billing.
    #[display("the server failed to get the account's billing")]
    Internal,
}

impl sub60_endpoints_common::Endpoint for Endpoint {
    const METHOD: Method = Method::GET;

    type Path = Path;
    type Request = ();
    type Response = Ok<Json<Billing>>;
    type Error = JsonError<Error>;
}

impl From<Error> for StatusCode {
    fn from(error: Error) -> Self {
        match error {
            Error::Unauthorized => Self::UNAUTHORIZED,
            Error::Forbidden { .. } => Self::FORBIDDEN,
            Error::NotFound => Self::NOT_FOUND,
            Error::Internal => Self::INTERNAL_SERVER_ERROR,
        }
    }
}
