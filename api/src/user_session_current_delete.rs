//! The `DELETE /user/sessions/current` endpoint.
//!
//! Logs out from the session the request is authenticated with.

use http::Method;
use sub60_endpoints_common::body::{Json, WithMessage};
use sub60_endpoints_common::response::{NoContent, WithStatusOf};
use sub60_endpoints_common::{PathParams, error};

#[derive(Debug, Copy, Clone)]
pub struct Endpoint;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PathParams)]
#[route("/user/sessions/current")]
pub struct Path;

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
    Internal(error::Internal),
}

impl sub60_endpoints_common::Endpoint for Endpoint {
    const METHOD: Method = Method::DELETE;

    type Path = Path;
    type Request = ();
    type Response = NoContent;
    type Error = WithStatusOf<Json<WithMessage<Error>>>;
}
