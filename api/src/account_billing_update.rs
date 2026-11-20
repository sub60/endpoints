//! The `PATCH /accounts/{account_id}/billing` endpoint.
//!
//! Changes an account's billing details, and returns the updated [`Billing`].

use http::{Method, StatusCode};
use sub60_endpoints_common::{Json, JsonError, Ok, Permission};

use crate::{Billing, account_billing_get};

#[expect(missing_docs)]
pub type Path = account_billing_get::Path;

#[derive(Debug, Copy, Clone)]
#[expect(missing_docs)]
pub struct Endpoint;

#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
)]
#[schemars(rename = "UpdateBillingRequest")]
#[expect(missing_docs)]
pub struct Request {
    /// The new email address billing notices are sent to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub billing_email: Option<String>,
}

#[derive(
    Debug, derive_more::Display, serde::Serialize, schemars::JsonSchema,
)]
#[serde(tag = "code", content = "details", rename_all = "snake_case")]
#[schemars(rename = "UpdateBillingError")]
#[expect(missing_docs)]
pub enum Error {
    /// The request has no token, or its token is invalid or expired.
    #[display("the request needs a valid token")]
    Unauthorized,

    /// The principal has access to the account, but can't change its billing.
    #[display("the request's principal can't change the account's billing")]
    Forbidden {
        /// The permissions the principal is missing on the account.
        missing: Vec<Permission>,
    },

    /// No account has the ID, or the principal doesn't have access to it.
    #[display("there's no account with the ID")]
    NotFound,

    /// The new billing email isn't a valid email address.
    #[display("the billing email isn't a valid email address")]
    InvalidEmail,

    /// The server failed to change the account's billing.
    #[display("the server failed to change the account's billing")]
    Internal,
}

impl sub60_endpoints_common::Endpoint for Endpoint {
    const METHOD: Method = Method::PATCH;

    type Path = Path;
    type Request = Json<Request>;
    type Response = Ok<Json<Billing>>;
    type Error = JsonError<Error>;
}

impl From<Error> for StatusCode {
    fn from(error: Error) -> Self {
        match error {
            Error::Unauthorized => Self::UNAUTHORIZED,
            Error::Forbidden { .. } => Self::FORBIDDEN,
            Error::NotFound => Self::NOT_FOUND,
            Error::InvalidEmail => Self::BAD_REQUEST,
            Error::Internal => Self::INTERNAL_SERVER_ERROR,
        }
    }
}
