//! The `GET /user` endpoint.
//!
//! Gets the user the request's token acts as.

use http::{Method, StatusCode};
use sub60_endpoints_common::{Json, JsonError, Ok, PathParams};

use crate::{AccountSummary, UserId};

#[derive(Debug, Copy, Clone)]
#[expect(missing_docs)]
pub struct Endpoint;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PathParams)]
#[route("/user")]
#[expect(missing_docs)]
pub struct Path;

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
)]
#[expect(missing_docs)]
pub struct User {
    #[expect(missing_docs)]
    pub id: UserId,
    /// The user's personal account, or `None` if it hasn't been created yet.
    pub account: Option<AccountSummary>,
}

#[derive(
    Debug, derive_more::Display, serde::Serialize, schemars::JsonSchema,
)]
#[serde(tag = "code", content = "details", rename_all = "snake_case")]
#[schemars(rename = "GetUserError")]
#[expect(missing_docs)]
pub enum Error {
    /// The request has no user token, or its token is invalid or expired.
    #[display("the request needs a valid user token")]
    Unauthorized,

    /// The server failed to get the user.
    #[display("the server failed to get the user")]
    Internal,
}

impl sub60_endpoints_common::Endpoint for Endpoint {
    const METHOD: Method = Method::GET;

    type Path = Path;
    type Request = ();
    type Response = Ok<Json<User>>;
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
