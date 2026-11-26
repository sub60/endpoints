//! The `POST /{account}/{cache}/nar-multiparts/{upload_id}/parts` endpoint.
//!
//! Gets the URLs to upload some parts of a multipart upload to.

use core::num::NonZeroU16;

use http::{Method, StatusCode};
use smallvec::SmallVec;
use sub60_endpoints_common::body::Json;
use sub60_endpoints_common::response::Ok;
use sub60_endpoints_common::{PathParams, Timestamp};
use url::Url;

use crate::CachePath;
use crate::nar_multiparts::UploadId;

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
    /// The list of part numbers to request upload URLs for. The numbers can be
    /// in any order, but there shouldn't be any duplicates.
    pub part_numbers: SmallVec<[NonZeroU16; 8]>,
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
    /// The pre-signed URLs whose order matches the one of the
    /// [part numbers](RequestBody::part_numbers) in the
    /// [`RequestBody`], i.e. the first URL is for the first part number,
    /// the second URL is for the part number item, and so on.
    pub urls: SmallVec<[Url; 8]>,

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
