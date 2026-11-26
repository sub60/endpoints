//! The `PATCH /user/tokens/{token_id}` endpoint.
//!
//! Changes a user token's description or expiration.

use http::Method;
use sub60_endpoints_common::body::{Json, WithMessage};
use sub60_endpoints_common::response::Ok;
use sub60_endpoints_common::{PathParams, error};

use crate::{Expiration, UserToken, UserTokenId};

#[derive(Debug, Copy, Clone)]
pub struct Endpoint;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PathParams)]
#[route("/user/tokens/{token_id}")]
pub struct Path {
    pub token_id: UserTokenId,
}

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
#[schemars(rename = "UpdateUserTokenRequest")]
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
    sub60_endpoints_common::response::Response,
)]
#[serde(tag = "code", rename_all = "snake_case")]
pub enum Error {
    Unauthorized(error::Unauthorized),
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
    type Response = Ok<Json<UserToken>>;
    type Error = Json<WithMessage<Error>>;
}
