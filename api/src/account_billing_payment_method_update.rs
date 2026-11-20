//! The `POST /accounts/{account_id}/billing/payment-method` endpoint.
//!
//! Starts changing an account's payment method. The client will then complete
//! the change by submitting the new payment method to Stripe with the
//! response's client secret.

use http::{Method, StatusCode};
use sub60_endpoints_common::{Json, JsonError, Ok, PathParams, Permission};

use crate::AccountId;

#[derive(Debug, Copy, Clone)]
#[expect(missing_docs)]
pub struct Endpoint;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PathParams)]
#[route("/accounts/{account_id}/billing/payment-method")]
#[expect(missing_docs)]
pub struct Path {
    pub account_id: AccountId,
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
#[schemars(rename = "PaymentMethodUpdate")]
#[expect(missing_docs)]
pub struct Response {
    /// The client secret of the Stripe SetupIntent the new payment method is
    /// submitted with.
    pub client_secret: String,
}

#[derive(
    Debug, derive_more::Display, serde::Serialize, schemars::JsonSchema,
)]
#[serde(tag = "code", content = "details", rename_all = "snake_case")]
#[schemars(rename = "UpdatePaymentMethodError")]
#[expect(missing_docs)]
pub enum Error {
    /// The request has no token, or its token is invalid or expired.
    #[display("the request needs a valid token")]
    Unauthorized,

    /// The principal has access to the account, but can't change its billing.
    #[display("the request's principal can't change the account's billing")]
    Forbidden {
        /// The permissions the principal is missing on the account.
        missing: Vec<Permission>,
    },

    /// No account has the ID, or the principal doesn't have access to it.
    #[display("there's no account with the ID")]
    NotFound,

    /// The server failed to start changing the payment method.
    #[display("the server failed to start changing the payment method")]
    Internal,
}

impl sub60_endpoints_common::Endpoint for Endpoint {
    const METHOD: Method = Method::POST;

    type Path = Path;
    type Request = ();
    type Response = Ok<Json<Response>>;
    type Error = JsonError<Error>;
}

impl From<Error> for StatusCode {
    fn from(error: Error) -> Self {
        match error {
            Error::Unauthorized => Self::UNAUTHORIZED,
            Error::Forbidden { .. } => Self::FORBIDDEN,
            Error::NotFound => Self::NOT_FOUND,
            Error::Internal => Self::INTERNAL_SERVER_ERROR,
        }
    }
}
