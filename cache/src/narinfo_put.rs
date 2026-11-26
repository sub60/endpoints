//! The `PUT /{account}/{cache}/{narinfo_file_name}` endpoint.
//!
//! Uploads a NARInfo.
//!
//! Its NAR must already have been uploaded.

use http::{Method, StatusCode};
use sub60_endpoints_common::response::{Created, Ok};

use crate::narinfo_get::{self, NarInfoBody};

pub type Path = narinfo_get::Path;

#[derive(Debug, Copy, Clone)]
pub struct Endpoint;

#[derive(
    Debug, Copy, Clone, PartialEq, Eq, sub60_endpoints_common::Alternatives,
)]
pub enum Response {
    /// The NARInfo was stored.
    Created(Created),

    /// The NARInfo already existed, and was left as is.
    AlreadyExists(Ok),
}

impl sub60_endpoints_common::Endpoint for Endpoint {
    const METHOD: Method = Method::PUT;

    type Path = Path;
    type Request = NarInfoBody;
    type Response = Response;
    type Error = StatusCode;
}
