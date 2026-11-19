//! The `POST /{account}/{cache}/nar-multiparts/{upload_id}/complete` endpoint.
//!
//! Completes a multipart upload.

use http::{Method, StatusCode};
use nix_types::Nix32Digest;
use smallvec::SmallVec;

use crate::nar_multiparts::UploadId;
use crate::{CachePath, Etag, PathParams};

#[derive(Debug, Copy, Clone)]
#[expect(missing_docs)]
pub struct Endpoint;

/// The path parameters of the endpoint's requests.
#[derive(Debug, Clone, PartialEq, Eq, derive_more::AsRef, PathParams)]
#[route(CachePath + "/nar-multiparts/{upload_id}/complete")]
pub struct Path<Id = UploadId> {
    /// The cache the NAR is uploaded to.
    #[as_ref]
    pub cache: CachePath,

    /// The ID of the upload.
    pub upload_id: Id,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[expect(missing_docs)]
pub enum Response {
    /// The NAR was stored.
    Created(ResponseBody),

    /// The NAR already existed, e.g. because another upload of it completed
    /// first, and this upload was discarded.
    AlreadyExists(ResponseBody),
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[expect(missing_docs)]
pub struct RequestBody {
    /// The list of [`Etag`]s returned by the `UploadPart` requests to the
    /// cache's object storage.
    ///
    /// The [`Etag`]s must be in order, i.e. the first one must be the one
    /// returned by uploading part 1, the second one by uploading part 2, etc.
    pub etags: SmallVec<[Etag; 8]>,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[expect(missing_docs)]
pub struct ResponseBody {
    /// The SHA-256 hash of the uploaded NAR.
    pub file_hash: Nix32Digest<32>,
}

impl crate::Endpoint for Endpoint {
    const METHOD: Method = Method::POST;

    type Error = StatusCode;
    type Path = Path;
    type Request = RequestBody;
    type Response = Response;
}

impl Response {
    /// Returns the response's body.
    #[inline]
    pub fn body(&self) -> &ResponseBody {
        match self {
            Self::Created(body) | Self::AlreadyExists(body) => body,
        }
    }

    /// Returns the response's status code.
    #[inline]
    pub fn status_code(&self) -> StatusCode {
        match self {
            Self::Created(_) => StatusCode::CREATED,
            Self::AlreadyExists(_) => StatusCode::OK,
        }
    }
}
