//! The `PUT /{account}/{cache}/nar/{nar_file_name}` endpoint.
//!
//! Uploads a NAR.
//!
//! The server refuses NARs whose SHA-256 hash isn't the one in their file name
//! with a `400 Bad Request`.

use http::{Method, StatusCode};

use crate::nar_head;

#[derive(Debug, Copy, Clone)]
#[expect(missing_docs)]
pub struct Endpoint;

/// The path parameters of the endpoint's requests.
pub type Path = nar_head::Path;

#[derive(Debug, Copy, Clone)]
#[expect(missing_docs)]
pub struct RequestBody;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[expect(missing_docs)]
pub enum Response {
    /// The NAR was stored.
    Created,

    /// The NAR already existed, and was left as is.
    AlreadyExists,
}

impl crate::Endpoint for Endpoint {
    const METHOD: Method = Method::PUT;

    type Path = Path;
    type Request = RequestBody;
    type Response = Response;
}

impl Response {
    /// Returns the response's status code.
    #[inline]
    pub fn status_code(&self) -> StatusCode {
        match self {
            Self::Created => StatusCode::CREATED,
            Self::AlreadyExists => StatusCode::NO_CONTENT,
        }
    }
}
