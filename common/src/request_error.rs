use http::StatusCode;

use crate::response::VariantVisitor;
use crate::{Response, ResponseVisitor, error};

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
    InvalidRequest(error::InvalidRequest),
    NotFound(error::NotFound),
    MethodNotAllowed(error::MethodNotAllowed),
    Internal(error::Internal),
}

impl From<StatusCode> for RequestError {
    fn from(status: StatusCode) -> Self {
        match status {
            StatusCode::NOT_FOUND => Self::NotFound(error::NotFound),
            StatusCode::METHOD_NOT_ALLOWED => {
                Self::MethodNotAllowed(error::MethodNotAllowed)
            },
            status if status.is_server_error() => {
                Self::Internal(error::Internal)
            },
            _ => Self::InvalidRequest(error::InvalidRequest),
        }
    }
}

impl Response for RequestError {
    #[inline]
    fn status(&self) -> StatusCode {
        match self {
            Self::InvalidRequest(error) => error.status(),
            Self::NotFound(error) => error.status(),
            Self::MethodNotAllowed(error) => error.status(),
            Self::Internal(error) => error.status(),
        }
    }

    #[inline]
    fn visit<V: ResponseVisitor>(visitor: &mut V) {
        error::InvalidRequest::visit(visitor);
        error::NotFound::visit(visitor);
        error::MethodNotAllowed::visit(visitor);
        error::Internal::visit(visitor);
    }

    #[inline]
    fn visit_variants<V: VariantVisitor>(visitor: &mut V) {
        visitor.visit::<error::InvalidRequest, 0>();
        visitor.visit::<error::NotFound, 1>();
        visitor.visit::<error::MethodNotAllowed, 2>();
        visitor.visit::<error::Internal, 3>();
    }
}
