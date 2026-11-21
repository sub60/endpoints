//! The endpoints of `api.sub60.dev`.

extern crate alloc;

mod account;
pub mod account_billing_get;
pub mod account_billing_invoice_list;
pub mod account_billing_invoice_pay;
pub mod account_billing_payment_method_delete;
pub mod account_billing_payment_method_update;
pub mod account_billing_plan_preview;
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
mod cache;
pub mod cache_create;
pub mod cache_get;
pub mod cache_list;
pub mod cache_update;
mod expiration;
mod id;
mod session;
pub mod storage_size;
pub mod user_delete;
pub mod user_get;
pub mod user_session_current_delete;
pub mod user_session_delete;
pub mod user_session_list;
mod user_token;
pub mod user_token_create;
pub mod user_token_delete;
pub mod user_token_list;
pub mod user_token_update;

pub use account::{Account, AccountKind, AccountSummary};
pub use billing::{
    BasePlan,
    Billing,
    ChargeConfirmation,
    Invoice,
    InvoiceReason,
    InvoiceStatus,
    Money,
    NextInvoice,
    PaidPlan,
    PaymentMethod,
    Subscription,
};
pub use cache::{Cache, InvalidPublicPermissions, PublicKey};
pub use expiration::Expiration;
pub use id::{
    AccountId,
    CacheId,
    IdFromStrError,
    InvoiceId,
    SessionId,
    UserId,
    UserTokenId,
};
pub use session::Session;
pub use user_token::UserToken;

/// Calls the visitor on each of the API's endpoints.
pub fn visit_endpoints<V: sub60_endpoints_common::EndpointVisitor>(
    visitor: &mut V,
) {
    visitor.visit::<account_billing_get::Endpoint>();
    visitor.visit::<account_billing_invoice_list::Endpoint>();
    visitor.visit::<account_billing_invoice_pay::Endpoint>();
    visitor.visit::<account_billing_payment_method_delete::Endpoint>();
    visitor.visit::<account_billing_payment_method_update::Endpoint>();
    visitor.visit::<account_billing_plan_preview::Endpoint>();
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
    visitor.visit::<cache_create::Endpoint>();
    visitor.visit::<cache_get::Endpoint>();
    visitor.visit::<cache_list::Endpoint>();
    visitor.visit::<cache_update::Endpoint>();
    visitor.visit::<storage_size::Endpoint>();
    visitor.visit::<user_delete::Endpoint>();
    visitor.visit::<user_get::Endpoint>();
    visitor.visit::<user_session_current_delete::Endpoint>();
    visitor.visit::<user_session_delete::Endpoint>();
    visitor.visit::<user_session_list::Endpoint>();
    visitor.visit::<user_token_create::Endpoint>();
    visitor.visit::<user_token_delete::Endpoint>();
    visitor.visit::<user_token_list::Endpoint>();
    visitor.visit::<user_token_update::Endpoint>();
}
