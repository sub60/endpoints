//! The `PATCH /grants/{grant_id}` endpoint.
//!
//! Changes a grant's permissions, or the repos and refs a CI job grant matches.

use http::Method;
use sub60_endpoints_common::body::{Json, WithMessage};
use sub60_endpoints_common::response::{Ok, WithStatusOf};
use sub60_endpoints_common::{PathParams, Permission, error, status};

use crate::{CiJobs, Grant, GrantId, InvalidGrantPermissions};

#[derive(Debug, Copy, Clone)]
pub struct Endpoint;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PathParams)]
#[route("/grants/{grant_id}")]
pub struct Path {
    pub grant_id: GrantId,
}

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
#[schemars(rename = "UpdateGrantRequest")]
pub struct Request {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<Permission>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ci_jobs: Option<CiJobs>,
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
    Forbidden(error::Forbidden),
    NotFound(error::NotFound),
    InvalidPermissions(InvalidGrantPermissions),
    /// The request has `ci_jobs`, but the grant is to a user or a machine
    /// token.
    #[display(
        "the request has `ci_jobs`, but the grant is to a user or a machine \
         token, not to CI jobs"
    )]
    NotACiJobGrant(status::BadRequest),
    /// The grant is the only one giving an org's owner every permission, and
    /// the request takes some away.
    #[display("the change would leave the org without an owner")]
    LastOwner(status::Conflict),
    Internal(error::Internal),
}

impl sub60_endpoints_common::Endpoint for Endpoint {
    const METHOD: Method = Method::PATCH;

    type Path = Path;
    type Request = Json<Request>;
    type Response = Ok<Json<Grant>>;
    type Error = WithStatusOf<Json<WithMessage<Error>>>;
}
