//! The `POST /accounts/{account_id}/machine-tokens` endpoint.
//!
//! Creates a machine token for an account.

use http::Method;
use sub60_endpoints_common::body::{Json, WithMessage};
use sub60_endpoints_common::response::{Created, WithStatusOf};
use sub60_endpoints_common::{error, status};

use crate::{Expiration, MachineToken, account_machine_token_list};

pub type Path = account_machine_token_list::Path;

#[derive(Debug, Copy, Clone)]
pub struct Endpoint;

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
)]
#[schemars(rename = "CreateMachineTokenRequest")]
pub struct Request {
    pub description: String,
    pub expires_at: Expiration,
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
#[schemars(rename = "NewMachineToken")]
pub struct Response {
    #[serde(flatten)]
    pub token: MachineToken,
    pub secret: String,
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
    #[display("the token's expiration time is in the past")]
    ExpiresInThePast(status::BadRequest),
    Internal(error::Internal),
}

impl sub60_endpoints_common::Endpoint for Endpoint {
    const METHOD: Method = Method::POST;

    type Path = Path;
    type Request = Json<Request>;
    type Response = Created<Json<Response>>;
    type Error = WithStatusOf<Json<WithMessage<Error>>>;
}
