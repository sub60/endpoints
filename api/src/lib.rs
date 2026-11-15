//! The endpoints of `api.sub60.dev`.

pub mod auth_github_actions_token;
pub mod storage_size;

pub use sub60_endpoints_common::{
    AccountName,
    AccountNameFromStrError,
    CacheName,
    CacheNameFromStrError,
    Endpoint,
    EndpointError,
    EndpointVisitor,
    JsonEndpoint,
    PathParams,
    PathParamsError,
    RequestError,
};

/// The prefix of the paths of version 1 of the API.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PathParams)]
#[route("/v1")]
pub struct V1Path;

/// Calls the visitor on each of the API's endpoints.
pub fn visit_endpoints<V: EndpointVisitor>(visitor: &mut V) {
    visitor.visit::<auth_github_actions_token::Endpoint>();
    visitor.visit::<storage_size::Endpoint>();
}
