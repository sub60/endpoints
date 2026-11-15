use core::fmt;
use core::hash::{Hash, Hasher};
use core::ops::Deref;
use core::str::FromStr;

use smol_str::SmolStr;

/// The type representing the name of an account.
///
/// Valid names are between 1 and [`MAX_LEN`](Self::MAX_LEN) bytes long, and can
/// only contain ASCII letters and digits, `-` and `_`. They must start with a
/// letter or a digit.
///
/// Names keep the casing they were created with (e.g. `NixOS`). Two names that
/// only differ in casing are considered equal as far as [`PartialEq`] and
/// [`Hash`] are concerned.
#[derive(Debug, Clone)]
pub struct AccountName(SmolStr);

/// The type of error that can occur when parsing a string into a
/// [`AccountName`].
#[derive(Debug, PartialEq, Eq, derive_more::Display, cauchy::Error)]
pub enum AccountNameFromStrError {
    /// The string was empty.
    #[display("Account name cannot be empty")]
    Empty,

    /// The string was longer than the [maximum](AccountName::MAX_LEN) length.
    #[display(
        "Account name cannot be longer than {} characters",
        AccountName::MAX_LEN
    )]
    TooLong,

    /// The account name starts with a `-` or a `_`.
    #[display(
        "Account name must start with a letter or a digit, not '{character}'"
    )]
    InvalidFirstCharacter {
        /// The first character.
        character: char,
    },

    /// The account name contains an invalid character at the given offset.
    #[display(
        "Account name contains an invalid character at offset {offset}: \
         '{character}'"
    )]
    InvalidCharacter {
        /// The invalid character.
        character: char,

        /// The offset of the invalid character.
        offset: usize,
    },
}

impl AccountName {
    /// The maximum length of an account name, in bytes.
    pub const MAX_LEN: usize = 64;

    fn validate(s: &str) -> Result<(), AccountNameFromStrError> {
        if s.is_empty() {
            return Err(AccountNameFromStrError::Empty);
        }

        if s.len() > Self::MAX_LEN {
            return Err(AccountNameFromStrError::TooLong);
        }

        if let Some(character @ ('-' | '_')) = s.chars().next() {
            return Err(AccountNameFromStrError::InvalidFirstCharacter {
                character,
            });
        }

        for (offset, character) in s.char_indices() {
            if !character.is_ascii_alphanumeric()
                && character != '-'
                && character != '_'
            {
                return Err(AccountNameFromStrError::InvalidCharacter {
                    character,
                    offset,
                });
            }
        }

        Ok(())
    }
}

impl PartialEq for AccountName {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq_ignore_ascii_case(&other.0)
    }
}

impl Eq for AccountName {}

impl Hash for AccountName {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Must be consistent with `PartialEq`, so names that only differ in
        // casing must hash the same.
        for byte in self.0.bytes() {
            state.write_u8(byte.to_ascii_lowercase());
        }
        // Delimit the name like `str`'s `Hash` impl does, so that hashing a
        // tuple of names doesn't depend on where one ends and the next starts.
        state.write_u8(0xff);
    }
}

impl fmt::Display for AccountName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl FromStr for AccountName {
    type Err = AccountNameFromStrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::validate(s)?;
        Ok(Self(SmolStr::new(s)))
    }
}

impl TryFrom<&str> for AccountName {
    type Error = <Self as FromStr>::Err;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl Deref for AccountName {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'de> serde::de::Deserialize<'de> for AccountName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let s = SmolStr::deserialize(deserializer)?;
        Self::validate(&s).map_err(serde::de::Error::custom)?;
        Ok(Self(s))
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use std::hash::DefaultHasher;

    use super::*;

    fn name(s: &str) -> AccountName {
        s.parse().unwrap()
    }

    fn hash_of(value: &impl Hash) -> u64 {
        let mut hasher = DefaultHasher::new();
        value.hash(&mut hasher);
        hasher.finish()
    }

    #[test]
    fn must_start_with_letter_or_digit() {
        for s in ["-", "_", "-foo", "_foo"] {
            assert_eq!(
                s.parse::<AccountName>().unwrap_err(),
                AccountNameFromStrError::InvalidFirstCharacter {
                    character: s.chars().next().unwrap(),
                },
                "{s}",
            );
        }
    }

    #[test]
    fn can_contain_dash_and_underscore_after_first_character() {
        for s in ["a-", "a_", "foo-bar_baz", "0foo"] {
            assert!(s.parse::<AccountName>().is_ok(), "{s}");
        }
    }

    #[test]
    fn keeps_casing() {
        assert_eq!(name("NixOS").to_string(), "NixOS");
    }

    #[test]
    fn eq_ignores_case() {
        assert_eq!(name("NixOS"), name("nixos"));
        assert_eq!(name("NixOS"), name("NIXOS"));
        assert_ne!(name("NixOS"), name("nixos-2"));
    }

    #[test]
    fn hash_ignores_case() {
        let names = HashSet::from([name("NixOS")]);
        assert!(names.contains(&name("nixos")));
        assert!(!names.contains(&name("nixos-2")));
    }

    #[test]
    fn hash_delimits_names() {
        assert_ne!(
            hash_of(&(name("ab"), name("c"))),
            hash_of(&(name("a"), name("bc"))),
        );
    }
}
