//! The endpoints of `api.sub60.dev`.

extern crate alloc;

mod account;
pub mod auth_github_actions_token;
pub mod auth_github_callback;
pub mod auth_github_login;
mod id;
pub mod storage_size;
pub mod user_delete;
pub mod user_get;

pub use account::AccountSummary;
pub use id::{AccountId, CacheId, IdFromStrError, UserId};

/// Calls the visitor on each of the API's endpoints.
pub fn visit_endpoints<V: sub60_endpoints_common::EndpointVisitor>(
    visitor: &mut V,
) {
    visitor.visit::<auth_github_actions_token::Endpoint>();
    visitor.visit::<auth_github_callback::Endpoint>();
    visitor.visit::<auth_github_login::Endpoint>();
    visitor.visit::<storage_size::Endpoint>();
    visitor.visit::<user_delete::Endpoint>();
    visitor.visit::<user_get::Endpoint>();
}
