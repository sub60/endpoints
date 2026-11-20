//! The `POST /auth/github/callback` endpoint.
//!
//! This endpoint is used to finish logging in with GitHub, once GitHub has
//! redirected the browser back with a `code`.

use http::Method;
use sub60_endpoints_common::{
    Json,
    Ok,
    PathParams,
    Timestamp,
    WithMessage,
    error,
};

#[derive(Debug, Copy, Clone)]
#[expect(missing_docs)]
pub struct Endpoint;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PathParams)]
#[route("/auth/github/callback")]
#[expect(missing_docs)]
pub struct Path;

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
)]
#[schemars(rename = "GitHubCallbackRequest")]
#[expect(missing_docs)]
pub struct Request {
    /// The `code` query parameter of GitHub's redirect.
    pub code: String,

    /// The [`code_verifier`](crate::auth_github_login::Response::code_verifier)
    /// returned by the login request that started the login.
    pub code_verifier: String,
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
#[schemars(rename = "GitHubCallbackResponse")]
#[expect(missing_docs)]
pub struct Response {
    /// The token the client will use to authenticate the user's requests.
    pub token: String,

    /// The time the token stops being valid.
    pub expires_at: Timestamp,

    /// What the login did.
    pub outcome: LoginOutcome,
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
pub enum LoginOutcome {
    /// The user already had a personal account.
    LoggedIn,

    /// The user's personal account was just created, named after their GitHub
    /// handle.
    SignedUp,

    /// The user's personal account couldn't be created because their GitHub
    /// handle is already another account's name, so they have to pick a name
    /// for it.
    AccountNameTaken,
}

#[derive(
    Debug,
    derive_more::Display,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    sub60_endpoints_common::Response,
)]
#[serde(tag = "code", rename_all = "snake_case")]
#[expect(missing_docs)]
pub enum Error {
    /// GitHub refused to exchange the code, e.g. because it expired, was
    /// already used, or doesn't match the code verifier.
    #[display("GitHub refused to exchange the code")]
    #[status(BAD_REQUEST)]
    InvalidCode,
    /// The GitHub user's primary email address isn't verified.
    #[display("the GitHub user's primary email address isn't verified")]
    #[status(FORBIDDEN)]
    NoVerifiedEmail,
    /// GitHub couldn't be reached, or answered with an unexpected response.
    #[display("the request to GitHub failed")]
    #[serde(rename = "github_unavailable")]
    #[status(BAD_GATEWAY)]
    GitHubUnavailable,
    Internal(error::Internal),
}

impl sub60_endpoints_common::Endpoint for Endpoint {
    const METHOD: Method = Method::POST;

    type Path = Path;
    type Request = Json<Request>;
    type Response = Ok<Json<Response>>;
    type Error = Json<WithMessage<Error>>;
}
