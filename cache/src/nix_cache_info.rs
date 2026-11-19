//! The `GET /{account}/{cache}/nix-cache-info` endpoint.
//!
//! Gets a cache's `nix-cache-info`.

use http::{Method, StatusCode};
use nix_types::NixCacheInfo;
use smol_str::SmolStr;
use sub60_endpoints_common::{Body, Ok, PathParams};

use crate::CachePath;

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

/// The body of a response holding a cache's `nix-cache-info`.
#[derive(Debug, PartialEq, Eq)]
pub struct NixCacheInfoBody(pub NixCacheInfo<SmolStr>);

impl sub60_endpoints_common::Endpoint for Endpoint {
    const METHOD: Method = Method::GET;

    type Error = StatusCode;
    type Path = Path;
    type Request = ();
    type Response = Ok<NixCacheInfoBody>;
}

impl Body for NixCacheInfoBody {
    const MEDIA_TYPE: Option<&'static str> = Some("text/x-nix-cache-info");

    #[inline]
    fn schema(_: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schemars::json_schema!({ "type": "string" })
    }
}

#[cfg(feature = "axum")]
impl axum::response::IntoResponse for NixCacheInfoBody {
    fn into_response(self) -> axum::response::Response {
        use axum::http::{HeaderValue, header};

        const CONTENT_TYPE: HeaderValue =
            HeaderValue::from_static("text/x-nix-cache-info");

        ([(header::CONTENT_TYPE, CONTENT_TYPE)], self.0.to_string())
            .into_response()
    }
}
