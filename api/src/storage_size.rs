//! The `GET /storage-size` endpoint.
//!
//! Gets the number of bytes stored across all caches.

use http::{Method, StatusCode};
use sub60_endpoints_common::{Json, JsonError, Ok, PathParams};

#[derive(Debug, Copy, Clone)]
#[expect(missing_docs)]
pub struct Endpoint;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PathParams)]
#[route("/storage-size")]
#[expect(missing_docs)]
pub struct Path;

#[derive(Debug, PartialEq, Eq, serde::Serialize, schemars::JsonSchema)]
#[schemars(rename = "StorageSizeResponse")]
#[expect(missing_docs)]
pub struct Response {
    /// The number of bytes stored across all caches, counting each NAR once
    /// however many caches reference it, plus every NARInfo.
    pub bytes: u64,
}

#[derive(
    Debug, derive_more::Display, serde::Serialize, schemars::JsonSchema,
)]
#[serde(tag = "code", content = "details", rename_all = "snake_case")]
#[schemars(rename = "StorageSizeError")]
#[expect(missing_docs)]
pub enum Error {
    /// The server failed to sum the sizes.
    #[display("the server failed to sum the sizes")]
    Internal,
}

impl sub60_endpoints_common::Endpoint for Endpoint {
    const METHOD: Method = Method::GET;

    type Error = JsonError<Error>;
    type Path = Path;
    type Request = ();
    type Response = Ok<Json<Response>>;
}

impl From<Error> for StatusCode {
    fn from(error: Error) -> Self {
        match error {
            Error::Internal => Self::INTERNAL_SERVER_ERROR,
        }
    }
}
