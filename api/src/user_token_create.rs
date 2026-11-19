//! The `POST /user/tokens` endpoint.
//!
//! Creates a user token for the caller.

use http::Method;
use sub60_endpoints_common::{Created, Json, Permission, WithMessage, error};

use crate::{Expiration, UserToken, user_token_list};

pub type Path = user_token_list::Path;

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
#[schemars(rename = "CreateUserTokenRequest")]
pub struct Request {
    pub description: String,
    pub max_permissions: Vec<Permission>,
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
#[schemars(rename = "NewUserToken")]
pub struct Response {
    #[serde(flatten)]
    pub token: UserToken,
    pub secret: String,
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
    /// The request's `max_permissions` has permissions the request's token
    /// doesn't.
    Forbidden(error::Forbidden),
    #[display("the token's expiration time is in the past")]
    #[status(BAD_REQUEST)]
    ExpiresInThePast,
    Internal(error::Internal),
}

impl sub60_endpoints_common::Endpoint for Endpoint {
    const METHOD: Method = Method::POST;

    type Path = Path;
    type Request = Json<Request>;
    type Response = Created<Json<Response>>;
    type Error = Json<WithMessage<Error>>;
}
