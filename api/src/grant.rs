use sub60_endpoints_common::{Permission, Timestamp};

use crate::{AccountId, CacheId, GrantId, MachineTokenId, UserId};

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
    pub created_by: UserId,
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
    User { id: UserId },
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
        repository: Option<String>,
        /// The Git ref (e.g. `refs/heads/main`). `None` matches any ref.
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
