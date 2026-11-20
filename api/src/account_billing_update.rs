//! The `PATCH /accounts/{account_id}/billing` endpoint.
//!
//! Changes an account's billing details, and returns the updated [`Billing`].

use http::Method;
use sub60_endpoints_common::{Json, Ok, WithMessage, error};

use crate::{Billing, account_billing_get};

#[expect(missing_docs)]
pub type Path = account_billing_get::Path;

#[derive(Debug, Copy, Clone)]
#[expect(missing_docs)]
pub struct Endpoint;

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
#[schemars(rename = "UpdateBillingRequest")]
#[expect(missing_docs)]
pub struct Request {
    /// The new email address billing notices are sent to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub billing_email: Option<String>,
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
#[expect(missing_docs)]
pub enum Error {
    Unauthorized(error::Unauthorized),
    Forbidden(error::Forbidden),
    NotFound(error::NotFound),
    /// The new billing email isn't a valid email address.
    #[display("the billing email isn't a valid email address")]
    #[status(BAD_REQUEST)]
    InvalidEmail,
    Internal(error::Internal),
}

impl sub60_endpoints_common::Endpoint for Endpoint {
    const METHOD: Method = Method::PATCH;

    type Path = Path;
    type Request = Json<Request>;
    type Response = Ok<Json<Billing>>;
    type Error = Json<WithMessage<Error>>;
}
