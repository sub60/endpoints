//! Compatibility with axum.

use core::fmt;

use ::axum::extract::rejection::RawPathParamsRejection;
use ::axum::extract::{FromRequestParts, RawPathParams};
use ::axum::response::{IntoResponse, Response};
use http::request::Parts;
use http::{HeaderValue, StatusCode, header};

use crate::body::Encode;
use crate::response::WithStatus;
use crate::{PathParams, PathParamsError, request};

/// The rejection returned when [`PathParams`] can't be extracted from a
/// request.
#[derive(Debug, derive_more::Display, cauchy::Error)]
pub enum PathParamsRejection {
    /// The path's parameters couldn't be read.
    #[display("{_0}")]
    Raw(RawPathParamsRejection),

    /// The path's parameters didn't parse.
    #[display("{_0}")]
    Parse(PathParamsError),
}

/// Extracts the path parameters of a request, for the `FromRequestParts` impls
/// the derive macro expands to.
#[doc(hidden)]
pub async fn from_request_parts<T: PathParams, S: Send + Sync>(
    parts: &mut Parts,
    state: &S,
) -> Result<T, PathParamsRejection> {
    let params = RawPathParams::from_request_parts(parts, state)
        .await
        .map_err(PathParamsRejection::Raw)?;

    T::parse(&mut params.iter()).map_err(PathParamsRejection::Parse)
}

impl<E: fmt::Display> IntoResponse for request::DecodeError<E> {
    #[inline]
    fn into_response(self) -> Response {
        let status = match &self {
            Self::MediaType { .. } => StatusCode::UNSUPPORTED_MEDIA_TYPE,
            Self::Query(_) | Self::Body(_) => StatusCode::BAD_REQUEST,
        };

        (status, self.to_string()).into_response()
    }
}

impl IntoResponse for PathParamsError {
    fn into_response(self) -> Response {
        let status = match self {
            Self::Invalid { .. } => StatusCode::BAD_REQUEST,
            // The route's template and the parameters disagree, which is a
            // bug on the server.
            Self::Missing(_) | Self::UnexpectedName { .. } => {
                StatusCode::INTERNAL_SERVER_ERROR
            },
        };

        (status, self.to_string()).into_response()
    }
}

impl IntoResponse for PathParamsRejection {
    fn into_response(self) -> Response {
        match self {
            Self::Raw(rejection) => rejection.into_response(),
            Self::Parse(err) => err.into_response(),
        }
    }
}

impl<const S: u16, B: Encode> IntoResponse for WithStatus<S, B> {
    #[inline]
    fn into_response(self) -> Response {
        let mut body = bytes::BytesMut::new();
        self.0.encode(&mut body);

        let mut response =
            Response::new(::axum::body::Body::from(body.freeze()));

        *response.status_mut() = Self::STATUS;

        if let Some(media_type) = B::MEDIA_TYPE {
            response.headers_mut().insert(
                header::CONTENT_TYPE,
                HeaderValue::from_static(media_type),
            );
        }

        response
    }
}
