//! The errors endpoints respond with.

use http::StatusCode;

use crate::response::Status;
use crate::{PathParamsError, Permission, Statuses, request};

/// The request has no token, or its token is invalid, expired or of a kind the
/// endpoint doesn't take.
#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    Hash,
    derive_more::Display,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
)]
#[display("the request needs a valid token")]
pub struct Unauthorized;

/// The request's principal is missing permissions the request needs.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    derive_more::Display,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
)]
#[display("the request's principal is missing permissions")]
pub struct Forbidden {
    /// The permissions the principal is missing.
    pub missing: Vec<Permission>,
}

/// Nothing exists at the request's path, or the request's principal doesn't
/// have access to it.
#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    Hash,
    derive_more::Display,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
)]
#[display("there's nothing at the request's path")]
pub struct NotFound;

/// The request's path, query or body didn't parse.
#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    Hash,
    derive_more::Display,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
)]
#[display("the request is invalid")]
pub struct InvalidRequest;

/// The request's cursor isn't a `next_cursor` the list returned.
#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    Hash,
    derive_more::Display,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
)]
#[display("the request's cursor is invalid")]
pub struct InvalidCursor;

/// The request's body is longer than the endpoint can accept.
#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    Hash,
    derive_more::Display,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
)]
#[display("the request's body is too long")]
pub struct PayloadTooLarge;

/// The request's `Content-Type` isn't the media type of the endpoint's body.
#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    Hash,
    derive_more::Display,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
)]
#[display("the request's Content-Type isn't the one the endpoint takes")]
pub struct UnsupportedMediaType;

/// The endpoint at the request's path doesn't take the request's method.
#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    Hash,
    derive_more::Display,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
)]
#[display("the endpoint at the request's path doesn't take its method")]
pub struct MethodNotAllowed;

/// The server failed to handle the request.
#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    Hash,
    derive_more::Display,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
)]
#[display("the server failed to handle the request")]
pub struct Internal;

/// The ways a request can fail before it reaches an endpoint, e.g. when its
/// path matches no endpoint.
#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    Hash,
    derive_more::Display,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    Statuses,
)]
#[serde(tag = "code", rename_all = "snake_case")]
pub enum RequestError {
    InvalidRequest(InvalidRequest),
    PayloadTooLarge(PayloadTooLarge),
    UnsupportedMediaType(UnsupportedMediaType),
    NotFound(NotFound),
    MethodNotAllowed(MethodNotAllowed),
    Internal(Internal),
}

impl Status for Unauthorized {
    const STATUS: StatusCode = StatusCode::UNAUTHORIZED;
}

impl Status for Forbidden {
    const STATUS: StatusCode = StatusCode::FORBIDDEN;
}

impl Status for NotFound {
    const STATUS: StatusCode = StatusCode::NOT_FOUND;
}

impl Status for InvalidRequest {
    const STATUS: StatusCode = StatusCode::BAD_REQUEST;
}

impl Status for InvalidCursor {
    const STATUS: StatusCode = StatusCode::BAD_REQUEST;
}

impl Status for PayloadTooLarge {
    const STATUS: StatusCode = StatusCode::PAYLOAD_TOO_LARGE;
}

impl Status for UnsupportedMediaType {
    const STATUS: StatusCode = StatusCode::UNSUPPORTED_MEDIA_TYPE;
}

impl Status for MethodNotAllowed {
    const STATUS: StatusCode = StatusCode::METHOD_NOT_ALLOWED;
}

impl Status for Internal {
    const STATUS: StatusCode = StatusCode::INTERNAL_SERVER_ERROR;
}

impl<E> From<request::DecodeError<E>> for RequestError {
    #[inline]
    fn from(error: request::DecodeError<E>) -> Self {
        match error {
            request::DecodeError::MediaType { .. } => {
                Self::UnsupportedMediaType(UnsupportedMediaType)
            },
            request::DecodeError::Query(_) | request::DecodeError::Body(_) => {
                Self::InvalidRequest(InvalidRequest)
            },
        }
    }
}

impl From<PathParamsError> for RequestError {
    #[inline]
    fn from(error: PathParamsError) -> Self {
        match error {
            PathParamsError::Invalid { .. } => {
                Self::InvalidRequest(InvalidRequest)
            },
            // A route whose template and parameters disagree is a bug on the
            // server.
            PathParamsError::Missing(_)
            | PathParamsError::UnexpectedName { .. } => {
                Self::Internal(Internal)
            },
        }
    }
}
