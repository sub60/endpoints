//! The `POST /{account}/{cache}/nar-multiparts` endpoint.
//!
//! Starts a multipart upload of a NAR.

use core::num::{NonZeroU32, NonZeroU64};
use core::time::Duration;

use http::{Method, StatusCode};
use smallvec::SmallVec;
use sub60_endpoints_common::{Json, Ok, PathParams};

use crate::CachePath;

#[derive(Debug, Copy, Clone)]
#[expect(missing_docs)]
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
#[expect(missing_docs)]
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
#[expect(missing_docs)]
pub struct ResponseBody {
    /// TODO: docs.
    pub upload_id: UploadId,

    /// TODO: docs.
    pub part_size: NonZeroU32,

    /// The URLs to upload the first parts of the NAR to, in order, starting
    /// from part 1. There are at most 4 of them, and fewer if the NAR has
    /// fewer parts.
    pub parts_urls: SmallVec<[url::Url; 4]>,

    /// The expiration timestamp of this batch of upload URLs, expressed as the
    /// elapsed time since the UNIX epoch.
    #[serde(with = "sub60_endpoints_common::serde_utils::secs")]
    #[schemars(with = "u64")]
    pub expiration_ts: Duration,
}

impl sub60_endpoints_common::Endpoint for Endpoint {
    const METHOD: Method = Method::POST;

    type Error = StatusCode;
    type Path = Path;
    type Request = Json<RequestBody>;
    type Response = Ok<Json<ResponseBody>>;
}
