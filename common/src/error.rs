//! The errors endpoints respond with.

use http::StatusCode;

use crate::Permission;
use crate::response::{Response, ResponseVisitor, VariantVisitor};

/// An error an endpoint responds with.
pub trait Error {
    /// The status code of the error's responses.
    const STATUS: StatusCode;
}

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
)]
#[serde(tag = "code", rename_all = "snake_case")]
pub enum RequestError {
    InvalidRequest(InvalidRequest),
    PayloadTooLarge(PayloadTooLarge),
    NotFound(NotFound),
    MethodNotAllowed(MethodNotAllowed),
    Internal(Internal),
}

/// A response with the error's status code and no body.
impl<E: Error> Response for E {
    #[inline]
    fn status(&self) -> StatusCode {
        E::STATUS
    }

    #[inline]
    fn visit<V: ResponseVisitor>(visitor: &mut V) {
        visitor.visit::<()>(E::STATUS, None);
    }
}

impl Error for Unauthorized {
    const STATUS: StatusCode = StatusCode::UNAUTHORIZED;
}

impl Error for Forbidden {
    const STATUS: StatusCode = StatusCode::FORBIDDEN;
}

impl Error for NotFound {
    const STATUS: StatusCode = StatusCode::NOT_FOUND;
}

impl Error for InvalidRequest {
    const STATUS: StatusCode = StatusCode::BAD_REQUEST;
}

impl Error for InvalidCursor {
    const STATUS: StatusCode = StatusCode::BAD_REQUEST;
}

impl Error for PayloadTooLarge {
    const STATUS: StatusCode = StatusCode::PAYLOAD_TOO_LARGE;
}

impl Error for MethodNotAllowed {
    const STATUS: StatusCode = StatusCode::METHOD_NOT_ALLOWED;
}

impl Error for Internal {
    const STATUS: StatusCode = StatusCode::INTERNAL_SERVER_ERROR;
}

impl From<StatusCode> for RequestError {
    fn from(status: StatusCode) -> Self {
        match status {
            StatusCode::PAYLOAD_TOO_LARGE => {
                Self::PayloadTooLarge(PayloadTooLarge)
            },
            StatusCode::NOT_FOUND => Self::NotFound(NotFound),
            StatusCode::METHOD_NOT_ALLOWED => {
                Self::MethodNotAllowed(MethodNotAllowed)
            },
            status if status.is_server_error() => Self::Internal(Internal),
            _ => Self::InvalidRequest(InvalidRequest),
        }
    }
}

impl Response for RequestError {
    #[inline]
    fn status(&self) -> StatusCode {
        match self {
            Self::InvalidRequest(error) => error.status(),
            Self::PayloadTooLarge(error) => error.status(),
            Self::NotFound(error) => error.status(),
            Self::MethodNotAllowed(error) => error.status(),
            Self::Internal(error) => error.status(),
        }
    }

    #[inline]
    fn visit<V: ResponseVisitor>(visitor: &mut V) {
        InvalidRequest::visit(visitor);
        PayloadTooLarge::visit(visitor);
        NotFound::visit(visitor);
        MethodNotAllowed::visit(visitor);
        Internal::visit(visitor);
    }

    #[inline]
    fn visit_variants<V: VariantVisitor>(visitor: &mut V) {
        visitor.visit::<InvalidRequest, 0>();
        visitor.visit::<PayloadTooLarge, 1>();
        visitor.visit::<NotFound, 2>();
        visitor.visit::<MethodNotAllowed, 3>();
        visitor.visit::<Internal, 4>();
    }
}
