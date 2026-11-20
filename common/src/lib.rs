//! TODO: docs.

#![allow(incomplete_features)]

extern crate alloc;

mod account_name;
mod body;
mod cache_name;
mod endpoint;
mod path_params;
mod permission;
mod request;
mod request_error;
mod response;
mod timestamp;

pub use account_name::{AccountName, AccountNameFromStrError};
pub use body::{Body, Json, JsonError};
pub use cache_name::{CacheName, CacheNameFromStrError};
pub use endpoint::{Endpoint, EndpointVisitor};
#[doc(hidden)]
pub use path_params::__private;
#[cfg(feature = "axum")]
pub use path_params::PathParamsRejection;
pub use path_params::{PathParams, PathParamsError};
pub use permission::{Permission, Permissions};
pub use request::{Request, WithQuery};
pub use request_error::RequestError;
pub use response::{
    Accepted,
    Created,
    NoContent,
    NotFound,
    Ok,
    Response,
    ResponseVisitor,
    WithStatus,
};
pub use timestamp::Timestamp;
