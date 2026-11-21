use alloc::borrow::Cow;
use core::fmt;

use sub60_endpoints_common::Timestamp;

/// When a token stops being valid.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Expiration {
    Never,
    At(Timestamp),
}

impl serde::Serialize for Expiration {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        match self {
            Self::Never => serializer.serialize_str("never"),
            Self::At(timestamp) => timestamp.serialize(serializer),
        }
    }
}

impl<'de> serde::Deserialize<'de> for Expiration {
    fn deserialize<D: serde::Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Self, D::Error> {
        struct Visitor;

        impl serde::de::Visitor<'_> for Visitor {
            type Value = Expiration;

            fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str("`\"never\"` or a timestamp")
            }

            fn visit_str<E: serde::de::Error>(
                self,
                s: &str,
            ) -> Result<Self::Value, E> {
                match s {
                    "never" => Ok(Expiration::Never),
                    _ => Err(E::invalid_value(
                        serde::de::Unexpected::Str(s),
                        &self,
                    )),
                }
            }

            fn visit_u64<E: serde::de::Error>(
                self,
                secs: u64,
            ) -> Result<Self::Value, E> {
                Ok(Expiration::At(Timestamp::from_secs(secs)))
            }
        }

        deserializer.deserialize_any(Visitor)
    }
}

impl schemars::JsonSchema for Expiration {
    fn schema_name() -> Cow<'static, str> {
        "Expiration".into()
    }

    fn json_schema(
        generator: &mut schemars::SchemaGenerator,
    ) -> schemars::Schema {
        schemars::json_schema!({
            "description": "When a token stops being valid.",
            "anyOf": [
                { "const": "never", "type": "string" },
                generator.subschema_for::<Timestamp>(),
            ],
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serde_roundtrip() {
        for (expiration, json) in [
            (Expiration::Never, r#""never""#),
            (Expiration::At(Timestamp::from_secs(1_767_225_600)), "1767225600"),
        ] {
            assert_eq!(serde_json::to_string(&expiration).unwrap(), json);
            assert_eq!(
                serde_json::from_str::<Expiration>(json).unwrap(),
                expiration
            );
        }
    }
}
