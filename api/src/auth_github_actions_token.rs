//! The `POST /auth/github/actions/token` endpoint.
//!
//! Exchanges the OIDC token of a GitHub Actions job for a token.

use http::{Method, StatusCode};
use sub60_endpoints_common::{Json, JsonError, Ok, PathParams, Timestamp};

#[derive(Debug, Copy, Clone)]
#[expect(missing_docs)]
pub struct Endpoint;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PathParams)]
#[route("/auth/github/actions/token")]
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
#[schemars(rename = "GitHubActionsAuthRequest")]
#[expect(missing_docs)]
pub struct Request {
    /// The OIDC token the job got from GitHub, requested with the server's
    /// origin as its audience.
    pub token: String,
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
#[schemars(rename = "GitHubActionsAuthResponse")]
#[expect(missing_docs)]
pub struct Response {
    /// The token to authenticate requests with, as a bearer token in their
    /// `Authorization` header.
    pub token: String,

    /// The time the token stops being valid.
    pub expires_at: Timestamp,
}

#[derive(
    Debug, derive_more::Display, serde::Serialize, schemars::JsonSchema,
)]
#[serde(tag = "code", content = "details", rename_all = "snake_case")]
#[schemars(rename = "GitHubActionsAuthError")]
#[expect(missing_docs)]
pub enum Error {
    /// The OIDC token isn't a valid one for this server, e.g. because it has
    /// expired, it wasn't signed by GitHub Actions, or its audience isn't the
    /// server's origin.
    #[display("the token is invalid: {reason}")]
    InvalidToken {
        /// Why the token is invalid.
        reason: String,
    },

    /// The job can't access any cache.
    #[display("the job can't access any cache")]
    NoAccess,

    /// The server failed to verify the token or to store the new one.
    #[display("the server failed to exchange the token")]
    Internal,
}

impl sub60_endpoints_common::Endpoint for Endpoint {
    const METHOD: Method = Method::POST;

    type Path = Path;
    type Request = Json<Request>;
    type Response = Ok<Json<Response>>;
    type Error = JsonError<Error>;
}

impl From<Error> for StatusCode {
    fn from(error: Error) -> Self {
        match error {
            Error::InvalidToken { .. } => Self::UNAUTHORIZED,
            Error::NoAccess => Self::FORBIDDEN,
            Error::Internal => Self::INTERNAL_SERVER_ERROR,
        }
    }
}
