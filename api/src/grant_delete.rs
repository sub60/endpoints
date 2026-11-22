//! The `DELETE /grants/{grant_id}` endpoint.
//!
//! Removes a grant.

use http::Method;
use sub60_endpoints_common::{Json, NoContent, WithMessage, error};

use crate::grant_update;

pub type Path = grant_update::Path;

#[derive(Debug, Copy, Clone)]
pub struct Endpoint;

#[derive(
    Debug,
    derive_more::Display,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    sub60_endpoints_common::Response,
)]
#[serde(tag = "code", rename_all = "snake_case")]
pub enum Error {
    Unauthorized(error::Unauthorized),
    Forbidden(error::Forbidden),
    NotFound(error::NotFound),
    /// The grant is the only one giving an org's owner every permission.
    #[display("removing the grant would leave the org without an owner")]
    #[status(CONFLICT)]
    LastOwner,
    Internal(error::Internal),
}

impl sub60_endpoints_common::Endpoint for Endpoint {
    const METHOD: Method = Method::DELETE;

    type Path = Path;
    type Request = ();
    type Response = NoContent;
    type Error = Json<WithMessage<Error>>;
}
