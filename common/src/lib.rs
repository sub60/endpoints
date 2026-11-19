//! TODO: docs.

#![allow(incomplete_features)]

extern crate alloc;

mod account_name;
mod cache_name;
mod endpoint;
mod path_params;
mod request_error;
#[doc(hidden)]
pub mod serde_utils;

pub use account_name::{AccountName, AccountNameFromStrError};
pub use cache_name::{CacheName, CacheNameFromStrError};
pub use endpoint::{
    Endpoint,
    EndpointVisitor,
    JsonEndpoint,
    JsonEndpointVisitor,
};
#[doc(hidden)]
pub use path_params::__private;
#[cfg(feature = "axum")]
pub use path_params::PathParamsRejection;
pub use path_params::{PathParams, PathParamsError};
pub use request_error::RequestError;
