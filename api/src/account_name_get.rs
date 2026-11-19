//! The `GET /account-names/{name}` endpoint.
//!
//! Gets whether a given candidate name can be given to a new account.

use http::{Method, StatusCode};
use sub60_endpoints_common::{
    AccountNameFromStrError,
    Json,
    JsonError,
    Ok,
    PathParams,
};

#[derive(Debug, Copy, Clone)]
#[expect(missing_docs)]
pub struct Endpoint;

#[derive(Debug, Clone, PartialEq, Eq, PathParams)]
#[route("/account-names/{name}")]
#[expect(missing_docs)]
pub struct Path {
    pub name: String,
}

#[derive(
    Debug,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
)]
#[serde(tag = "status", rename_all = "snake_case")]
#[expect(missing_docs)]
pub enum NameAvailability {
    Available,
    Taken,
    Invalid(AccountNameFromStrError),
}

#[derive(
    Debug, derive_more::Display, serde::Serialize, schemars::JsonSchema,
)]
#[serde(tag = "code", content = "details", rename_all = "snake_case")]
#[schemars(rename = "GetAccountNameError")]
#[expect(missing_docs)]
pub enum Error {
    #[display("the server failed to look the name up")]
    Internal,
}

impl sub60_endpoints_common::Endpoint for Endpoint {
    const METHOD: Method = Method::GET;

    type Path = Path;
    type Request = ();
    type Response = Ok<Json<NameAvailability>>;
    type Error = JsonError<Error>;
}

impl From<Error> for StatusCode {
    fn from(error: Error) -> Self {
        match error {
            Error::Internal => Self::INTERNAL_SERVER_ERROR,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invalid_serializes_the_reason_next_to_the_status() {
        let availability = NameAvailability::Invalid(
            AccountNameFromStrError::InvalidCharacter {
                character: '!',
                offset: 3,
            },
        );

        let json = serde_json::json!({
            "status": "invalid",
            "code": "invalid_character",
            "details": { "character": "!", "offset": 3 },
        });

        assert_eq!(serde_json::to_value(&availability).unwrap(), json);
        assert_eq!(
            serde_json::from_value::<NameAvailability>(json).unwrap(),
            availability,
        );
    }
}
