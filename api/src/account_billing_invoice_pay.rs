//! The `POST /accounts/{account_id}/billing/invoices/{invoice_id}/pay`
//! endpoint.
//!
//! Pays an open invoice.

use http::Method;
use sub60_endpoints_common::body::{Json, WithMessage};
use sub60_endpoints_common::response::{Accepted, Ok};
use sub60_endpoints_common::{PathParams, error};

use crate::{AccountId, ChargeConfirmation, Invoice, InvoiceId};

#[derive(Debug, Copy, Clone)]
pub struct Endpoint;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PathParams)]
#[route("/accounts/{account_id}/billing/invoices/{invoice_id}/pay")]
pub struct Path {
    pub account_id: AccountId,
    pub invoice_id: InvoiceId,
}

#[derive(
    Debug, Clone, PartialEq, Eq, sub60_endpoints_common::response::Response,
)]
pub enum Response {
    Paid(Ok<Json<Invoice>>),
    ConfirmationRequired(Accepted<Json<ChargeConfirmation>>),
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
    /// The invoice is already paid or void.
    #[display("the invoice isn't open")]
    #[status(CONFLICT)]
    InvoiceNotOpen,
    /// The charge was declined.
    #[display("the payment failed: {reason}")]
    #[status(PAYMENT_REQUIRED)]
    PaymentFailed {
        reason: String,
    },
    Internal(error::Internal),
}

impl sub60_endpoints_common::Endpoint for Endpoint {
    const METHOD: Method = Method::POST;

    type Path = Path;
    type Request = ();
    type Response = Response;
    type Error = Json<WithMessage<Error>>;
}
