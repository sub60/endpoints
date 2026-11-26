//! The `DELETE /accounts/{account_id}` endpoint.
//!
//! Deletes an org account.

use http::Method;
use sub60_endpoints_common::body::{Json, WithMessage};
use sub60_endpoints_common::error;
use sub60_endpoints_common::response::NoContent;

use crate::account_get;

pub type Path = account_get::Path;

#[derive(Debug, Copy, Clone)]
pub struct Endpoint;

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
    /// The account is a personal one, which can only be deleted by deleting
    /// its user.
    #[display(
        "a personal account can only be deleted by deleting its user, with \
         DELETE /user"
    )]
    #[status(CONFLICT)]
    PersonalAccount,
    Internal(error::Internal),
}

impl sub60_endpoints_common::Endpoint for Endpoint {
    const METHOD: Method = Method::DELETE;

    type Path = Path;
    type Request = ();
    type Response = NoContent;
    type Error = Json<WithMessage<Error>>;
}
