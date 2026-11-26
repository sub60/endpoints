//! The `POST /auth/github/login` endpoint.
//!
//! This endpoint is used to start logging in with GitHub, and it returns the
//! URL of GitHub's page asking the person to authorize sub60.

use http::Method;
use sub60_endpoints_common::body::{Json, WithMessage};
use sub60_endpoints_common::response::{Ok, WithStatusOf};
use sub60_endpoints_common::{PathParams, status};
use url::Url;

#[derive(Debug, Copy, Clone)]
pub struct Endpoint;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PathParams)]
#[route("/auth/github/login")]
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
#[schemars(rename = "GitHubLoginRequest")]
pub struct Request {
    /// The URL GitHub redirects the browser to once the person has authorized
    /// sub60. It must be one of the callback URLs registered on sub60's GitHub
    /// App.
    pub redirect_url: Url,
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
#[schemars(rename = "GitHubLoginResponse")]
pub struct Response {
    /// The URL of GitHub's authorization page.
    pub authorize_url: Url,

    /// The random value in the [authorization URL](Self::authorize_url). The
    /// client must keep it until GitHub redirects the browser to
    /// [`redirect_url`](Request::redirect_url), and refuse the redirect unless
    /// its `state` query parameter matches this.
    pub state: String,

    /// The PKCE code verifier whose challenge is in the
    /// [authorization URL](Self::authorize_url). The client must keep it
    /// secret until GitHub redirects the browser to
    /// [`redirect_url`](Request::redirect_url), and then send it with the
    /// redirect's `code` to the [callback](crate::auth_github_callback)
    /// endpoint to finish the login.
    pub code_verifier: String,
}

#[derive(
    Debug,
    derive_more::Display,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    sub60_endpoints_common::Statuses,
)]
#[serde(tag = "code", rename_all = "snake_case")]
pub enum Error {
    /// The redirect URL's scheme isn't `http` or `https`.
    #[display("the redirect URL isn't an HTTP(S) URL")]
    InvalidRedirectUrl(status::BadRequest),
}

impl sub60_endpoints_common::Endpoint for Endpoint {
    const METHOD: Method = Method::POST;

    type Path = Path;
    type Request = Json<Request>;
    type Response = Ok<Json<Response>>;
    type Error = WithStatusOf<Json<WithMessage<Error>>>;
}
