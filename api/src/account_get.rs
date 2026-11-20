//! The `GET /accounts/{account_id}` endpoint.
//!
//! Gets an [`Account`].

use http::{Method, StatusCode};
use sub60_endpoints_common::{Json, JsonError, Ok, PathParams};

use crate::{Account, AccountId};

#[derive(Debug, Copy, Clone)]
#[expect(missing_docs)]
pub struct Endpoint;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PathParams)]
#[route("/accounts/{account_id}")]
#[expect(missing_docs)]
pub struct Path {
    #[expect(missing_docs)]
    pub account_id: AccountId,
}

#[derive(
    Debug, derive_more::Display, serde::Serialize, schemars::JsonSchema,
)]
#[serde(tag = "code", content = "details", rename_all = "snake_case")]
#[schemars(rename = "GetAccountError")]
#[expect(missing_docs)]
pub enum Error {
    /// The request has no token, or its token is invalid or expired.
    #[display("the request needs a valid token")]
    Unauthorized,

    /// No account has the ID, or the principal can't see it.
    #[display("there's no account with the ID")]
    NotFound,

    /// The server failed to get the account.
    #[display("the server failed to get the account")]
    Internal,
}

impl sub60_endpoints_common::Endpoint for Endpoint {
    const METHOD: Method = Method::GET;

    type Path = Path;
    type Request = ();
    type Response = Ok<Json<Account>>;
    type Error = JsonError<Error>;
}

impl From<Error> for StatusCode {
    fn from(error: Error) -> Self {
        match error {
            Error::Unauthorized => Self::UNAUTHORIZED,
            Error::NotFound => Self::NOT_FOUND,
            Error::Internal => Self::INTERNAL_SERVER_ERROR,
        }
    }
}
