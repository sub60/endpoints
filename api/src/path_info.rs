use core::num::NonZeroU64;

use nix_types::{
    CompressionAlgorithm,
    ContentAddress,
    Nix32Digest,
    StoreBasename,
    StorePath,
};
use sub60_endpoints_common::Timestamp;

/// Infos about a store path in a cache.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
)]
pub struct PathInfo {
    pub store_path: StorePath<String>,
    pub nar_hash: Nix32Digest<32>,
    pub nar_size: NonZeroU64,
    pub references: Vec<StoreBasename>,
    pub deriver: Option<StoreBasename>,
    pub content_address: Option<ContentAddress>,
    pub compression: CompressionAlgorithm,
    pub file_hash: Nix32Digest<32>,
    pub file_size: NonZeroU64,
    pub pushed_at: Timestamp,
}
