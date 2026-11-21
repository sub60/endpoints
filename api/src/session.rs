use sub60_endpoints_common::Timestamp;

use crate::SessionId;

/// A user's login in a browser.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
)]
pub struct Session {
    pub id: SessionId,
    pub created_at: Timestamp,
    pub expires_at: Timestamp,
    /// A recent time the session authenticated a request, or `None` if it
    /// never has.
    pub recently_used_at: Option<Timestamp>,
    /// Whether the session is the one the request is authenticated with.
    pub current: bool,
}
