//! The `POST /auth/github/actions/token` endpoint.
//!
//! Exchanges the OIDC token of a GitHub Actions job for a token.

use http::{Method, StatusCode};
use sub60_endpoints_common::body::{Json, WithMessage};
use sub60_endpoints_common::response::{Ok, Status, WithStatusOf};
use sub60_endpoints_common::{PathParams, Timestamp, error, status};

#[derive(Debug, Copy, Clone)]
pub struct Endpoint;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PathParams)]
#[route("/auth/github/actions/token")]
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
pub struct Response {
    /// The token to authenticate requests with, as a bearer token in their
    /// `Authorization` header.
    pub token: String,

    /// The time the token stops being valid.
    pub expires_at: Timestamp,
}

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
    /// The OIDC token isn't a valid one for this server, e.g. because it has
    /// expired, it wasn't signed by GitHub Actions, or its audience isn't the
    /// server's origin.
    #[display("the token is invalid: {}", _0.reason)]
    InvalidToken(TokenRejection),
    /// The job can't access any cache.
    #[display("the job can't access any cache")]
    NoAccess(status::Forbidden),
    Internal(error::Internal),
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
pub struct TokenRejection {
    pub reason: String,
}

impl sub60_endpoints_common::Endpoint for Endpoint {
    const METHOD: Method = Method::POST;

    type Path = Path;
    type Request = Json<Request>;
    type Response = Ok<Json<Response>>;
    type Error = WithStatusOf<Json<WithMessage<Error>>>;
}

impl Status for TokenRejection {
    const STATUS: StatusCode = StatusCode::UNAUTHORIZED;
}
