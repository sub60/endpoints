//! The `POST /accounts` endpoint.
//!
//! Creates an account for the user the request's token acts as. This can be
//! used to create a user's personal account, if it wasn't created at sign-up
//! due to their sign-up identity's name being already taken or reserved, or a
//! new org account, which makes the user its owner.

use http::{Method, StatusCode};
use sub60_endpoints_common::{AccountName, Created, Json, JsonError};

use crate::{Account, AccountKind, account_list};

#[derive(Debug, Copy, Clone)]
#[expect(missing_docs)]
pub struct Endpoint;

#[expect(missing_docs)]
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
#[expect(missing_docs)]
pub struct Request {
    pub name: AccountName,
    pub kind: AccountKind,
}

#[derive(
    Debug, derive_more::Display, serde::Serialize, schemars::JsonSchema,
)]
#[serde(tag = "code", content = "details", rename_all = "snake_case")]
#[schemars(rename = "CreateAccountError")]
#[expect(missing_docs)]
pub enum Error {
    /// The request has no user token, or its token is invalid or expired.
    #[display("the request needs a valid user token")]
    Unauthorized,

    /// Another account has the name, ignoring case.
    #[display("the name is already taken by another account")]
    NameTaken,

    /// The request asks for a personal account, but the user already has one.
    #[display("the user already has a personal account")]
    PersonalAccountExists,

    /// The server failed to create the account.
    #[display("the server failed to create the account")]
    Internal,
}

impl sub60_endpoints_common::Endpoint for Endpoint {
    const METHOD: Method = Method::POST;

    type Path = Path;
    type Request = Json<Request>;
    type Response = Created<Json<Account>>;
    type Error = JsonError<Error>;
}

impl From<Error> for StatusCode {
    fn from(error: Error) -> Self {
        match error {
            Error::Unauthorized => Self::UNAUTHORIZED,
            Error::NameTaken | Error::PersonalAccountExists => Self::CONFLICT,
            Error::Internal => Self::INTERNAL_SERVER_ERROR,
        }
    }
}
