//! The `GET /caches/{cache_id}/paths` endpoint.
//!
//! Lists a cache's store paths.

use alloc::borrow::Cow;
use core::fmt;
use core::num::NonZeroU16;
use core::ops::Deref;
use core::str::FromStr;

use http::Method;
use sub60_endpoints_common::body::{Json, WithMessage};
use sub60_endpoints_common::request::WithQuery;
use sub60_endpoints_common::response::{Ok, WithStatusOf};
use sub60_endpoints_common::{Cursor, PathParams, error};

use crate::{CacheId, PathInfo};

#[derive(Debug, Copy, Clone)]
pub struct Endpoint;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PathParams)]
#[route("/caches/{cache_id}/paths")]
pub struct Path {
    pub cache_id: CacheId,
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
pub struct Query {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contains: Option<BasenameFragment>,
    #[serde(default)]
    pub order_by: PathOrder,
    pub limit: NonZeroU16,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<Cursor>,
}

/// A part of a store path's basename of at least [`MIN_LEN`](Self::MIN_LEN)
/// characters.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
#[serde(transparent)]
pub struct BasenameFragment(String);

#[derive(
    Debug,
    Copy,
    Clone,
    Default,
    PartialEq,
    Eq,
    Hash,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
)]
pub enum PathOrder {
    #[default]
    #[serde(rename = "-pushed_at")]
    Newest,
    #[serde(rename = "pushed_at")]
    Oldest,
    #[serde(rename = "-file_size")]
    Largest,
    #[serde(rename = "file_size")]
    Smallest,
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
#[schemars(rename = "PathInfoList")]
pub struct Response {
    pub paths: Vec<PathInfo>,
    pub next_cursor: Option<Cursor>,
}

/// The error returned when parsing a string that isn't a [`BasenameFragment`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, derive_more::Display)]
pub enum BasenameFragmentFromStrError {
    /// The string is shorter than [`MIN_LEN`](BasenameFragment::MIN_LEN).
    #[display(
        "a basename fragment has at least {} characters",
        BasenameFragment::MIN_LEN
    )]
    TooShort,

    /// The string has a character no basename can contain.
    #[display(
        "a basename can't contain '{character}', found at offset {offset}"
    )]
    InvalidCharacter { character: char, offset: usize },
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
    Unauthorized(error::Unauthorized),
    Forbidden(error::Forbidden),
    NotFound(error::NotFound),
    InvalidCursor(error::InvalidCursor),
    Internal(error::Internal),
}

impl BasenameFragment {
    /// The minimum number of characters in a fragment.
    pub const MIN_LEN: usize = 3;

    fn validate(s: &str) -> Result<(), BasenameFragmentFromStrError> {
        for (offset, character) in s.char_indices() {
            if !character.is_ascii_alphanumeric()
                && !matches!(character, '+' | '-' | '.' | '_' | '?' | '=')
            {
                return Err(BasenameFragmentFromStrError::InvalidCharacter {
                    character,
                    offset,
                });
            }
        }

        // Every character is ASCII, so the byte length is the number of
        // characters.
        if s.len() < Self::MIN_LEN {
            return Err(BasenameFragmentFromStrError::TooShort);
        }

        Ok(())
    }
}

impl sub60_endpoints_common::Endpoint for Endpoint {
    const METHOD: Method = Method::GET;

    type Path = Path;
    type Request = WithQuery<Query>;
    type Response = Ok<Json<Response>>;
    type Error = WithStatusOf<Json<WithMessage<Error>>>;
}

impl fmt::Display for BasenameFragment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl FromStr for BasenameFragment {
    type Err = BasenameFragmentFromStrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::validate(s)?;
        Ok(Self(s.to_owned()))
    }
}

impl TryFrom<&str> for BasenameFragment {
    type Error = BasenameFragmentFromStrError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl Deref for BasenameFragment {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'de> serde::Deserialize<'de> for BasenameFragment {
    fn deserialize<D: serde::Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        Self::validate(&s).map_err(serde::de::Error::custom)?;
        Ok(Self(s))
    }
}

impl schemars::JsonSchema for BasenameFragment {
    fn schema_name() -> Cow<'static, str> {
        "BasenameFragment".into()
    }

    fn json_schema(_: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schemars::json_schema!({
            "description": "A part of a store path's basename.",
            "type": "string",
            "minLength": Self::MIN_LEN,
            "pattern": "^[0-9A-Za-z+._?=-]*$",
        })
    }
}

impl core::error::Error for BasenameFragmentFromStrError {}
