//! The payloads of error variants that only carry a status code.

use http::StatusCode;

use crate::response::Status;

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
pub struct BadRequest;

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
pub struct Forbidden;

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
pub struct Conflict;

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
pub struct BadGateway;

impl Status for BadRequest {
    const STATUS: StatusCode = StatusCode::BAD_REQUEST;
}

impl Status for Forbidden {
    const STATUS: StatusCode = StatusCode::FORBIDDEN;
}

impl Status for Conflict {
    const STATUS: StatusCode = StatusCode::CONFLICT;
}

impl Status for BadGateway {
    const STATUS: StatusCode = StatusCode::BAD_GATEWAY;
}
