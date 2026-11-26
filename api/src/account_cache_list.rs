//! The `GET /accounts/{account_id}/caches` endpoint.
//!
//! Lists an account's caches, sorted by name.

use core::num::NonZeroU16;

use http::Method;
use sub60_endpoints_common::body::{Json, WithMessage};
use sub60_endpoints_common::request::WithQuery;
use sub60_endpoints_common::response::{Ok, WithStatusOf};
use sub60_endpoints_common::{CacheName, Cursor, PathParams, error};

use crate::{AccountId, Cache};

#[derive(Debug, Copy, Clone)]
pub struct Endpoint;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PathParams)]
#[route("/accounts/{account_id}/caches")]
pub struct Path {
    pub account_id: AccountId,
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
pub struct Query {
    /// The name of the only cache to list.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<CacheName>,
    pub limit: NonZeroU16,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<Cursor>,
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
#[schemars(rename = "CacheList")]
pub struct Response {
    pub caches: Vec<Cache>,
    pub next_cursor: Option<Cursor>,
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
    /// The account doesn't exist, or the caller can't see it.
    NotFound(error::NotFound),
    InvalidCursor(error::InvalidCursor),
    Internal(error::Internal),
}

impl sub60_endpoints_common::Endpoint for Endpoint {
    const METHOD: Method = Method::GET;

    type Path = Path;
    type Request = WithQuery<Query>;
    type Response = Ok<Json<Response>>;
    type Error = WithStatusOf<Json<WithMessage<Error>>>;
}
