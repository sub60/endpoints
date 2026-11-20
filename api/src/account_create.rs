//! The `POST /accounts` endpoint.
//!
//! Creates an account for the user the request's token acts as. This can be
//! used to create a user's personal account, if it wasn't created at sign-up
//! due to their sign-up identity's name being already taken or reserved, or a
//! new org account, which makes the user its owner.

use http::Method;
use sub60_endpoints_common::{AccountName, Created, Json, WithMessage, error};

use crate::{Account, AccountKind, account_list};

#[derive(Debug, Copy, Clone)]
pub struct Endpoint;

pub type Path = account_list::Path;

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
    Internal(error::Internal),
}

impl sub60_endpoints_common::Endpoint for Endpoint {
    const METHOD: Method = Method::POST;

    type Path = Path;
    type Request = Json<Request>;
    type Response = Created<Json<Account>>;
    type Error = Json<WithMessage<Error>>;
}
