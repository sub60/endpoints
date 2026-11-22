#[cfg(feature = "axum")]
use axum::extract::{FromRequest, FromRequestParts, Query};
#[cfg(feature = "axum")]
use axum::response::IntoResponse;

use crate::Body;

/// TODO: docs.
pub trait Request {
    /// TODO: docs.
    type Query: serde::Serialize
        + serde::de::DeserializeOwned
        + schemars::JsonSchema;

    /// TODO: docs.
    type Body: Body;
}

/// A request with query parameters `Q` and body `B`.
#[derive(Debug, Copy, Clone, Default, PartialEq, Eq, Hash)]
#[doc(hidden)]
pub struct WithQuery<Q, B = ()> {
    pub query: Q,
    pub body: B,
}

impl<B: Body> Request for B {
    type Query = ();
    type Body = Self;
}

impl<Q, B> Request for WithQuery<Q, B>
where
    Q: serde::Serialize + serde::de::DeserializeOwned + schemars::JsonSchema,
    B: Body,
{
    type Query = Q;
    type Body = B;
}

#[cfg(feature = "axum")]
impl<S, Q> FromRequestParts<S> for WithQuery<Q>
where
    S: Send + Sync,
    Q: serde::de::DeserializeOwned,
{
    type Rejection = axum::extract::rejection::QueryRejection;

    #[inline]
    async fn from_request_parts(
        parts: &mut http::request::Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        let Query(query) = Query::from_request_parts(parts, state).await?;
        Ok(Self { query, body: () })
    }
}

#[cfg(feature = "axum")]
impl<S, Q, T> FromRequest<S> for WithQuery<Q, crate::Json<T>>
where
    S: Send + Sync,
    Q: serde::de::DeserializeOwned + Send,
    T: serde::de::DeserializeOwned,
{
    type Rejection = axum::response::Response;

    #[inline]
    async fn from_request(
        request: axum::extract::Request,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        let (mut parts, body) = request.into_parts();

        let Query(query) = Query::from_request_parts(&mut parts, state)
            .await
            .map_err(IntoResponse::into_response)?;

        let request = axum::extract::Request::from_parts(parts, body);

        let body = crate::Json::from_request(request, state)
            .await
            .map_err(IntoResponse::into_response)?;

        Ok(Self { query, body })
    }
}
