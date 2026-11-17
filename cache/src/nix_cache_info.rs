//! The `GET /{account}/{cache}/nix-cache-info` endpoint.
//!
//! Gets a cache's `nix-cache-info`.

use http::Method;
use nix_types::NixCacheInfo;
use smol_str::SmolStr;

use crate::{CachePath, PathParams};

#[derive(Debug, Copy, Clone)]
#[expect(missing_docs)]
pub struct Endpoint;

/// The path parameters of the endpoint's requests.
#[derive(Debug, Clone, PartialEq, Eq, derive_more::AsRef, PathParams)]
#[route(CachePath + "/nix-cache-info")]
pub struct Path {
    /// The cache.
    #[as_ref]
    pub cache: CachePath,
}

#[derive(Debug, PartialEq, Eq)]
#[expect(missing_docs)]
pub struct Response(pub NixCacheInfo<SmolStr>);

impl crate::Endpoint for Endpoint {
    const METHOD: Method = Method::GET;

    type Path = Path;
    type Request = ();
    type Response = Response;
}

#[cfg(feature = "axum")]
impl axum::response::IntoResponse for Response {
    fn into_response(self) -> axum::response::Response {
        use axum::http::{HeaderValue, header};

        const CONTENT_TYPE: HeaderValue =
            HeaderValue::from_static("text/x-nix-cache-info");

        ([(header::CONTENT_TYPE, CONTENT_TYPE)], self.0.to_string())
            .into_response()
    }
}
