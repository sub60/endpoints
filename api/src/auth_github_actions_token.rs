//! The `POST /v1/auth/github/actions/token` endpoint.

use core::time::Duration;

use http::{Method, StatusCode};

use crate::{EndpointError, JsonEndpoint, PathParams, V1Path};

/// Exchanges the OIDC token of a GitHub Actions job for a token.
#[derive(Debug, Copy, Clone)]
pub struct Endpoint;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PathParams)]
#[route(V1Path + "/auth/github/actions/token")]
#[expect(missing_docs)]
pub struct Path {
    pub v1: V1Path,
}

/// The body of the endpoint's requests.
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
pub struct Request {
    /// The OIDC token the job got from GitHub, requested with the server's
    /// origin as its audience.
    pub token: String,
}

/// The body of the endpoint's successful responses.
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
pub struct Response {
    /// The token to authenticate requests with, as a bearer token in their
    /// `Authorization` header.
    pub token: String,

    /// When the token stops being valid, as the number of seconds since the
    /// UNIX epoch.
    #[serde(with = "sub60_endpoints_common::serde_utils::secs")]
    #[schemars(with = "u64")]
    pub expires_at: Duration,
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

impl crate::Endpoint for Endpoint {
    const METHOD: Method = Method::POST;

    type Path = Path;
    type Request = Request;
    type Response = Response;
}

impl JsonEndpoint for Endpoint {
    type Error = Error;
}

impl EndpointError for Error {}

impl From<Error> for StatusCode {
    fn from(error: Error) -> Self {
        match error {
            Error::InvalidToken { .. } => Self::UNAUTHORIZED,
            Error::NoAccess => Self::FORBIDDEN,
            Error::Internal => Self::INTERNAL_SERVER_ERROR,
        }
    }
}

