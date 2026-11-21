//! The `PATCH /caches/{cache_id}` endpoint.
//!
//! Changes a cache's settings, and returns the updated [`Cache`].

use http::Method;
use sub60_endpoints_common::{Json, Ok, Permission, WithMessage, error};

use crate::{Cache, InvalidPublicPermissions, cache_get};

pub type Path = cache_get::Path;

#[derive(Debug, Copy, Clone)]
pub struct Endpoint;

#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
)]
#[schemars(rename = "UpdateCacheRequest")]
pub struct Request {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub public_permissions: Option<Vec<Permission>>,
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
    InvalidPublicPermissions(InvalidPublicPermissions),
    Internal(error::Internal),
}

impl sub60_endpoints_common::Endpoint for Endpoint {
    const METHOD: Method = Method::PATCH;

    type Path = Path;
    type Request = Json<Request>;
    type Response = Ok<Json<Cache>>;
    type Error = Json<WithMessage<Error>>;
}
