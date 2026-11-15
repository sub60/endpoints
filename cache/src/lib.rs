//! The endpoints of `cache.sub60.dev`.

#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

extern crate alloc;

mod etag;
pub mod missing_narinfos;
pub mod nar_get;
pub mod nar_head;
pub mod nar_multiparts;
pub mod nar_multiparts_complete;
pub mod nar_multiparts_parts;
pub mod nar_put;
pub mod narinfo_get;
pub mod narinfo_head;
pub mod narinfo_put;
pub mod nix_cache_info;

pub use etag::{Etag, EtagFromStrError};
#[cfg(feature = "axum")]
pub use sub60_endpoints_common::PathParamsRejection;
pub use sub60_endpoints_common::{
    AccountName,
    AccountNameFromStrError,
    CacheName,
    CacheNameFromStrError,
    Endpoint,
    PathParams,
    PathParamsError,
};

/// The path parameters identifying a cache, which the paths of all the cache's
/// endpoints start with.
#[derive(Debug, Clone, PartialEq, Eq, PathParams)]
#[route("/{account}/{cache}")]
pub struct CachePath {
    /// The name of the account owning the cache.
    pub account: AccountName,

    /// The cache's name.
    pub cache: CacheName,
}
