use sub60_endpoints_common::AccountName;

use crate::AccountId;

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
