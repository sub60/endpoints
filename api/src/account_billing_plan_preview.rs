//! The `GET /accounts/{account_id}/billing/plan/preview` endpoint.
//!
//! Previews switching an account to a paid plan.

use http::Method;
use sub60_endpoints_common::body::{Json, WithMessage};
use sub60_endpoints_common::request::WithQuery;
use sub60_endpoints_common::response::Ok;
use sub60_endpoints_common::{PathParams, error};

use crate::{AccountId, Money, PaidPlan};

#[derive(Debug, Copy, Clone)]
pub struct Endpoint;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PathParams)]
#[route("/accounts/{account_id}/billing/plan/preview")]
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
pub struct Query {
    pub plan: PaidPlan,
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
#[schemars(rename = "PlanPreview")]
pub struct Response {
    /// The amount the switch charges right away, or `None` if it charges
    /// nothing. An upgrade charges the price difference for the rest of the
    /// current period, a first subscription the plan's full price, and a
    /// downgrade or a switch to the current plan nothing.
    pub charge: Option<Money>,
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
    /// The account's subscription is past due, and its debt has to be paid
    /// before its plan can change.
    #[display("the account's subscription is past due")]
    #[status(CONFLICT)]
    PastDue,
    Internal(error::Internal),
}

impl sub60_endpoints_common::Endpoint for Endpoint {
    const METHOD: Method = Method::GET;

    type Path = Path;
    type Request = WithQuery<Query>;
    type Response = Ok<Json<Response>>;
    type Error = Json<WithMessage<Error>>;
}
