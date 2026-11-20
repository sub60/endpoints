//! The `GET /accounts` endpoint.
//!
//! Lists the accounts the request's principal has access to, or looks one up by
//! name.

use http::Method;
use sub60_endpoints_common::{
    AccountName,
    Json,
    Ok,
    PathParams,
    WithMessage,
    WithQuery,
    error,
};

use crate::Account;

#[derive(Debug, Copy, Clone)]
#[expect(missing_docs)]
pub struct Endpoint;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PathParams)]
#[route("/accounts")]
#[expect(missing_docs)]
pub struct Path;

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
#[expect(missing_docs)]
pub struct Query {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<AccountName>,
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
#[expect(missing_docs)]
pub struct Response {
    pub accounts: Vec<Account>,
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
#[expect(missing_docs)]
pub enum Error {
    Unauthorized(error::Unauthorized),
    Internal(error::Internal),
}

impl sub60_endpoints_common::Endpoint for Endpoint {
    const METHOD: Method = Method::GET;

    type Path = Path;
    type Request = WithQuery<Query>;
    type Response = Ok<Json<Response>>;
    type Error = Json<WithMessage<Error>>;
}
