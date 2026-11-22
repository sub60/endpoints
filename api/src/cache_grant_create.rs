//! The `POST /caches/{cache_id}/grants` endpoint.
//!
//! Gives a principal permissions on a cache.

use http::Method;
use sub60_endpoints_common::{Created, Json, Permission, WithMessage, error};

use crate::{
    Grant,
    GrantId,
    InvalidGrantPermissions,
    Principal,
    cache_grant_list,
};

pub type Path = cache_grant_list::Path;

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
#[schemars(rename = "CreateCacheGrantRequest")]
pub struct Request {
    pub principal: Principal,
    pub permissions: Vec<Permission>,
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
    /// The cache or the principal doesn't exist, or the caller can't see it.
    NotFound(error::NotFound),
    /// The user principal's [`id`](Principal::User::id) is an org.
    #[display("the user principal's ID is an org's, not a personal account's")]
    #[status(BAD_REQUEST)]
    NotAPersonalAccount,
    InvalidPermissions(InvalidGrantPermissions),
    /// The user or machine token already has a grant on the cache.
    #[display(
        "the principal already has a grant on the cache, so it can't get a \
         second one"
    )]
    #[status(CONFLICT)]
    Conflict {
        grant_id: GrantId,
    },
    Internal(error::Internal),
}

impl sub60_endpoints_common::Endpoint for Endpoint {
    const METHOD: Method = Method::POST;

    type Path = Path;
    type Request = Json<Request>;
    type Response = Created<Json<Grant>>;
    type Error = Json<WithMessage<Error>>;
}
