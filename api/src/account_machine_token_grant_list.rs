//! The `GET /accounts/{account_id}/machine-tokens/{token_id}/grants` endpoint.
//!
//! Lists a machine token's grants, newest first.

use http::Method;
use sub60_endpoints_common::body::{Json, WithMessage};
use sub60_endpoints_common::request::WithQuery;
use sub60_endpoints_common::response::{Ok, WithStatusOf};
use sub60_endpoints_common::{PathParams, error};

use crate::{AccountId, MachineTokenId, account_grant_list};

pub type Query = account_grant_list::Query;

pub type Response = account_grant_list::Response;

#[derive(Debug, Copy, Clone)]
pub struct Endpoint;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PathParams)]
#[route("/accounts/{account_id}/machine-tokens/{token_id}/grants")]
pub struct Path {
    pub account_id: AccountId,
    pub token_id: MachineTokenId,
}

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
    NotFound(error::NotFound),
    InvalidCursor(error::InvalidCursor),
    Internal(error::Internal),
}

impl sub60_endpoints_common::Endpoint for Endpoint {
    const METHOD: Method = Method::GET;

    type Path = Path;
    type Request = WithQuery<Query>;
    type Response = Ok<Json<Response>>;
    type Error = WithStatusOf<Json<WithMessage<Error>>>;
}
