//! The `POST /accounts` endpoint.
//!
//! Creates an org or the caller's personal account.

use http::Method;
use sub60_endpoints_common::{
    AccountName,
    Created,
    Json,
    PathParams,
    WithMessage,
    error,
};

use crate::{Account, AccountKind};

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
#[schemars(rename = "CreateAccountRequest")]
pub struct Request {
    pub name: AccountName,
    pub kind: AccountKind,
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
    /// Another account has the name, ignoring case.
    #[display("the name is already taken by another account")]
    #[status(CONFLICT)]
    NameTaken,
    /// The request asks for a personal account, but the user already has one.
    #[display("the user already has a personal account")]
    #[status(CONFLICT)]
    PersonalAccountExists,
    /// The request asks for an org, but the user has no personal account yet.
    #[display(
        "the request creates an org, but the user has no personal account yet"
    )]
    #[status(CONFLICT)]
    PersonalAccountRequired,
    Internal(error::Internal),
}

impl sub60_endpoints_common::Endpoint for Endpoint {
    const METHOD: Method = Method::POST;

    type Path = Path;
    type Request = Json<Request>;
    type Response = Created<Json<Account>>;
    type Error = Json<WithMessage<Error>>;
}
