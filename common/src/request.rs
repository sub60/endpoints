//! TODO: docs.

use crate::body::{Body, Decode};

/// TODO: docs.
pub trait Request {
    /// TODO: docs.
    type Query: schemars::JsonSchema;

    /// TODO: docs.
    type Body: Body;

    /// TODO: docs.
    fn from_parts(query: Self::Query, body: Self::Body) -> Self;
}

/// TODO: docs.
#[derive(Debug, derive_more::Display, cauchy::Error)]
pub enum DecodeError<E> {
    /// The request's `Content-Type` isn't the body's media type.
    #[display("the request's Content-Type must be {expected}")]
    MediaType {
        /// The body's media type.
        expected: &'static str,
    },

    /// The request's query string doesn't parse into the query parameters.
    #[display("the request's query string is invalid: {_0}")]
    Query(serde_urlencoded::de::Error),

    /// The request's body bytes don't decode.
    #[display("the request's body is invalid: {_0}")]
    Body(E),
}

/// A request with query parameters `Q` and body `B`.
#[derive(Debug, Copy, Clone, Default, PartialEq, Eq, Hash)]
#[doc(hidden)]
pub struct WithQuery<Q, B = ()> {
    pub query: Q,
    pub body: B,
}

/// TODO: docs.
pub fn decode<R>(
    parts: &http::request::Parts,
    body: &[u8],
) -> Result<R, DecodeError<<R::Body as Decode>::Error>>
where
    R: Request<Query: serde::de::DeserializeOwned, Body: Decode>,
{
    fn essence(media_type: &str) -> &str {
        media_type.split(';').next().unwrap_or_default().trim()
    }

    if let Some(expected) = R::Body::MEDIA_TYPE
        && !parts
            .headers
            .get(http::header::CONTENT_TYPE)
            .and_then(|value| value.to_str().ok())
            .is_some_and(|value| {
                essence(value).eq_ignore_ascii_case(essence(expected))
            })
    {
        return Err(DecodeError::MediaType { expected });
    }

    let query =
        serde_urlencoded::from_str(parts.uri.query().unwrap_or_default())
            .map_err(DecodeError::Query)?;

    let body = R::Body::decode(body).map_err(DecodeError::Body)?;

    Ok(R::from_parts(query, body))
}

impl<B: Body> Request for B {
    type Query = ();
    type Body = Self;

    #[inline]
    fn from_parts((): (), body: Self) -> Self {
        body
    }
}

impl<Q: schemars::JsonSchema, B: Body> Request for WithQuery<Q, B> {
    type Query = Q;
    type Body = B;

    #[inline]
    fn from_parts(query: Q, body: B) -> Self {
        Self { query, body }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::body::Json;

    #[derive(
        Debug,
        PartialEq,
        serde::Serialize,
        serde::Deserialize,
        schemars::JsonSchema,
    )]
    struct Limit {
        limit: u32,
    }

    fn decode_from<R>(
        uri: &str,
        content_type: Option<&str>,
        body: &str,
    ) -> Result<R, DecodeError<<R::Body as Decode>::Error>>
    where
        R: Request<Query: serde::de::DeserializeOwned, Body: Decode>,
    {
        let mut request = http::Request::builder().uri(uri);

        if let Some(content_type) = content_type {
            request = request.header(http::header::CONTENT_TYPE, content_type);
        }

        let (parts, ()) =
            request.body(()).expect("the request is valid").into_parts();

        decode(&parts, body.as_bytes())
    }

    #[test]
    fn query_and_json_body() {
        let request = decode_from::<WithQuery<Limit, Json<Limit>>>(
            "/?limit=3",
            Some("application/json; charset=utf-8"),
            r#"{"limit":4}"#,
        )
        .expect("the request decodes");

        assert_eq!(request.query, Limit { limit: 3 });
        assert_eq!(request.body, Json(Limit { limit: 4 }));
    }

    #[test]
    fn json_body_needs_its_content_type() {
        let error = decode_from::<Json<Limit>>("/", None, r#"{"limit":4}"#)
            .unwrap_err();

        assert!(matches!(error, DecodeError::MediaType { .. }));
    }

    #[test]
    fn empty_body_rejects_bytes() {
        let error = decode_from::<()>("/", None, "{}").unwrap_err();

        assert!(matches!(error, DecodeError::Body(_)));
    }

    #[test]
    fn invalid_query() {
        let error = decode_from::<WithQuery<Limit>>("/?limit=many", None, "")
            .unwrap_err();

        assert!(matches!(error, DecodeError::Query(_)));
    }
}
