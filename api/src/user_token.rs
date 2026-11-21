use sub60_endpoints_common::{Permission, Timestamp};

use crate::UserTokenId;

/// A token acting as its user, with at most its `max_permissions`.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
)]
pub struct UserToken {
    pub id: UserTokenId,
    pub description: String,
    /// The most the token can do, whatever its user's grants.
    pub max_permissions: Vec<Permission>,
    pub created_at: Timestamp,
    /// The time the token stops being valid, or `None` if it never expires.
    pub expires_at: Option<Timestamp>,
    /// A recent time the token authenticated a request, or `None` if it never
    /// has.
    pub recently_used_at: Option<Timestamp>,
}
