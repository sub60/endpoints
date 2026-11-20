use core::ops::Range;

use sub60_endpoints_common::Timestamp;

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
pub struct Billing {
    /// The plan the account is on while it has no subscription.
    pub base_plan: BasePlan,
    /// The account's subscription, or `None` if it's on the base plan.
    pub subscription: Option<Subscription>,
    pub payment_method: Option<PaymentMethod>,
    pub billing_email: String,
}

#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    Hash,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
)]
#[serde(rename_all = "snake_case")]
#[expect(missing_docs)]
pub enum BasePlan {
    Free,
    Insider,
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
#[serde(tag = "status", rename_all = "snake_case")]
#[expect(missing_docs)]
pub enum Subscription {
    Active {
        plan: PaidPlan,
        current_period: Range<Timestamp>,
        /// The invoice the account will be charged next, or `None` if the
        /// subscription ends with the current period, after which the account
        /// is back on its base plan.
        next_invoice: Option<NextInvoice>,
    },
    PastDue {
        plan: PaidPlan,
        unpaid_period: Range<Timestamp>,
        amount_due: Money,
    },
}

#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    Hash,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
)]
#[serde(rename_all = "snake_case")]
#[expect(missing_docs)]
pub enum PaidPlan {
    Pro,
    Enterprise,
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
#[expect(missing_docs)]
pub struct NextInvoice {
    pub amount: Money,
    pub charged_at: Timestamp,
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
#[expect(missing_docs)]
pub enum Money {
    Usd { cents: u64 },
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
#[expect(missing_docs)]
pub enum PaymentMethod {
    Card {
        /// The card's network, e.g. `visa` or `mastercard`.
        brand: String,
        /// The last four digits of the card's number.
        last4: String,
        expiration_month: u8,
        expiration_year: u16,
    },
}
