use http::StatusCode;
use sub60_endpoints_common::{Permission, Timestamp};

use crate::{AccountId, CacheId, GrantId, MachineTokenId};

/// A principal's permissions on an account or a cache.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
)]
pub struct Grant {
    pub id: GrantId,
    pub principal: Principal,
    pub target: Target,
    pub permissions: Vec<Permission>,
    pub created_by: AccountId,
    pub created_at: Timestamp,
}

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Principal {
    User { id: AccountId },
    MachineToken { id: MachineTokenId },
    CiJob(CiJobs),
}

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
)]
#[serde(tag = "host", rename_all = "snake_case")]
pub enum CiJobs {
    GithubActions {
        owner: String,
        /// The repo's name without its owner's. `None` matches all of the
        /// owner's repos.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        repository: Option<String>,
        /// The Git ref (e.g. `refs/heads/main`). `None` matches any ref.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        r#ref: Option<String>,
    },
}

#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Target {
    Account { id: AccountId },
    Cache { id: CacheId },
}

/// A grant on a cache lists permissions that only exist on accounts.
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
#[display(
    "the grant is on a cache, but some of its permissions only exist on \
     accounts"
)]
pub struct InvalidGrantPermissions {
    pub invalid: Vec<Permission>,
}

impl sub60_endpoints_common::Error for InvalidGrantPermissions {
    const STATUS: StatusCode = StatusCode::BAD_REQUEST;
}
