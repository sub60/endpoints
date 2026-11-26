//! The `GET /caches/{cache_id}` endpoint.
//!
//! Gets a [`Cache`].

use http::Method;
use sub60_endpoints_common::body::{Json, WithMessage};
use sub60_endpoints_common::response::{Ok, WithStatusOf};
use sub60_endpoints_common::{PathParams, error};

use crate::{Cache, CacheId};

#[derive(Debug, Copy, Clone)]
pub struct Endpoint;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PathParams)]
#[route("/caches/{cache_id}")]
pub struct Path {
    pub cache_id: CacheId,
}

#[derive(
    Debug,
    derive_more::Display,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    sub60_endpoints_common::Statuses,
)]
#[serde(tag = "code", rename_all = "snake_case")]
pub enum Error {
    Unauthorized(error::Unauthorized),
    NotFound(error::NotFound),
    Internal(error::Internal),
}

impl sub60_endpoints_common::Endpoint for Endpoint {
    const METHOD: Method = Method::GET;

    type Path = Path;
    type Request = ();
    type Response = Ok<Json<Cache>>;
    type Error = WithStatusOf<Json<WithMessage<Error>>>;
}
