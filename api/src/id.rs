use alloc::borrow::Cow;
use core::fmt;
use core::str::FromStr;

const BASE62_ALPHABET: &[u8; 62] =
    b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

const NUM_DIGITS_AFTER_KIND: usize = 21;

// Each kind's u8 repr is the first base62 digit of its IDs' text form, so it
// must never change, be reused after the kind is removed, or be >= 62.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(u8)]
enum IdKind {
    User = 0,
    Account = 1,
    Cache = 2,
    MachineToken = 3,
    UserToken = 4,
    CiJobGrant = 5,
    MachineGrant = 6,
    UserGrant = 7,
    Invoice = 8,
    Session = 9,
}

/// The error returned when parsing text that isn't an ID of the expected kind.
#[derive(Debug, Copy, Clone, PartialEq, Eq, derive_more::Display)]
#[display("invalid ID")]
pub struct IdFromStrError;

macro_rules! id {
    ($(#[$attr:meta])* $name:ident, $kind:expr) => {
        $(#[$attr])*
        #[derive(Copy, Clone, PartialEq, Eq, Hash)]
        pub struct $name(pub [u8; 15]);

        impl $name {
            const KIND: IdKind = $kind;
        }

        impl fmt::Debug for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.debug_tuple(stringify!($name))
                    .field(&format_args!("{self}"))
                    .finish()
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write_id(Self::KIND, &self.0, f)
            }
        }

        impl FromStr for $name {
            type Err = IdFromStrError;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                parse_id(s, Self::KIND).map(Self)
            }
        }

        impl serde::Serialize for $name {
            fn serialize<S: serde::Serializer>(
                &self,
                serializer: S,
            ) -> Result<S::Ok, S::Error> {
                serializer.collect_str(self)
            }
        }

        impl<'de> serde::Deserialize<'de> for $name {
            fn deserialize<D: serde::Deserializer<'de>>(
                deserializer: D,
            ) -> Result<Self, D::Error> {
                struct Visitor;

                impl serde::de::Visitor<'_> for Visitor {
                    type Value = $name;

                    fn expecting(
                        &self,
                        f: &mut fmt::Formatter<'_>,
                    ) -> fmt::Result {
                        f.write_str("an ID")
                    }

                    fn visit_str<E: serde::de::Error>(
                        self,
                        s: &str,
                    ) -> Result<Self::Value, E> {
                        s.parse().map_err(E::custom)
                    }
                }

                deserializer.deserialize_str(Visitor)
            }
        }

        impl schemars::JsonSchema for $name {
            fn schema_name() -> Cow<'static, str> {
                stringify!($name).into()
            }

            fn json_schema(
                _: &mut schemars::SchemaGenerator,
            ) -> schemars::Schema {
                let kind = char::from(BASE62_ALPHABET[Self::KIND as usize]);
                schemars::json_schema!({
                    "type": "string",
                    "pattern": format!("^{kind}[0-9A-Za-z]{{21}}$"),
                })
            }
        }
    };
}

id!(AccountId, IdKind::Account);
id!(CacheId, IdKind::Cache);
id!(CiJobGrantId, IdKind::CiJobGrant);
id!(InvoiceId, IdKind::Invoice);
id!(MachineGrantId, IdKind::MachineGrant);
id!(MachineTokenId, IdKind::MachineToken);
id!(SessionId, IdKind::Session);
id!(UserGrantId, IdKind::UserGrant);
id!(UserId, IdKind::User);
id!(UserTokenId, IdKind::UserToken);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum GrantId {
    CiJob(CiJobGrantId),
    Machine(MachineGrantId),
    User(UserGrantId),
}

impl fmt::Display for GrantId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CiJob(id) => id.fmt(f),
            Self::Machine(id) => id.fmt(f),
            Self::User(id) => id.fmt(f),
        }
    }
}

impl FromStr for GrantId {
    type Err = IdFromStrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if has_kind(s, CiJobGrantId::KIND) {
            s.parse().map(Self::CiJob)
        } else if has_kind(s, MachineGrantId::KIND) {
            s.parse().map(Self::Machine)
        } else if has_kind(s, UserGrantId::KIND) {
            s.parse().map(Self::User)
        } else {
            Err(IdFromStrError)
        }
    }
}

impl serde::Serialize for GrantId {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        serializer.collect_str(self)
    }
}

impl<'de> serde::Deserialize<'de> for GrantId {
    fn deserialize<D: serde::Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Self, D::Error> {
        Cow::<'de, str>::deserialize(deserializer)?
            .parse()
            .map_err(serde::de::Error::custom)
    }
}

impl schemars::JsonSchema for GrantId {
    fn schema_name() -> Cow<'static, str> {
        "GrantId".into()
    }

    fn json_schema(_: &mut schemars::SchemaGenerator) -> schemars::Schema {
        let kinds =
            [CiJobGrantId::KIND, MachineGrantId::KIND, UserGrantId::KIND]
                .map(|kind| char::from(BASE62_ALPHABET[kind as usize]))
                .into_iter()
                .collect::<alloc::string::String>();

        schemars::json_schema!({
            "type": "string",
            "pattern": format!("^[{kinds}][0-9A-Za-z]{{21}}$"),
        })
    }
}

fn has_kind(s: &str, kind: IdKind) -> bool {
    s.as_bytes().first() == Some(&BASE62_ALPHABET[kind as usize])
}

fn write_id(
    kind: IdKind,
    random: &[u8; 15],
    f: &mut fmt::Formatter<'_>,
) -> fmt::Result {
    let mut padded = [0; 16];
    padded[1..].copy_from_slice(random);
    let mut number = u128::from_be_bytes(padded);

    let mut digits = [0; 1 + NUM_DIGITS_AFTER_KIND];
    digits[0] = BASE62_ALPHABET[kind as usize];

    for digit in digits[1..].iter_mut().rev() {
        *digit = BASE62_ALPHABET[(number % 62) as usize];
        number /= 62;
    }

    f.write_str(str::from_utf8(&digits).expect("digits are ASCII"))
}

fn parse_id(s: &str, kind: IdKind) -> Result<[u8; 15], IdFromStrError> {
    let digits = s
        .strip_prefix(char::from(BASE62_ALPHABET[kind as usize]))
        .ok_or(IdFromStrError)?;

    if digits.len() != NUM_DIGITS_AFTER_KIND {
        return Err(IdFromStrError);
    }

    let mut number = 0u128;

    for digit in digits.bytes() {
        let value = BASE62_ALPHABET
            .iter()
            .position(|&c| c == digit)
            .ok_or(IdFromStrError)?;

        number = number * 62 + value as u128;
    }

    // The digits can encode numbers that don't fit in 15 bytes.
    if number >> 120 != 0 {
        return Err(IdFromStrError);
    }

    let [_, random @ ..] = number.to_be_bytes();
    Ok(random)
}

impl core::error::Error for IdFromStrError {}

#[cfg(test)]
mod tests {
    use alloc::format;
    use alloc::string::ToString;

    use super::*;

    #[test]
    fn display_from_str_roundtrip() {
        for random in [[0; 15], [0xff; 15], *b"0123456789abcde"] {
            let id = AccountId(random);
            let text = id.to_string();
            assert_eq!(text.len(), 1 + NUM_DIGITS_AFTER_KIND, "{text}");
            assert!(text.starts_with('1'), "{text}");
            assert_eq!(text.parse::<AccountId>(), Ok(id), "{text}");
        }
    }

    #[test]
    fn display_pads_with_zeros() {
        assert_eq!(AccountId([0; 15]).to_string(), "1000000000000000000000");
    }

    #[test]
    fn from_str_rejects_invalid_text() {
        let text = UserId([7; 15]).to_string();
        let digits = &text[1..];

        for text in [
            // Another kind's.
            AccountId([7; 15]).to_string(),
            // Wrong length.
            text[1..].to_owned(),
            format!("{text}0"),
            // Not base62.
            format!("0{}-", &digits[1..]),
            // More than 15 bytes after the kind.
            format!("0{}", "z".repeat(NUM_DIGITS_AFTER_KIND)),
        ] {
            assert_eq!(text.parse::<UserId>(), Err(IdFromStrError), "{text}");
        }
    }

    #[test]
    fn serde_roundtrip() {
        let id = AccountId(*b"0123456789abcde");
        let json = serde_json::to_string(&id).unwrap();
        assert_eq!(json, format!("\"{id}\""));
        assert_eq!(serde_json::from_str::<AccountId>(&json).unwrap(), id);
    }
}
