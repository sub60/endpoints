use alloc::borrow::Cow;
use core::marker::PhantomData;
use core::{any, fmt};

use http::StatusCode;
use schemars::{JsonSchema, Schema, SchemaGenerator};
use serde_json::{Map, Value};

use crate::response::VariantVisitor;
use crate::{Response, ResponseVisitor};

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

/// A `T` serialized with a `message` from `T`'s `Display` impl.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct WithMessage<T>(pub T);

struct Variant<Enum, const I: usize>(PhantomData<Enum>);

struct VariantBodies<'a, E, V> {
    visitor: &'a mut V,
    error: PhantomData<E>,
}

struct WithBody<'a, B, V> {
    visitor: &'a mut V,
    body: PhantomData<B>,
}

impl<E: Response + JsonSchema> Response for Json<WithMessage<E>> {
    #[inline]
    fn status(&self) -> StatusCode {
        self.0.0.status()
    }

    #[inline]
    fn visit<V: ResponseVisitor>(visitor: &mut V) {
        E::visit_variants(&mut VariantBodies::<E, V> {
            visitor,
            error: PhantomData,
        });
    }
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

impl<E: JsonSchema, const I: usize> JsonSchema for Variant<E, I> {
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
        format!("{}Variant{I}", E::schema_name()).into()
    }

    fn json_schema(generator: &mut SchemaGenerator) -> Schema {
        let schema = E::json_schema(generator);

        schema
            .get("oneOf")
            .and_then(|variants| variants.get(I))
            .cloned()
            .and_then(|variant| variant.try_into().ok())
            .unwrap_or_else(|| {
                panic!(
                    "the schema of `{}` has no variant {I}",
                    any::type_name::<E>(),
                )
            })
    }
}

impl<E, V> VariantVisitor for VariantBodies<'_, E, V>
where
    E: JsonSchema,
    V: ResponseVisitor,
{
    #[inline]
    fn visit<T: Response, const I: usize>(&mut self) {
        T::visit(&mut WithBody::<Json<WithMessage<Variant<E, I>>>, V> {
            visitor: self.visitor,
            body: PhantomData,
        });
    }
}

impl<B: Body, V: ResponseVisitor> ResponseVisitor for WithBody<'_, B, V> {
    #[inline]
    fn visit<Ignored: Body>(
        &mut self,
        status: StatusCode,
        description: Option<&'static str>,
    ) {
        self.visitor.visit::<B>(status, description);
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
