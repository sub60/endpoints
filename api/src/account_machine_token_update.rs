//! The `PATCH /accounts/{account_id}/machine-tokens/{token_id}` endpoint.
//!
//! Changes a machine token's description or expiration.

use http::Method;
use sub60_endpoints_common::{Json, Ok, WithMessage, error};

use crate::{Expiration, MachineToken, account_machine_token_get};

pub type Path = account_machine_token_get::Path;

#[derive(Debug, Copy, Clone)]
pub struct Endpoint;

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
#[schemars(rename = "UpdateMachineTokenRequest")]
pub struct Request {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<Expiration>,
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
    /// The request's `expires_at` is in the past.
    #[display("the token's expiration time is in the past")]
    #[status(BAD_REQUEST)]
    ExpiresInThePast,
    Internal(error::Internal),
}

impl sub60_endpoints_common::Endpoint for Endpoint {
    const METHOD: Method = Method::PATCH;

    type Path = Path;
    type Request = Json<Request>;
    type Response = Ok<Json<MachineToken>>;
    type Error = Json<WithMessage<Error>>;
}
