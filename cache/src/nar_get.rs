//! The `GET /{account}/{cache}/nar/{nar_file_name}` endpoint.
//!
//! Gets a NAR.
//!
//! Clients only download NARs from here if the cache's NARInfos point to it,
//! which they do with a `URL` of `nar/{nar_file_name}`.

use http::{Method, StatusCode};
use nix_types::NarFileName;

use crate::{CachePath, PathParams};

#[derive(Debug, Copy, Clone)]
#[expect(missing_docs)]
pub struct Endpoint;

/// The path parameters of the endpoint's requests.
#[derive(Debug, Clone, PartialEq, Eq, derive_more::AsRef, PathParams)]
#[route(CachePath + "/nar/{nar_file_name}")]
pub struct Path {
    /// The cache the NAR is in.
    #[as_ref]
    pub cache: CachePath,

    /// The NAR's file name.
    pub nar_file_name: NarFileName,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[expect(missing_docs)]
pub enum Response {
    /// The NAR exists, and it's in the response's body.
    Found,

    /// The NAR doesn't exist.
    NotFound,
}

impl crate::Endpoint for Endpoint {
    const METHOD: Method = Method::GET;

    type Error = StatusCode;
    type Path = Path;
    type Request = ();
    type Response = Response;
}

impl Response {
    /// Returns the response's status code.
    #[inline]
    pub fn status_code(&self) -> StatusCode {
        match self {
            Self::Found => StatusCode::OK,
            Self::NotFound => StatusCode::NOT_FOUND,
        }
    }
}
