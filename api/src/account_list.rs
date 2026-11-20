//! The `GET /accounts` endpoint.
//!
//! Lists the accounts the request's principal has access to, or looks one up by
//! name.

use http::{Method, StatusCode};
use sub60_endpoints_common::{
    AccountName,
    Json,
    JsonError,
    Ok,
    PathParams,
    WithQuery,
};

use crate::Account;

#[derive(Debug, Copy, Clone)]
#[expect(missing_docs)]
pub struct Endpoint;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PathParams)]
#[route("/accounts")]
#[expect(missing_docs)]
pub struct Path;

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
#[expect(missing_docs)]
pub struct Query {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<AccountName>,
}

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
)]
#[schemars(rename = "AccountList")]
#[expect(missing_docs)]
pub struct Response {
    pub accounts: Vec<Account>,
}

#[derive(
    Debug, derive_more::Display, serde::Serialize, schemars::JsonSchema,
)]
#[serde(tag = "code", content = "details", rename_all = "snake_case")]
#[schemars(rename = "ListAccountsError")]
#[expect(missing_docs)]
pub enum Error {
    /// The request has no token, or its token is invalid or expired.
    #[display("the request needs a valid token")]
    Unauthorized,

    /// The server failed to list the accounts.
    #[display("the server failed to list the accounts")]
    Internal,
}

impl sub60_endpoints_common::Endpoint for Endpoint {
    const METHOD: Method = Method::GET;

    type Path = Path;
    type Request = WithQuery<Query>;
    type Response = Ok<Json<Response>>;
    type Error = JsonError<Error>;
}

impl From<Error> for StatusCode {
    fn from(error: Error) -> Self {
        match error {
            Error::Unauthorized => Self::UNAUTHORIZED,
            Error::Internal => Self::INTERNAL_SERVER_ERROR,
        }
    }
}
