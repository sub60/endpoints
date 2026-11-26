//! The `GET /user/tokens` endpoint.
//!
//! Lists the caller's user tokens, newest first.

use core::num::NonZeroU16;

use http::Method;
use sub60_endpoints_common::body::{Json, WithMessage};
use sub60_endpoints_common::request::WithQuery;
use sub60_endpoints_common::response::{Ok, WithStatusOf};
use sub60_endpoints_common::{Cursor, PathParams, error};

use crate::UserToken;

#[derive(Debug, Copy, Clone)]
pub struct Endpoint;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PathParams)]
#[route("/user/tokens")]
pub struct Path;

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
#[schemars(rename = "UserTokenList")]
pub struct Response {
    pub tokens: Vec<UserToken>,
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
