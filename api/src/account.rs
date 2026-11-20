use core::time::Duration;

use sub60_endpoints_common::{AccountName, Permission};

use crate::AccountId;

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
)]
#[expect(missing_docs)]
pub struct Account {
    pub id: AccountId,
    pub name: AccountName,
    pub kind: AccountKind,
    /// The time the account was created, as the number of seconds since the
    /// UNIX epoch.
    #[serde(with = "sub60_endpoints_common::serde_utils::secs")]
    #[schemars(with = "u64")]
    pub created_at: Duration,
    /// The permissions the request's principal has on the account.
    pub my_permissions: Vec<Permission>,
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
#[serde(rename_all = "snake_case")]
#[expect(missing_docs)]
pub enum AccountKind {
    Personal,
    Org,
}

/// TODO: docs.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
)]
pub struct AccountSummary {
    #[expect(missing_docs)]
    pub id: AccountId,
    #[expect(missing_docs)]
    pub name: AccountName,
}
