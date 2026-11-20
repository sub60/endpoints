//! The endpoints of `api.sub60.dev`.

extern crate alloc;

mod account;
pub mod account_billing_get;
pub mod account_billing_payment_method_delete;
pub mod account_billing_payment_method_update;
pub mod account_billing_plan_update;
pub mod account_billing_update;
pub mod account_create;
pub mod account_delete;
pub mod account_get;
pub mod account_list;
pub mod account_name_get;
pub mod auth_github_actions_token;
pub mod auth_github_callback;
pub mod auth_github_login;
mod billing;
mod id;
pub mod storage_size;
pub mod user_delete;
pub mod user_get;

pub use account::{Account, AccountKind, AccountSummary};
pub use billing::{
    BasePlan,
    Billing,
    ChargeConfirmation,
    Money,
    NextInvoice,
    PaidPlan,
    PaymentMethod,
    Subscription,
};
pub use id::{AccountId, CacheId, IdFromStrError, UserId};

/// Calls the visitor on each of the API's endpoints.
pub fn visit_endpoints<V: sub60_endpoints_common::EndpointVisitor>(
    visitor: &mut V,
) {
    visitor.visit::<account_billing_get::Endpoint>();
    visitor.visit::<account_billing_payment_method_delete::Endpoint>();
    visitor.visit::<account_billing_payment_method_update::Endpoint>();
    visitor.visit::<account_billing_plan_update::Endpoint>();
    visitor.visit::<account_billing_update::Endpoint>();
    visitor.visit::<account_create::Endpoint>();
    visitor.visit::<account_delete::Endpoint>();
    visitor.visit::<account_get::Endpoint>();
    visitor.visit::<account_list::Endpoint>();
    visitor.visit::<account_name_get::Endpoint>();
    visitor.visit::<auth_github_actions_token::Endpoint>();
    visitor.visit::<auth_github_callback::Endpoint>();
    visitor.visit::<auth_github_login::Endpoint>();
    visitor.visit::<storage_size::Endpoint>();
    visitor.visit::<user_delete::Endpoint>();
    visitor.visit::<user_get::Endpoint>();
}
