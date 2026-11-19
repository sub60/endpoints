//! The `POST /v1/auth/github/login` endpoint.
//!
//! This endpoint is used to start logging in with GitHub, and it returns the
//! URL of GitHub's page asking the person to authorize sub60.

use http::{Method, StatusCode};

use crate::{PathParams, V1Path};

#[derive(Debug, Copy, Clone)]
#[expect(missing_docs)]
pub struct Endpoint;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PathParams)]
#[route(V1Path + "/auth/github/login")]
#[expect(missing_docs)]
pub struct Path {
    pub v1: V1Path,
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
#[schemars(rename = "GitHubLoginRequest")]
#[expect(missing_docs)]
pub struct Request {
    /// The URL GitHub redirects the browser to once the person has authorized
    /// sub60. It must be one of the callback URLs registered on sub60's GitHub
    /// App.
    pub redirect_uri: String,
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
#[expect(missing_docs)]
pub struct Response {
    /// The URL of GitHub's authorization page.
    pub authorize_url: String,

    /// The random value in the [authorization URL](Self::authorize_url). The
    /// client must keep it until GitHub redirects the browser to
    /// [`redirect_uri`](Request::redirect_uri), and refuse the redirect unless
    /// its `state` query parameter matches this.
    pub state: String,

    /// The PKCE code verifier whose challenge is in the
    /// [authorization URL](Self::authorize_url). The client must keep it
    /// secret until GitHub redirects the browser to
    /// [`redirect_uri`](Request::redirect_uri), and then send it with the
    /// redirect's `code` to the [callback](crate::auth_github_callback)
    /// endpoint to finish the login.
    pub code_verifier: String,
}

#[derive(
    Debug, derive_more::Display, serde::Serialize, schemars::JsonSchema,
)]
#[serde(tag = "code", content = "details", rename_all = "snake_case")]
#[schemars(rename = "GitHubLoginError")]
#[expect(missing_docs)]
pub enum Error {
    /// The redirect URI isn't an absolute `http` or `https` URL.
    #[display("the redirect URI isn't an absolute HTTP(S) URL")]
    InvalidRedirectUri,
}

impl crate::Endpoint for Endpoint {
    const METHOD: Method = Method::POST;

    type Error = Error;
    type Path = Path;
    type Request = Request;
    type Response = Response;
}

impl From<Error> for StatusCode {
    fn from(error: Error) -> Self {
        match error {
            Error::InvalidRedirectUri => Self::BAD_REQUEST,
        }
    }
}
