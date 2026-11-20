//! The `HEAD /{account}/{cache}/nar/{nar_file_name}` endpoint.
//!
//! Checks whether a NAR exists.

use http::{Method, StatusCode};
use nix_types::NarFileName;
use sub60_endpoints_common::{NotFound, Ok, PathParams};

use crate::CachePath;

#[derive(Debug, Copy, Clone)]
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

#[derive(
    Debug, Copy, Clone, PartialEq, Eq, sub60_endpoints_common::Response,
)]
pub enum Response {
    /// The NAR exists.
    Exists(Ok),

    /// The NAR doesn't exist.
    NotFound(NotFound),
}

impl sub60_endpoints_common::Endpoint for Endpoint {
    const METHOD: Method = Method::HEAD;

    type Path = Path;
    type Request = ();
    type Response = Response;
    type Error = StatusCode;
}
