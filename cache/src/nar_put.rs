//! The `PUT /{account}/{cache}/nar/{nar_file_name}` endpoint.
//!
//! Uploads a NAR.
//!
//! The server refuses NARs whose SHA-256 hash isn't the one in their file name
//! with a `400 Bad Request`.

use http::{Method, StatusCode};
use sub60_endpoints_common::{Created, NoContent};

use crate::{NarBody, nar_head};

pub type Path = nar_head::Path;

#[derive(Debug, Copy, Clone)]
pub struct Endpoint;

#[derive(
    Debug, Copy, Clone, PartialEq, Eq, sub60_endpoints_common::Response,
)]
pub enum Response {
    /// The NAR was stored.
    Created(Created),

    /// The NAR already existed, and was left as is.
    AlreadyExists(NoContent),
}

impl sub60_endpoints_common::Endpoint for Endpoint {
    const METHOD: Method = Method::PUT;

    type Path = Path;
    type Request = NarBody;
    type Response = Response;
    type Error = StatusCode;
}
