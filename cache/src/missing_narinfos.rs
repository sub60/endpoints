//! The `POST /{account}/{cache}/missing-narinfos` endpoint.
//!
//! Returns which of the given NARInfos the cache doesn't have.
//!
//! It answers what a `HEAD` request for each of the NARInfos would, in a single
//! request.

use core::str::FromStr;
use core::{fmt, ops, str};
use std::io::Write as _;

use bytes::BufMut;
use http::{Method, StatusCode};
use nix_types::{StoreHash, StoreHashFromStrError};
use sub60_endpoints_common::PathParams;
use sub60_endpoints_common::body::{Body, Decode, Encode};
use sub60_endpoints_common::response::Ok;

use crate::CachePath;

/// The maximum number of store hashes in a request.
pub const MAX_STORE_HASHES: usize = 10_000;

/// The store hashes of the NARInfos to look up, at most [`MAX_STORE_HASHES`] of
/// them.
pub type RequestBody = StoreHashes;

/// The store hashes of the requested NARInfos the cache doesn't have, in the
/// order they were requested.
pub type ResponseBody = StoreHashes;

#[derive(Debug, Copy, Clone)]
pub struct Endpoint;

/// The path parameters of the endpoint's requests.
#[derive(Debug, Clone, PartialEq, Eq, derive_more::AsRef, PathParams)]
#[route(CachePath + "/missing-narinfos")]
pub struct Path {
    /// The cache the NARInfos are in.
    #[as_ref]
    pub cache: CachePath,
}

/// A list of store hashes, written one per line.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct StoreHashes(pub Vec<StoreHash>);

/// The type of error that can occur when parsing a string into [`StoreHashes`].
#[derive(Debug, PartialEq, Eq, derive_more::Display, cauchy::Error)]
#[display("line {line}: {error}")]
pub struct StoreHashesFromStrError {
    /// The line with the invalid store hash, starting from 1.
    pub line: usize,

    /// Why the line isn't a valid store hash.
    pub error: StoreHashFromStrError,
}

/// The type of error that can occur when decoding bytes into [`StoreHashes`].
#[derive(Debug, PartialEq, Eq, derive_more::Display, cauchy::Error)]
pub enum StoreHashesDecodeError {
    /// The bytes aren't UTF-8.
    #[display("the store hashes aren't UTF-8")]
    Utf8(core::str::Utf8Error),

    /// The bytes aren't store hashes written one per line.
    #[display("{_0}")]
    FromStr(StoreHashesFromStrError),
}

impl sub60_endpoints_common::Endpoint for Endpoint {
    const METHOD: Method = Method::POST;

    type Path = Path;
    type Request = RequestBody;
    type Response = Ok<ResponseBody>;
    type Error = StatusCode;
}

impl fmt::Display for StoreHashes {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.iter().try_for_each(|store_hash| writeln!(f, "{store_hash}"))
    }
}

impl ops::Deref for StoreHashes {
    type Target = [StoreHash];

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for StoreHashes {
    type Err = StoreHashesFromStrError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.lines()
            .enumerate()
            .map(|(idx, line)| {
                line.parse().map_err(|error| StoreHashesFromStrError {
                    line: idx + 1,
                    error,
                })
            })
            .collect::<Result<_, _>>()
            .map(Self)
    }
}

impl Body for StoreHashes {
    const MEDIA_TYPE: Option<&'static str> = Some("text/plain; charset=utf-8");

    #[inline]
    fn schema(_: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schemars::json_schema!({ "type": "string" })
    }
}

impl Encode for StoreHashes {
    #[inline]
    fn encode(&self, buf: &mut impl BufMut) {
        write!(buf.writer(), "{self}")
            .expect("the buffer has room for the body");
    }
}

impl Decode for StoreHashes {
    type Error = StoreHashesDecodeError;

    #[inline]
    fn decode(bytes: &[u8]) -> Result<Self, Self::Error> {
        str::from_utf8(bytes)
            .map_err(StoreHashesDecodeError::Utf8)?
            .parse()
            .map_err(StoreHashesDecodeError::FromStr)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_hashes_round_trip() {
        let store_hashes = StoreHashes(vec![
            "0123456789abcdfghijklmnpqrsvwxyz".parse().unwrap(),
            "zyxwvsrqpnmlkjihgfdcba9876543210".parse().unwrap(),
        ]);

        let mut buf = Vec::new();
        store_hashes.encode(&mut buf);
        assert_eq!(StoreHashes::decode(&buf).unwrap(), store_hashes);
    }
}
