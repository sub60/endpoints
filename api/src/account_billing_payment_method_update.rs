//! The `POST /accounts/{account_id}/billing/payment-method` endpoint.
//!
//! Starts changing an account's payment method. The client will then complete
//! the change by submitting the new payment method to Stripe with the
//! response's client secret.

use http::Method;
use sub60_endpoints_common::body::{Json, WithMessage};
use sub60_endpoints_common::response::Ok;
use sub60_endpoints_common::{PathParams, error};

use crate::AccountId;

#[derive(Debug, Copy, Clone)]
pub struct Endpoint;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PathParams)]
#[route("/accounts/{account_id}/billing/payment-method")]
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
pub struct Response {
    /// The client secret of the Stripe SetupIntent the new payment method is
    /// submitted with.
    pub client_secret: String,
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
    type Response = Ok<Json<Response>>;
    type Error = Json<WithMessage<Error>>;
}
