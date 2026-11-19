use http::StatusCode;

/// The ways a request can fail before it reaches an endpoint, e.g. when its
/// path matches no endpoint.
#[derive(
    Debug, derive_more::Display, serde::Serialize, schemars::JsonSchema,
)]
#[serde(tag = "code", content = "details", rename_all = "snake_case")]
pub enum RequestError {
    /// The request's path or body didn't parse.
    #[display("the request is invalid")]
    InvalidRequest,

    /// No endpoint has the request's path.
    #[display("there's no endpoint at the request's path")]
    NotFound,

    /// The endpoint at the request's path doesn't take the request's method.
    #[display("the endpoint at the request's path doesn't take its method")]
    MethodNotAllowed,

    /// The server failed to handle the request.
    #[display("the server failed to handle the request")]
    Internal,
}

impl From<StatusCode> for RequestError {
    fn from(status: StatusCode) -> Self {
        match status {
            StatusCode::NOT_FOUND => Self::NotFound,
            StatusCode::METHOD_NOT_ALLOWED => Self::MethodNotAllowed,
            status if status.is_server_error() => Self::Internal,
            _ => Self::InvalidRequest,
        }
    }
}
