use sub60_endpoints_common::{AccountName, Permission, Timestamp};

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
pub struct Account {
    pub id: AccountId,
    pub name: AccountName,
    pub kind: AccountKind,
    /// The time the account was created.
    pub created_at: Timestamp,
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
    pub id: AccountId,
    pub name: AccountName,
}
