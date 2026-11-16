//! The `POST /{account}/{cache}/nar-multiparts` endpoint.

use core::num::{NonZeroU32, NonZeroU64};
use core::time::Duration;

use http::Method;
use smallvec::SmallVec;

use crate::{CachePath, PathParams};

/// Starts a multipart upload of a NAR.
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

/// The body of a `/nar-multiparts` request.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct RequestBody {
    /// The size of the NAR being uploaded, in bytes.
    pub file_size: NonZeroU64,
}

/// The body of a `/nar-multiparts` response.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
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
    pub expiration_ts: Duration,
}

impl crate::Endpoint for Endpoint {
    const METHOD: Method = Method::POST;

    type Path = Path;
    type Request = RequestBody;
    type Response = ResponseBody;
}
