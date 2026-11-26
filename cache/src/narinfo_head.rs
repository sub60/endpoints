//! The `HEAD /{account}/{cache}/{narinfo_file_name}` endpoint.
//!
//! Checks whether a NARInfo exists.

use http::{Method, StatusCode};
use sub60_endpoints_common::response::{NotFound, Ok};

use crate::narinfo_get;

pub type Path = narinfo_get::Path;

#[derive(Debug, Copy, Clone)]
pub struct Endpoint;

#[derive(
    Debug, Copy, Clone, PartialEq, Eq, sub60_endpoints_common::Alternatives,
)]
pub enum Response {
    /// The NARInfo exists.
    Exists(Ok),

    /// The NARInfo doesn't exist.
    NotFound(NotFound),
}

impl sub60_endpoints_common::Endpoint for Endpoint {
    const METHOD: Method = Method::HEAD;

    type Path = Path;
    type Request = ();
    type Response = Response;
    type Error = StatusCode;
}
