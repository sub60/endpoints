//! The `POST /caches` endpoint.
//!
//! Creates a cache.

use http::Method;
use sub60_endpoints_common::{
    CacheName,
    Created,
    Json,
    Permission,
    WithMessage,
    error,
};

use crate::{AccountId, Cache, InvalidPublicPermissions, cache_list};

pub type Path = cache_list::Path;

#[derive(Debug, Copy, Clone)]
pub struct Endpoint;

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
)]
#[schemars(rename = "CreateCacheRequest")]
pub struct Request {
    pub account: AccountId,
    pub name: CacheName,
    pub priority: u32,
    /// What anyone can do on the cache, with or without a token.
    pub public_permissions: Vec<Permission>,
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
    /// The account doesn't exist, or the caller can't see it.
    NotFound(error::NotFound),
    InvalidPublicPermissions(InvalidPublicPermissions),
    /// The account already has a cache with the name, ignoring case.
    #[display("the account already has a cache with that name")]
    #[status(CONFLICT)]
    NameTaken,
    Internal(error::Internal),
}

impl sub60_endpoints_common::Endpoint for Endpoint {
    const METHOD: Method = Method::POST;

    type Path = Path;
    type Request = Json<Request>;
    type Response = Created<Json<Cache>>;
    type Error = Json<WithMessage<Error>>;
}
