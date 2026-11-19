//! The `GET /accounts` endpoint.
//!
//! Lists the accounts the request's principal has access to, sorted by name.

use core::num::NonZeroU16;

use http::Method;
use sub60_endpoints_common::{
    AccountName,
    Cursor,
    Json,
    Ok,
    PathParams,
    WithMessage,
    WithQuery,
    error,
};

use crate::Account;

#[derive(Debug, Copy, Clone)]
pub struct Endpoint;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PathParams)]
#[route("/accounts")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<AccountName>,
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
#[schemars(rename = "AccountList")]
pub struct Response {
    pub accounts: Vec<Account>,
    pub next_cursor: Option<Cursor>,
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
    InvalidCursor(error::InvalidCursor),
    Internal(error::Internal),
}

impl sub60_endpoints_common::Endpoint for Endpoint {
    const METHOD: Method = Method::GET;

    type Path = Path;
    type Request = WithQuery<Query>;
    type Response = Ok<Json<Response>>;
    type Error = Json<WithMessage<Error>>;
}
