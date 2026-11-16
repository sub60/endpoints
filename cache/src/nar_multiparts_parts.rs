//! The `POST /{account}/{cache}/nar-multiparts/{upload_id}/parts` endpoint.

use core::num::NonZeroU16;
use core::time::Duration;

use http::Method;
use smallvec::SmallVec;
use url::Url;

use crate::nar_multiparts::UploadId;
use crate::{CachePath, PathParams};

/// Gets the URLs to upload some parts of a multipart upload to.
#[derive(Debug, Copy, Clone)]
pub struct Endpoint;

/// The path parameters of the endpoint's requests.
#[derive(Debug, Clone, PartialEq, Eq, derive_more::AsRef, PathParams)]
#[route(CachePath + "/nar-multiparts/{upload_id}/parts")]
pub struct Path<Id = UploadId> {
    /// The cache the NAR is uploaded to.
    #[as_ref]
    pub cache: CachePath,

    /// The ID of the upload.
    pub upload_id: Id,
}

/// The body of a `/nar-multiparts/{uploadId}/parts` request.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct RequestBody {
    /// The list of part numbers to request upload URLs for. The numbers can be
    /// in any order, but there shouldn't be any duplicates.
    pub part_numbers: SmallVec<[NonZeroU16; 8]>,
}

/// The body of a `/nar-multiparts/{uploadId}/parts` response.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ResponseBody {
    /// The pre-signed URLs whose order matches the one of the
    /// [part numbers](RequestBody::part_numbers) in the
    /// [`RequestBody`], i.e. the first URL is for the first part number,
    /// the second URL is for the part number item, and so on.
    pub urls: SmallVec<[Url; 8]>,

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
