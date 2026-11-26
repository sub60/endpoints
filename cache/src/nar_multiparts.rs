//! The `POST /{account}/{cache}/nar-multiparts` endpoint.
//!
//! Starts a multipart upload of a NAR.

use core::num::{NonZeroU32, NonZeroU64};

use http::{Method, StatusCode};
use smallvec::SmallVec;
use sub60_endpoints_common::body::Json;
use sub60_endpoints_common::response::Ok;
use sub60_endpoints_common::{PathParams, Timestamp};

use crate::CachePath;

#[derive(Debug, Copy, Clone)]
pub struct Endpoint;

/// The path parameters of the endpoint's requests.
#[derive(Debug, Clone, PartialEq, Eq, derive_more::AsRef, PathParams)]
#[route(CachePath + "/nar-multiparts")]
pub struct Path {
    /// The cache the NAR is uploaded to.
    #[as_ref]
    pub cache: CachePath,
}

/// The ID of a multipart upload.
pub type UploadId = String;

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
)]
pub struct RequestBody {
    /// The size of the NAR being uploaded, in bytes.
    pub file_size: NonZeroU64,
}

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
)]
pub struct ResponseBody {
    /// TODO: docs.
    pub upload_id: UploadId,

    /// TODO: docs.
    pub part_size: NonZeroU32,

    /// The URLs to upload the first parts of the NAR to, in order, starting
    /// from part 1. There are at most 4 of them, and fewer if the NAR has
    /// fewer parts.
    pub parts_urls: SmallVec<[url::Url; 4]>,

    /// The time the upload URLs stop being valid.
    pub expiration_ts: Timestamp,
}

impl sub60_endpoints_common::Endpoint for Endpoint {
    const METHOD: Method = Method::POST;

    type Path = Path;
    type Request = Json<RequestBody>;
    type Response = Ok<Json<ResponseBody>>;
    type Error = StatusCode;
}
