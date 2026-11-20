//! The `GET /accounts/{account_id}` endpoint.
//!
//! Gets an [`Account`].

use http::Method;
use sub60_endpoints_common::{Json, Ok, PathParams, WithMessage, error};

use crate::{Account, AccountId};

#[derive(Debug, Copy, Clone)]
pub struct Endpoint;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PathParams)]
#[route("/accounts/{account_id}")]
pub struct Path {
    pub account_id: AccountId,
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
    NotFound(error::NotFound),
    Internal(error::Internal),
}

impl sub60_endpoints_common::Endpoint for Endpoint {
    const METHOD: Method = Method::GET;

    type Path = Path;
    type Request = ();
    type Response = Ok<Json<Account>>;
    type Error = Json<WithMessage<Error>>;
}
