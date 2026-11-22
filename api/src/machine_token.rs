use sub60_endpoints_common::Timestamp;

use crate::{AccountId, Expiration, MachineTokenId};

/// A token authenticating a machine, like a build server.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
)]
pub struct MachineToken {
    pub id: MachineTokenId,
    pub description: String,
    pub created_by: AccountId,
    pub created_at: Timestamp,
    pub expires_at: Expiration,
    pub recently_used_at: Option<Timestamp>,
}
