//! The `GET /account-names/{name}` endpoint.
//!
//! Gets an account name's availability.

use http::Method;
use sub60_endpoints_common::body::{Json, WithMessage};
use sub60_endpoints_common::response::Ok;
use sub60_endpoints_common::{AccountNameFromStrError, PathParams, error};

use crate::AccountSummary;

#[derive(Debug, Copy, Clone)]
pub struct Endpoint;

#[derive(Debug, Clone, PartialEq, Eq, PathParams)]
#[route("/account-names/{name}")]
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
pub enum NameAvailability {
    Available,
    Taken { account: AccountSummary },
    Invalid(AccountNameFromStrError),
}

#[derive(
    Debug,
    derive_more::Display,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    sub60_endpoints_common::response::Response,
)]
#[serde(tag = "code", rename_all = "snake_case")]
pub enum Error {
    Internal(error::Internal),
}

impl sub60_endpoints_common::Endpoint for Endpoint {
    const METHOD: Method = Method::GET;

    type Path = Path;
    type Request = ();
    type Response = Ok<Json<NameAvailability>>;
    type Error = Json<WithMessage<Error>>;
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
