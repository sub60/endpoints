//! TODO: docs.

#![allow(incomplete_features)]

extern crate alloc;

mod account_name;
#[cfg(feature = "axum")]
pub mod axum;
pub mod body;
mod bounded_vec;
mod cache_name;
mod cursor;
mod endpoint;
pub mod error;
mod path_params;
mod permission;
pub mod request;
pub mod response;
mod timestamp;

pub use account_name::{AccountName, AccountNameFromStrError};
pub use bounded_vec::{BoundedVec, BoundedVecTooLong};
pub use cache_name::{CacheName, CacheNameFromStrError};
pub use cursor::Cursor;
pub use endpoint::{Endpoint, EndpointVisitor};
pub use error::Error;
#[doc(hidden)]
pub use path_params::__private;
pub use path_params::{PathParams, PathParamsError};
pub use permission::{Permission, Permissions};
pub use timestamp::Timestamp;
