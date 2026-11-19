//! The `PUT /{account}/{cache}/{narinfo_file_name}` endpoint.
//!
//! Uploads a NARInfo.
//!
//! Its NAR must already have been uploaded.

use http::{Method, StatusCode};
use nix_types::NarInfo;

use crate::narinfo_get;

#[derive(Debug, Copy, Clone)]
#[expect(missing_docs)]
pub struct Endpoint;

/// The path parameters of the endpoint's requests.
pub type Path = narinfo_get::Path;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[expect(missing_docs)]
pub enum Response {
    /// The NARInfo was stored.
    Created,

    /// The NARInfo already existed, and was left as is.
    AlreadyExists,
}

impl crate::Endpoint for Endpoint {
    const METHOD: Method = Method::PUT;

    type Error = StatusCode;
    type Path = Path;
    type Request = NarInfo<String>;
    type Response = Response;
}

impl Response {
    /// Returns the response's status code.
    #[inline]
    pub fn status_code(&self) -> StatusCode {
        match self {
            Self::Created => StatusCode::CREATED,
            Self::AlreadyExists => StatusCode::OK,
        }
    }
}
