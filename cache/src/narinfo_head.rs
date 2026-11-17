//! The `HEAD /{account}/{cache}/{narinfo_file_name}` endpoint.
//!
//! Checks whether a NARInfo exists.

use http::{Method, StatusCode};

use crate::narinfo_get;

#[derive(Debug, Copy, Clone)]
#[expect(missing_docs)]
pub struct Endpoint;

/// The path parameters of the endpoint's requests.
pub type Path = narinfo_get::Path;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[expect(missing_docs)]
pub enum Response {
    /// The NARInfo exists.
    Exists,

    /// The NARInfo doesn't exist.
    NotFound,
}

impl crate::Endpoint for Endpoint {
    const METHOD: Method = Method::HEAD;

    type Path = Path;
    type Request = ();
    type Response = Response;
}

impl Response {
    /// Returns the response's status code.
    #[inline]
    pub fn status_code(&self) -> StatusCode {
        match self {
            Self::Exists => StatusCode::OK,
            Self::NotFound => StatusCode::NOT_FOUND,
        }
    }
}
