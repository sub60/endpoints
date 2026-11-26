//! Compatibility with axum.

use ::axum::extract::rejection::RawPathParamsRejection;
use ::axum::extract::{FromRequestParts, RawPathParams};
use ::axum::response::{IntoResponse, Response};
use http::request::Parts;
use http::{HeaderValue, StatusCode, header};

use crate::body::{Json, WithMessage};
use crate::error::{self, RequestError};
use crate::response::WithStatusOf;
use crate::{PathParams, PathParamsError, body, request, response};

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

/// A [`response::Encoder`] into an axum response.
struct IntoAxum;

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

/// Returns the axum response holding `response`.
pub fn into_response(response: &impl response::Encode) -> Response {
    response.encode(IntoAxum)
}

impl IntoResponse for RequestError {
    #[inline]
    fn into_response(self) -> Response {
        into_response(&WithStatusOf(Json(WithMessage(self))))
    }
}

impl<E> IntoResponse for request::DecodeError<E> {
    #[inline]
    fn into_response(self) -> Response {
        RequestError::from(self).into_response()
    }
}

impl IntoResponse for PathParamsError {
    #[inline]
    fn into_response(self) -> Response {
        RequestError::from(self).into_response()
    }
}

impl IntoResponse for PathParamsRejection {
    fn into_response(self) -> Response {
        match self {
            Self::Raw(rejection) if rejection.status().is_server_error() => {
                RequestError::Internal(error::Internal).into_response()
            },
            Self::Raw(_) => RequestError::InvalidRequest(error::InvalidRequest)
                .into_response(),
            Self::Parse(err) => err.into_response(),
        }
    }
}

impl response::Encoder for IntoAxum {
    type Output = Response;

    fn encode<B: body::Encode>(self, status: StatusCode, body: &B) -> Response {
        let mut bytes = bytes::BytesMut::new();
        body.encode(&mut bytes);

        let mut response =
            Response::new(::axum::body::Body::from(bytes.freeze()));

        *response.status_mut() = status;

        if let Some(media_type) = B::MEDIA_TYPE {
            response.headers_mut().insert(
                header::CONTENT_TYPE,
                HeaderValue::from_static(media_type),
            );
        }

        response
    }
}
