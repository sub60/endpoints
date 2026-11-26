//! The `GET /{account}/{cache}/{narinfo_file_name}` endpoint.
//!
//! Gets a NARInfo.

use std::io::Write as _;

use bytes::BufMut;
use http::{Method, StatusCode};
use nix_types::{NarInfo, NarInfoFileName};
use sub60_endpoints_common::PathParams;
use sub60_endpoints_common::body::{Body, Encode};
use sub60_endpoints_common::response::{NotFound, Ok};

use crate::CachePath;

#[derive(Debug, Copy, Clone)]
pub struct Endpoint;

/// The path parameters of the endpoint's requests.
#[derive(Debug, Clone, PartialEq, Eq, derive_more::AsRef, PathParams)]
#[route(CachePath + "/{narinfo_file_name}")]
pub struct Path {
    /// The cache the NARInfo is in.
    #[as_ref]
    pub cache: CachePath,

    /// The NARInfo's file name.
    pub narinfo_file_name: NarInfoFileName,
}

#[derive(Debug, PartialEq, Eq, sub60_endpoints_common::response::Response)]
#[expect(clippy::large_enum_variant)]
pub enum Response {
    /// The NARInfo, signed with the cache's key.
    Found(Ok<NarInfoBody>),

    /// The NARInfo doesn't exist.
    NotFound(NotFound),
}

/// The body of a request or response holding a NARInfo.
#[derive(Debug, PartialEq, Eq)]
pub struct NarInfoBody(pub NarInfo<String>);

impl sub60_endpoints_common::Endpoint for Endpoint {
    const METHOD: Method = Method::GET;

    type Path = Path;
    type Request = ();
    type Response = Response;
    type Error = StatusCode;
}

impl Body for NarInfoBody {
    const MEDIA_TYPE: Option<&'static str> = Some("text/x-nix-narinfo");

    #[inline]
    fn schema(_: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schemars::json_schema!({ "type": "string" })
    }
}

impl Encode for NarInfoBody {
    #[inline]
    fn encode(&self, buf: &mut impl BufMut) {
        write!(buf.writer(), "{}", self.0)
            .expect("the buffer has room for the body");
    }
}
