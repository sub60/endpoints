//! The `DELETE /accounts/{account_id}` endpoint.
//!
//! Deletes an org account.

use http::{Method, StatusCode};
use sub60_endpoints_common::{JsonError, NoContent, Permission};

use crate::account_get;

#[derive(Debug, Copy, Clone)]
#[expect(missing_docs)]
pub struct Endpoint;

#[expect(missing_docs)]
pub type Path = account_get::Path;

#[derive(
    Debug, derive_more::Display, serde::Serialize, schemars::JsonSchema,
)]
#[serde(tag = "code", content = "details", rename_all = "snake_case")]
#[schemars(rename = "DeleteAccountError")]
#[expect(missing_docs)]
pub enum Error {
    /// The request has no token, or its token is invalid or expired.
    #[display("the request needs a valid token")]
    Unauthorized,

    /// The principal can see the account, but can't delete it.
    #[display("the request's principal can't delete the account")]
    Forbidden {
        /// The permissions the principal is missing on the account.
        missing: Vec<Permission>,
    },

    /// No account has the ID, or the principal can't see it.
    #[display("there's no account with the ID")]
    NotFound,

    /// The account is a personal one, which can only be deleted by deleting its
    /// user.
    #[display(
        "a personal account can only be deleted by deleting its user, with \
         DELETE /user"
    )]
    PersonalAccount,

    /// The server failed to delete the account.
    #[display("the server failed to delete the account")]
    Internal,
}

impl sub60_endpoints_common::Endpoint for Endpoint {
    const METHOD: Method = Method::DELETE;

    type Path = Path;
    type Request = ();
    type Response = NoContent;
    type Error = JsonError<Error>;
}

impl From<Error> for StatusCode {
    fn from(error: Error) -> Self {
        match error {
            Error::Unauthorized => Self::UNAUTHORIZED,
            Error::Forbidden { .. } => Self::FORBIDDEN,
            Error::NotFound => Self::NOT_FOUND,
            Error::PersonalAccount => Self::CONFLICT,
            Error::Internal => Self::INTERNAL_SERVER_ERROR,
        }
    }
}
