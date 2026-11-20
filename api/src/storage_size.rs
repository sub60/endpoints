//! The `GET /storage-size` endpoint.
//!
//! Gets the number of bytes stored across all caches.

use http::Method;
use sub60_endpoints_common::{Json, Ok, PathParams, WithMessage, error};

#[derive(Debug, Copy, Clone)]
pub struct Endpoint;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PathParams)]
#[route("/storage-size")]
pub struct Path;

#[derive(Debug, PartialEq, Eq, serde::Serialize, schemars::JsonSchema)]
#[schemars(rename = "StorageSizeResponse")]
pub struct Response {
    /// The number of bytes stored across all caches, counting each NAR once
    /// however many caches reference it, plus every NARInfo.
    pub bytes: u64,
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
    Internal(error::Internal),
}

impl sub60_endpoints_common::Endpoint for Endpoint {
    const METHOD: Method = Method::GET;

    type Path = Path;
    type Request = ();
    type Response = Ok<Json<Response>>;
    type Error = Json<WithMessage<Error>>;
}
