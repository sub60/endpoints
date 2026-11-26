//! The `PUT /accounts/{account_id}/billing/plan` endpoint.
//!
//! Switches an account's plan.

use http::Method;
use sub60_endpoints_common::body::{Json, WithMessage};
use sub60_endpoints_common::response::{Accepted, Ok};
use sub60_endpoints_common::{PathParams, error};

use crate::{AccountId, Billing, ChargeConfirmation, PaidPlan};

#[derive(Debug, Copy, Clone)]
pub struct Endpoint;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PathParams)]
#[route("/accounts/{account_id}/billing/plan")]
pub struct Path {
    pub account_id: AccountId,
}

#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
)]
#[schemars(rename = "UpdatePlanRequest")]
pub struct Request {
    /// The paid plan to switch to, or `None` to go back to the base plan.
    #[serde(deserialize_with = "Option::deserialize")]
    #[schemars(required)]
    pub plan: Option<PaidPlan>,
}

#[derive(
    Debug, Clone, PartialEq, Eq, sub60_endpoints_common::response::Response,
)]
#[expect(clippy::large_enum_variant)]
pub enum Response {
    /// The plan was changed, or its change was scheduled for the next period,
    /// and any charge the change needed was paid.
    Changed(Ok<Json<Billing>>),

    /// The plan's charge is waiting for the customer to confirm it.
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
    /// The account has unpaid invoices that need to be settled before its plan
    /// can change.
    #[display("the account's subscription is past due")]
    #[status(CONFLICT)]
    PastDue,
    /// The charge was declined.
    #[display("the payment failed: {reason}")]
    #[status(PAYMENT_REQUIRED)]
    PaymentFailed {
        reason: String,
    },
    Internal(error::Internal),
}

impl sub60_endpoints_common::Endpoint for Endpoint {
    const METHOD: Method = Method::PUT;

    type Path = Path;
    type Request = Json<Request>;
    type Response = Response;
    type Error = Json<WithMessage<Error>>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_needs_null_plan_when_switching_to_base() {
        let request = |json| serde_json::from_str::<Request>(json);
        assert!(request("{}").is_err());
        assert_eq!(
            request(r#"{"plan":null}"#).unwrap(),
            Request { plan: None },
        );
    }
}
