use alloc::borrow::Cow;
use core::fmt::{self, Write};
use core::str::FromStr;

/// Nix's custom base32 alphabet for digests, which includes all digits and
/// all lowercase letters except 'e', 'o', 't', and 'u'.
const ALPHABET: &[u8; 32] = b"0123456789abcdfghijklmnpqrsvwxyz";

/// A digest of `NUM_BYTES` bytes written in Nix's base32 alphabet.
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Nix32Digest<const NUM_BYTES: usize>([u8; NUM_BYTES]);

/// The type of error that can occur when parsing a string into a
/// [`Nix32Digest`].
#[derive(
    Debug, Copy, Clone, PartialEq, Eq, derive_more::Display, cauchy::Error,
)]
pub enum Nix32DigestFromStrError<const NUM_BYTES: usize> {
    /// The string doesn't have [`NUM_CHARACTERS`](Nix32Digest::NUM_CHARACTERS)
    /// characters.
    #[display(
        "string must be exactly {} characters long",
        num_chars_of_bytes(NUM_BYTES)
    )]
    InvalidLength,

    /// The string contains an invalid character at the given offset.
    #[display(
        "string contains an invalid character at offset {offset}: \
         '{character}'"
    )]
    InvalidCharacter {
        /// The invalid character.
        character: char,

        /// The offset of the first instance of the invalid character in the
        /// string.
        offset: usize,
    },
}

impl<const NUM_BYTES: usize> Nix32Digest<NUM_BYTES> {
    /// The number of characters the digest is written with.
    pub const NUM_CHARACTERS: usize = num_chars_of_bytes(NUM_BYTES);

    pub const fn new(bytes: [u8; NUM_BYTES]) -> Self {
        Self(bytes)
    }

    pub const fn as_bytes(&self) -> &[u8; NUM_BYTES] {
        &self.0
    }

    const fn is_byte_valid(byte: u8) -> bool {
        const VALID_BYTES: [bool; 256] = {
            let mut lut = [false; 256];
            let mut idx = 0;

            while idx < ALPHABET.len() {
                lut[ALPHABET[idx] as usize] = true;
                idx += 1;
            }

            lut
        };

        VALID_BYTES[byte as usize]
    }

    fn validate(s: &str) -> Result<(), Nix32DigestFromStrError<NUM_BYTES>> {
        if s.len() != Self::NUM_CHARACTERS {
            return Err(Nix32DigestFromStrError::InvalidLength);
        }

        for (offset, byte) in s.bytes().enumerate() {
            if !Self::is_byte_valid(byte) {
                let character = s[offset..]
                    .chars()
                    .next()
                    .expect("offset must be less that string length");
                return Err(Nix32DigestFromStrError::InvalidCharacter {
                    character,
                    offset,
                });
            }
        }

        Ok(())
    }
}

impl<const NUM_BYTES: usize> fmt::Debug for Nix32Digest<NUM_BYTES> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\"{self}\"")
    }
}

impl<const NUM_BYTES: usize> fmt::Display for Nix32Digest<NUM_BYTES> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // The characters encode the bytes' bits from the last to the first, 5
        // at a time, so the first character holds the last byte's top bits.
        for char_idx in 0..Self::NUM_CHARACTERS {
            let b = (Self::NUM_CHARACTERS - char_idx - 1) * 5;
            let i = b / 8;
            let j = b % 8;

            let mut c = self.0[i] >> j;
            if j != 0 && i + 1 < NUM_BYTES {
                c |= self.0[i + 1] << (8 - j);
            }

            f.write_char(char::from(ALPHABET[usize::from(c & 0x1f)]))?;
        }

        Ok(())
    }
}

impl<const NUM_BYTES: usize> FromStr for Nix32Digest<NUM_BYTES> {
    type Err = Nix32DigestFromStrError<NUM_BYTES>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::validate(s)?;
        let mut bytes = [0; NUM_BYTES];
        decode_chars_into_bytes(s.as_bytes(), &mut bytes);
        Ok(Self(bytes))
    }
}

impl<const NUM_BYTES: usize> serde::ser::Serialize for Nix32Digest<NUM_BYTES> {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.collect_str(self)
    }
}

impl<'de, const NUM_BYTES: usize> serde::de::Deserialize<'de>
    for Nix32Digest<NUM_BYTES>
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let s = Cow::<'de, str>::deserialize(deserializer)?;
        Self::from_str(&s).map_err(serde::de::Error::custom)
    }
}

#[cfg(feature = "schemars")]
impl<const NUM_BYTES: usize> schemars::JsonSchema for Nix32Digest<NUM_BYTES> {
    fn inline_schema() -> bool {
        true
    }

    fn schema_name() -> Cow<'static, str> {
        "Nix32Digest".into()
    }

    fn json_schema(_: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schemars::json_schema!({
            "description": alloc::format!(
                "A {NUM_BYTES}-byte digest in Nix's base32 alphabet."
            ),
            "type": "string",
            "pattern": alloc::format!(
                "^[0-9a-df-np-sv-z]{{{}}}$",
                Self::NUM_CHARACTERS
            ),
        })
    }
}

const fn num_chars_of_bytes(num_bytes: usize) -> usize {
    (num_bytes * 8 - 1) / 5 + 1
}

const fn decode_chars_into_bytes(chars: &[u8], bytes: &mut [u8]) {
    let mut n = 0;
    while n < chars.len() {
        let ch = chars[chars.len() - n - 1];
        let digit = decode_digit(ch);

        let b = n * 5;
        let i = b / 8;
        let j = b % 8;

        bytes[i] |= digit.wrapping_shl(j as u32);

        if j != 0 {
            let carry = digit >> (8 - j);
            if i + 1 < bytes.len() {
                bytes[i + 1] |= carry;
            }
        }
        n += 1;
    }
}

const fn decode_digit(byte: u8) -> u8 {
    let mut idx = 0;

    while idx < ALPHABET.len() {
        if ALPHABET[idx] == byte {
            return idx as u8;
        }
        idx += 1;
    }

    panic!("Nix32Digest is always validated")
}

#[cfg(test)]
mod tests {
    use alloc::string::ToString;

    use super::*;

    /// The SHA-256 of the empty string and what `nix hash convert --to
    /// nix32` prints for it.
    const EMPTY_SHA256: ([u8; 32], &str) = (
        [
            0xe3, 0xb0, 0xc4, 0x42, 0x98, 0xfc, 0x1c, 0x14, 0x9a, 0xfb, 0xf4,
            0xc8, 0x99, 0x6f, 0xb9, 0x24, 0x27, 0xae, 0x41, 0xe4, 0x64, 0x9b,
            0x93, 0x4c, 0xa4, 0x95, 0x99, 0x1b, 0x78, 0x52, 0xb8, 0x55,
        ],
        "0mdqa9w1p6cmli6976v4wi0sw9r4p5prkj7lzfd1877wk11c9c73",
    );

    #[test]
    fn display_matches_nix() {
        let (bytes, nix32) = EMPTY_SHA256;
        assert_eq!(Nix32Digest::new(bytes).to_string(), nix32);
    }

    #[test]
    fn from_str_matches_nix() {
        let (bytes, nix32) = EMPTY_SHA256;
        assert_eq!(
            nix32.parse::<Nix32Digest<32>>().unwrap().as_bytes(),
            &bytes
        );
    }

    #[test]
    fn display_from_str_roundtrip() {
        for bytes in [[0; 20], [0xff; 20], *b"0123456789abcdefghij"] {
            let digest = Nix32Digest::new(bytes);
            let text = digest.to_string();
            assert_eq!(text.len(), Nix32Digest::<20>::NUM_CHARACTERS);
            assert_eq!(text.parse::<Nix32Digest<20>>(), Ok(digest));
        }
    }
}
