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
    EndpointVisitor,
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

/// Calls the visitor on each of the cache's endpoints.
pub fn visit_endpoints<V: EndpointVisitor>(visitor: &mut V) {
    visitor.visit::<missing_narinfos::Endpoint>();
    visitor.visit::<nar_get::Endpoint>();
    visitor.visit::<nar_head::Endpoint>();
    visitor.visit::<nar_multiparts::Endpoint>();
    visitor.visit::<nar_multiparts_complete::Endpoint>();
    visitor.visit::<nar_multiparts_parts::Endpoint>();
    visitor.visit::<nar_put::Endpoint>();
    visitor.visit::<narinfo_get::Endpoint>();
    visitor.visit::<narinfo_head::Endpoint>();
    visitor.visit::<narinfo_put::Endpoint>();
    visitor.visit::<nix_cache_info::Endpoint>();
}
