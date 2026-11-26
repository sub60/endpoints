//! The `GET /{account}/{cache}/nar/{nar_file_name}` endpoint.
//!
//! Gets a NAR.
//!
//! Clients only download NARs from here if the cache's NARInfos point to it,
//! which they do with a `URL` of `nar/{nar_file_name}`.

use http::{Method, StatusCode};
use nix_types::NarFileName;
use sub60_endpoints_common::PathParams;
use sub60_endpoints_common::response::{NotFound, Ok};

use crate::{CachePath, NarBody};

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
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    sub60_endpoints_common::response::Response,
)]
pub enum Response {
    /// The NAR exists, and it's in the response's body.
    Found(Ok<NarBody>),

    /// The NAR doesn't exist.
    NotFound(NotFound),
}

impl sub60_endpoints_common::Endpoint for Endpoint {
    const METHOD: Method = Method::GET;

    type Path = Path;
    type Request = ();
    type Response = Response;
    type Error = StatusCode;
}
