use http::StatusCode;
use sub60_endpoints_common::{CacheName, Permission, Timestamp};
use url::Url;

use crate::{AccountSummary, CacheId};

/// A public key as written in `trusted-public-keys`.
pub type PublicKey = String;

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
)]
pub struct Cache {
    pub id: CacheId,
    pub account: AccountSummary,
    pub name: CacheName,
    pub substituter_url: Url,
    /// The public half of the key the cache signs its NARInfos with.
    pub public_key: PublicKey,
    /// The public halves of the cache's previous keys, each still signing its
    /// NARInfos until the given timestamp.
    pub retiring_public_keys: Vec<(PublicKey, Timestamp)>,
    /// The public keys verifying the NARInfos signed before they were pushed,
    /// with the time each was added.
    pub client_public_keys: Vec<(PublicKey, Timestamp)>,
    pub priority: u32,
    pub store_dir: String,
    /// What anyone can do on the cache, with or without a token.
    pub public_permissions: Vec<Permission>,
    /// The permissions the request's principal has on the cache.
    pub my_permissions: Vec<Permission>,
    pub created_at: Timestamp,
    /// The number of store paths in the cache, or `None` if the request's
    /// principal isn't a member of the cache's account.
    pub path_count: Option<u64>,
    /// The total byte size of the cache's NARInfos and of the NARs they
    /// reference, or `None` if the request's principal isn't a member of the
    /// cache's account.
    pub size_bytes: Option<u64>,
}

/// A cache request's `public_permissions` lists permissions other than
/// [`PullFromCache`](Permission::PullFromCache) and
/// [`ListCachePaths`](Permission::ListCachePaths).
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    derive_more::Display,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
)]
#[display("only PullFromCache and ListCachePaths can be public permissions")]
pub struct InvalidPublicPermissions {
    pub invalid: Vec<Permission>,
}

impl sub60_endpoints_common::Error for InvalidPublicPermissions {
    const STATUS: StatusCode = StatusCode::BAD_REQUEST;
}
