//! The bodies of requests and responses.

use alloc::borrow::Cow;
use core::marker::PhantomData;
use core::{any, fmt};

use bytes::BufMut;
use schemars::{JsonSchema, Schema, SchemaGenerator};
use serde_json::{Map, Value};

/// The body of a request or response.
pub trait Body {
    /// The body's media type, or `None` if the body is empty.
    const MEDIA_TYPE: Option<&'static str>;

    /// Returns the body's schema.
    ///
    /// This is `false` for an empty body since no content matches it.
    fn schema(generator: &mut SchemaGenerator) -> Schema;
}

pub trait Encode: Body {
    /// Appends the body's bytes to `buf`.
    fn encode(&self, buf: &mut impl BufMut);
}

pub trait Decode: Body + Sized {
    type Error: core::error::Error + Send + Sync + 'static;

    /// Decodes the body from `bytes`.
    fn decode(bytes: &[u8]) -> Result<Self, Self::Error>;
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

/// A `T` serialized with a `message` from `T`'s `Display` impl.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct WithMessage<T>(pub T);

/// The error returned when decoding a non-empty body into `()`.
#[derive(
    Debug, Copy, Clone, PartialEq, Eq, derive_more::Display, cauchy::Error,
)]
#[display("expected an empty body")]
pub struct NonEmptyBodyError;

#[doc(hidden)]
pub struct Variant<B, const I: usize>(PhantomData<B>);

/// Returns whether the `Content-Type` in `headers` is `media_type`, ignoring
/// parameters like `charset`.
pub(crate) fn has_media_type(
    headers: &http::HeaderMap,
    media_type: &str,
) -> bool {
    fn essence(media_type: &str) -> &str {
        media_type.split(';').next().unwrap_or_default().trim()
    }

    headers
        .get(http::header::CONTENT_TYPE)
        .and_then(|value| value.to_str().ok())
        .is_some_and(|value| {
            essence(value).eq_ignore_ascii_case(essence(media_type))
        })
}

impl<T: fmt::Display + serde::Serialize> serde::Serialize for WithMessage<T> {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        #[derive(serde::Serialize)]
        struct Fields<'a, T> {
            #[serde(flatten)]
            inner: &'a T,
            message: String,
        }

        Fields { inner: &self.0, message: self.0.to_string() }
            .serialize(serializer)
    }
}

impl<'de, T: serde::de::DeserializeOwned> serde::Deserialize<'de>
    for WithMessage<T>
{
    fn deserialize<D: serde::Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Self, D::Error> {
        let mut fields = Map::<String, Value>::deserialize(deserializer)?;
        fields.remove("message");
        T::deserialize(Value::Object(fields))
            .map(Self)
            .map_err(serde::de::Error::custom)
    }
}

/// The schema is `T`'s with a `message` property.
impl<T: JsonSchema> JsonSchema for WithMessage<T> {
    #[inline]
    fn inline_schema() -> bool {
        true
    }

    #[inline]
    fn schema_id() -> Cow<'static, str> {
        any::type_name::<Self>().into()
    }

    #[inline]
    fn schema_name() -> Cow<'static, str> {
        format!("{}WithMessage", T::schema_name()).into()
    }

    fn json_schema(generator: &mut SchemaGenerator) -> Schema {
        fn add_message(schema: &mut Value) {
            let Some(schema) = schema.as_object_mut() else { return };

            if let Some(properties) = schema
                .entry("properties")
                .or_insert_with(|| Value::Object(Map::new()))
                .as_object_mut()
            {
                properties.insert(
                    "message".to_owned(),
                    serde_json::json!({
                        "description": "What went wrong, as a sentence for \
                                        people.",
                        "type": "string",
                    }),
                );
            }

            if let Some(required) = schema
                .entry("required")
                .or_insert_with(|| Value::Array(Vec::new()))
                .as_array_mut()
            {
                required.push("message".into());
            }
        }

        let mut schema = Value::from(T::json_schema(generator));

        match schema.get_mut("oneOf").and_then(Value::as_array_mut) {
            Some(variants) => variants.iter_mut().for_each(add_message),
            None => add_message(&mut schema),
        }

        schema.try_into().expect("the schema is still an object")
    }
}

impl Body for () {
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

impl<B: Body, const I: usize> Body for Variant<B, I> {
    const MEDIA_TYPE: Option<&'static str> = B::MEDIA_TYPE;

    fn schema(generator: &mut SchemaGenerator) -> Schema {
        B::schema(generator)
            .get("oneOf")
            .and_then(|variants| variants.get(I))
            .cloned()
            .and_then(|variant| variant.try_into().ok())
            .unwrap_or_else(|| {
                panic!(
                    "the schema of `{}` has no variant {I}",
                    any::type_name::<B>(),
                )
            })
    }
}

impl Decode for () {
    type Error = NonEmptyBodyError;

    #[inline]
    fn decode(bytes: &[u8]) -> Result<Self, Self::Error> {
        if bytes.is_empty() { Ok(()) } else { Err(NonEmptyBodyError) }
    }
}

impl<T> Decode for Json<T>
where
    T: JsonSchema + serde::de::DeserializeOwned,
{
    type Error = serde_json::Error;

    #[inline]
    fn decode(bytes: &[u8]) -> Result<Self, Self::Error> {
        serde_json::from_slice(bytes).map(Self)
    }
}

impl Encode for () {
    #[inline]
    fn encode(&self, _: &mut impl BufMut) {}
}

impl<T: JsonSchema + serde::Serialize> Encode for Json<T> {
    #[inline]
    fn encode(&self, buf: &mut impl BufMut) {
        serde_json::to_writer(buf.writer(), &self.0)
            .expect("the body serializes to JSON");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(
        Debug,
        PartialEq,
        Eq,
        derive_more::Display,
        serde::Serialize,
        serde::Deserialize,
    )]
    #[serde(tag = "code", rename_all = "snake_case")]
    enum Error {
        NoAccess(NoAccess),
        InvalidToken(InvalidToken),
    }

    #[derive(
        Debug,
        PartialEq,
        Eq,
        derive_more::Display,
        serde::Serialize,
        serde::Deserialize,
    )]
    #[display("no access")]
    struct NoAccess;

    #[derive(
        Debug,
        PartialEq,
        Eq,
        derive_more::Display,
        serde::Serialize,
        serde::Deserialize,
    )]
    #[display("the token is invalid")]
    struct InvalidToken {
        reason: String,
    }

    fn assert_round_trips<B>(body: B)
    where
        B: Encode + Decode<Error: fmt::Debug> + fmt::Debug + PartialEq,
    {
        let mut buf = Vec::new();
        body.encode(&mut buf);
        assert_eq!(B::decode(&buf).unwrap(), body);
    }

    #[test]
    fn bodies_round_trip() {
        assert_round_trips(());
        assert_round_trips(Json(vec![1_u32, 2, 3]));
    }

    #[test]
    fn with_message_round_trips() {
        let no_access = serde_json::json!({
            "code": "no_access",
            "message": "no access",
        });

        let invalid_token = serde_json::json!({
            "code": "invalid_token",
            "reason": "expired",
            "message": "the token is invalid",
        });

        let errors = [
            (Error::NoAccess(NoAccess), no_access),
            (
                Error::InvalidToken(InvalidToken { reason: "expired".into() }),
                invalid_token,
            ),
        ];

        for (error, json) in errors {
            let error = WithMessage(error);
            assert_eq!(serde_json::to_value(&error).unwrap(), json);
            assert_eq!(
                serde_json::from_value::<WithMessage<Error>>(json).unwrap(),
                error
            );
        }
    }
}
