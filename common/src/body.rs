use core::{any, fmt};

use http::StatusCode;
use schemars::{JsonSchema, Schema, SchemaGenerator};

/// The body of a request or response.
pub trait Body {
    /// The body's media type, or `None` if the body is empty.
    const MEDIA_TYPE: Option<&'static str>;

    /// Returns the body's schema.
    ///
    /// This is `false` for an empty body since no content matches it.
    fn schema(generator: &mut SchemaGenerator) -> Schema;
}

/// A body holding a `T` serialized as JSON.
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
)]
#[serde(transparent)]
pub struct Json<T>(pub T);

/// The JSON body of an error response, which is the error serialized with its
/// `code`, and its `details` if it has any, next to a `message` made from its
/// `Display`.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct JsonError<E>(pub E);

impl<E: fmt::Display> fmt::Display for JsonError<E> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<E: Into<Self>> From<JsonError<E>> for StatusCode {
    #[inline]
    fn from(error: JsonError<E>) -> Self {
        error.0.into()
    }
}

impl<E: serde::Serialize + fmt::Display> serde::Serialize for JsonError<E> {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        #[derive(serde::Serialize)]
        struct Envelope<'a, E> {
            #[serde(flatten)]
            error: &'a E,
            message: String,
        }

        Envelope { error: &self.0, message: self.0.to_string() }
            .serialize(serializer)
    }
}

impl Body for () {
    const MEDIA_TYPE: Option<&'static str> = None;

    #[inline]
    fn schema(_: &mut SchemaGenerator) -> Schema {
        false.into()
    }
}

impl Body for StatusCode {
    const MEDIA_TYPE: Option<&'static str> = None;

    #[inline]
    fn schema(_: &mut SchemaGenerator) -> Schema {
        false.into()
    }
}

impl<T: JsonSchema> Body for Json<T> {
    const MEDIA_TYPE: Option<&'static str> = Some("application/json");

    #[inline]
    fn schema(generator: &mut SchemaGenerator) -> Schema {
        generator.subschema_for::<T>()
    }
}

impl<E: JsonSchema> Body for JsonError<E> {
    const MEDIA_TYPE: Option<&'static str> = Some("application/json");

    fn schema(generator: &mut SchemaGenerator) -> Schema {
        let schema = generator.subschema_for::<E>();

        let ref_prefix = format!("#{}/", generator.settings().definitions_path);

        // The schema is usually a reference to the type's definition.
        let resolved = schema
            .get("$ref")
            .and_then(|path| path.as_str()?.strip_prefix(&*ref_prefix))
            .and_then(|name| generator.definitions().get(name))
            .unwrap_or_else(|| schema.as_value());

        let variants = match resolved.get("oneOf").and_then(|v| v.as_array()) {
            Some(variants) => variants.iter().collect(),
            None => vec![resolved],
        };

        for variant in variants {
            let has_code = variant
                .pointer("/properties/code/const")
                .is_some_and(|code| code.is_string())
                && variant
                    .get("required")
                    .and_then(|required| required.as_array())
                    .is_some_and(|required| {
                        required.iter().any(|name| *name == "code")
                    });

            assert!(
                has_code,
                "a variant of `{}` isn't serialized with a constant `code` \
                 string, add `#[serde(tag = \"code\", content = \
                 \"details\")]` to it",
                any::type_name::<E>(),
            );
        }

        schemars::json_schema!({
            "allOf": [
                schema,
                {
                    "type": "object",
                    "properties": {
                        "message": {
                            "description": "What went wrong, as a sentence for people.",
                            "type": "string",
                        },
                    },
                    "required": ["message"],
                },
            ],
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(derive_more::Display, serde::Serialize)]
    #[serde(tag = "code", content = "details", rename_all = "snake_case")]
    enum Error {
        #[display("no access")]
        NoAccess,

        #[display("the token is invalid")]
        InvalidToken { reason: &'static str },
    }

    #[test]
    fn json_error_serializes_code_details_and_message() {
        assert_eq!(
            serde_json::to_value(JsonError(Error::NoAccess)).unwrap(),
            serde_json::json!({ "code": "no_access", "message": "no access" }),
        );

        assert_eq!(
            serde_json::to_value(JsonError(Error::InvalidToken {
                reason: "expired",
            }))
            .unwrap(),
            serde_json::json!({
                "code": "invalid_token",
                "details": { "reason": "expired" },
                "message": "the token is invalid",
            }),
        );
    }
}
