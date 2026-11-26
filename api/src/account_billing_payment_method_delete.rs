//! The `DELETE /accounts/{account_id}/billing/payment-method` endpoint.
//!
//! Removes an account's payment method.

use http::Method;
use sub60_endpoints_common::body::{Json, WithMessage};
use sub60_endpoints_common::response::{NoContent, WithStatusOf};
use sub60_endpoints_common::{error, status};

use crate::account_billing_payment_method_update;

pub type Path = account_billing_payment_method_update::Path;

#[derive(Debug, Copy, Clone)]
pub struct Endpoint;

#[derive(
    Debug,
    derive_more::Display,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    sub60_endpoints_common::Statuses,
)]
#[serde(tag = "code", rename_all = "snake_case")]
pub enum Error {
    Unauthorized(error::Unauthorized),
    Forbidden(error::Forbidden),
    NotFound(error::NotFound),
    /// The account's subscription will charge the payment method again.
    #[display(
        "the account has an active subscription that will charge the payment \
         method again"
    )]
    PaymentMethodInUse(status::Conflict),
    Internal(error::Internal),
}

impl sub60_endpoints_common::Endpoint for Endpoint {
    const METHOD: Method = Method::DELETE;

    type Path = Path;
    type Request = ();
    type Response = NoContent;
    type Error = WithStatusOf<Json<WithMessage<Error>>>;
}
