//! The `GET /accounts/{account_id}/usage` endpoint.
//!
//! Gets an account's usage of each product.

use http::Method;
use sub60_endpoints_common::{
    Json,
    Ok,
    PathParams,
    Timestamp,
    WithMessage,
    error,
};

use crate::AccountId;

#[derive(Debug, Copy, Clone)]
pub struct Endpoint;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PathParams)]
#[route("/accounts/{account_id}/usage")]
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
#[schemars(rename = "AccountUsage")]
pub struct Response {
    pub caches: CachesUsage,
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
pub struct CachesUsage {
    /// The total byte size of the account's caches, counting each NAR once even
    /// if it's shared by several caches.
    pub bytes: u64,
    /// The time the usage was measured at.
    pub measured_at: Timestamp,
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
    type Response = Ok<Json<Response>>;
    type Error = Json<WithMessage<Error>>;
}
