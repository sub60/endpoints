//! The `GET /accounts/{account_id}/billing` endpoint.
//!
//! Gets an account's [`Billing`].

use http::Method;
use sub60_endpoints_common::body::{Json, WithMessage};
use sub60_endpoints_common::response::{Ok, WithStatusOf};
use sub60_endpoints_common::{PathParams, error};

use crate::{AccountId, Billing};

#[derive(Debug, Copy, Clone)]
pub struct Endpoint;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PathParams)]
#[route("/accounts/{account_id}/billing")]
pub struct Path {
    pub account_id: AccountId,
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
    Forbidden(error::Forbidden),
    NotFound(error::NotFound),
    Internal(error::Internal),
}

impl sub60_endpoints_common::Endpoint for Endpoint {
    const METHOD: Method = Method::GET;

    type Path = Path;
    type Request = ();
    type Response = Ok<Json<Billing>>;
    type Error = WithStatusOf<Json<WithMessage<Error>>>;
}
