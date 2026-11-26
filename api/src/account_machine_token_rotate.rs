//! The `POST /accounts/{account_id}/machine-tokens/{token_id}/rotate`
//! endpoint.
//!
//! Gives a machine token a new secret, revoking the old one.

use http::Method;
use sub60_endpoints_common::body::{Json, WithMessage};
use sub60_endpoints_common::response::Ok;
use sub60_endpoints_common::{PathParams, error};

use crate::{AccountId, MachineTokenId, account_machine_token_create};

#[derive(Debug, Copy, Clone)]
pub struct Endpoint;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PathParams)]
#[route("/accounts/{account_id}/machine-tokens/{token_id}/rotate")]
pub struct Path {
    pub account_id: AccountId,
    pub token_id: MachineTokenId,
}

#[derive(
    Debug,
    derive_more::Display,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    sub60_endpoints_common::response::Response,
)]
#[serde(tag = "code", rename_all = "snake_case")]
pub enum Error {
    Unauthorized(error::Unauthorized),
    Forbidden(error::Forbidden),
    NotFound(error::NotFound),
    Internal(error::Internal),
}

impl sub60_endpoints_common::Endpoint for Endpoint {
    const METHOD: Method = Method::POST;

    type Path = Path;
    type Request = ();
    type Response = Ok<Json<account_machine_token_create::Response>>;
    type Error = Json<WithMessage<Error>>;
}
