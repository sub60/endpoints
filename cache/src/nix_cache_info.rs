//! The `GET /{account}/{cache}/nix-cache-info` endpoint.
//!
//! Gets a cache's `nix-cache-info`.

use std::io::Write as _;

use bytes::BufMut;
use http::{Method, StatusCode};
use nix_types::NixCacheInfo;
use smol_str::SmolStr;
use sub60_endpoints_common::PathParams;
use sub60_endpoints_common::body::{Body, Encode};
use sub60_endpoints_common::response::Ok;

use crate::CachePath;

#[derive(Debug, Copy, Clone)]
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

    type Path = Path;
    type Request = ();
    type Response = Ok<NixCacheInfoBody>;
    type Error = StatusCode;
}

impl Body for NixCacheInfoBody {
    const MEDIA_TYPE: Option<&'static str> = Some("text/x-nix-cache-info");

    #[inline]
    fn schema(_: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schemars::json_schema!({ "type": "string" })
    }
}

impl Encode for NixCacheInfoBody {
    #[inline]
    fn encode(&self, buf: &mut impl BufMut) {
        write!(buf.writer(), "{}", self.0)
            .expect("the buffer has room for the body");
    }
}
