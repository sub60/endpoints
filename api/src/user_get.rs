//! The `GET /user` endpoint.
//!
//! Gets the user the request's token acts as.

use http::Method;
use sub60_endpoints_common::{Json, Ok, PathParams, WithMessage, error};

use crate::{AccountSummary, UserId};

#[derive(Debug, Copy, Clone)]
#[expect(missing_docs)]
pub struct Endpoint;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PathParams)]
#[route("/user")]
#[expect(missing_docs)]
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
#[expect(missing_docs)]
pub struct User {
    #[expect(missing_docs)]
    pub id: UserId,
    /// The user's personal account, or `None` if it hasn't been created yet.
    pub account: Option<AccountSummary>,
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
    type Request = ();
    type Response = Ok<Json<User>>;
    type Error = Json<WithMessage<Error>>;
}
