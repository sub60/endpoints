//! The `GET /caches/{cache_id}/paths/{store_hash}` endpoint.
//!
//! Gets one of a cache's store paths.

use http::Method;
use nix_types::StoreHash;
use sub60_endpoints_common::{Json, Ok, PathParams, WithMessage, error};

use crate::{CacheId, PathInfo};

#[derive(Debug, Copy, Clone)]
pub struct Endpoint;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PathParams)]
#[route("/caches/{cache_id}/paths/{store_hash}")]
pub struct Path {
    pub cache_id: CacheId,
    pub store_hash: StoreHash,
}

#[derive(
    Debug,
    derive_more::Display,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    sub60_endpoints_common::Response,
)]
#[serde(tag = "code", rename_all = "snake_case")]
pub enum Error {
    Unauthorized(error::Unauthorized),
    Forbidden(error::Forbidden),
    NotFound(error::NotFound),
    Internal(error::Internal),
}

impl sub60_endpoints_common::Endpoint for Endpoint {
    const METHOD: Method = Method::GET;

    type Path = Path;
    type Request = ();
    type Response = Ok<Json<PathInfo>>;
    type Error = Json<WithMessage<Error>>;
}
