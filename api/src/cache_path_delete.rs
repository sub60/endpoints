//! The `POST /caches/{cache_id}/paths/delete` endpoint.
//!
//! Deletes store paths from a cache.

use http::Method;
use nix_types::StoreHash;
use sub60_endpoints_common::{
    BoundedVec,
    Json,
    Ok,
    PathParams,
    WithMessage,
    error,
};

use crate::CacheId;

/// The maximum number of paths a request can delete.
pub const MAX_PATHS: usize = 1000;

#[derive(Debug, Copy, Clone)]
pub struct Endpoint;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PathParams)]
#[route("/caches/{cache_id}/paths/delete")]
pub struct Path {
    pub cache_id: CacheId,
}

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
)]
#[schemars(rename = "DeletePathsRequest")]
pub struct Request {
    pub store_hashes: BoundedVec<StoreHash, MAX_PATHS>,
}

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
)]
#[schemars(rename = "DeletedPaths")]
pub struct Response {
    /// The hashes of the deleted paths.
    pub deleted: Vec<StoreHash>,
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
    const METHOD: Method = Method::POST;

    type Path = Path;
    type Request = Json<Request>;
    type Response = Ok<Json<Response>>;
    type Error = Json<WithMessage<Error>>;
}
