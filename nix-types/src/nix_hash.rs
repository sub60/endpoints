use core::fmt;
use core::str::FromStr;

use crate::Nix32Digest;

/// TODO: docs.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum NixHash {
    /// The BLAKE3 hash algorithm.
    Blake3(Nix32Digest<32>),
    /// The MD5 hash algorithm.
    Md5(Nix32Digest<16>),
    /// The SHA-1 hash algorithm.
    Sha1(Nix32Digest<20>),
    /// The SHA-256 hash algorithm.
    Sha256(Nix32Digest<32>),
    /// The SHA-512 hash algorithm.
    Sha512(Nix32Digest<64>),
}

/// The type of error that can occur when parsing a string into a
/// [`NixHash`].
#[derive(Debug, PartialEq, Eq, derive_more::Display, cauchy::Error)]
pub enum NixHashFromStrError {
    /// The hash value did not parse into a valid BLAKE3 hash.
    #[display("{_0}")]
    InvalidBlake3Hash(<Nix32Digest<32> as FromStr>::Err),

    /// The hash algorithm specified in the string was not one of the supported
    /// ones.
    #[display("invalid hash algorithm")]
    InvalidHashAlgorithm,

    /// The hash value did not parse into a valid MD5 hash.
    #[display("{_0}")]
    InvalidMd5Hash(<Nix32Digest<16> as FromStr>::Err),

    /// The hash value did not parse into a valid SHA-1 hash.
    #[display("{_0}")]
    InvalidSha1Hash(<Nix32Digest<20> as FromStr>::Err),

    /// The hash value did not parse into a valid SHA-256 hash.
    #[display("{_0}")]
    InvalidSha256Hash(<Nix32Digest<32> as FromStr>::Err),

    /// The hash value did not parse into a valid SHA-512 hash.
    #[display("{_0}")]
    InvalidSha512Hash(<Nix32Digest<64> as FromStr>::Err),

    /// The string did not contain a `:` character separating the hash algorithm
    /// from the hash value.
    #[display("missing ':' separating hash algorithm from hash value")]
    MissingColon,
}

impl fmt::Display for NixHash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Blake3(hash) => write!(f, "blake3:{hash}"),
            Self::Md5(hash) => write!(f, "md5:{hash}"),
            Self::Sha1(hash) => write!(f, "sha1:{hash}"),
            Self::Sha256(hash) => write!(f, "sha256:{hash}"),
            Self::Sha512(hash) => write!(f, "sha512:{hash}"),
        }
    }
}

impl FromStr for NixHash {
    type Err = NixHashFromStrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (algorithm_str, hash_str) =
            s.split_once(':').ok_or(NixHashFromStrError::MissingColon)?;

        match algorithm_str {
            "sha256" => hash_str
                .parse()
                .map(Self::Sha256)
                .map_err(NixHashFromStrError::InvalidSha256Hash),
            "sha512" => hash_str
                .parse()
                .map(Self::Sha512)
                .map_err(NixHashFromStrError::InvalidSha512Hash),
            "sha1" => hash_str
                .parse()
                .map(Self::Sha1)
                .map_err(NixHashFromStrError::InvalidSha1Hash),
            "md5" => hash_str
                .parse()
                .map(Self::Md5)
                .map_err(NixHashFromStrError::InvalidMd5Hash),
            "blake3" => hash_str
                .parse()
                .map(Self::Blake3)
                .map_err(NixHashFromStrError::InvalidBlake3Hash),
            _ => Err(NixHashFromStrError::InvalidHashAlgorithm),
        }
    }
}

#[cfg(test)]
mod tests {
    use alloc::string::ToString;

    use super::*;

    #[test]
    fn nix_hash_display_from_str_roundtrip() {
        let hashes = [
            NixHash::Md5(Nix32Digest::new([1; 16])),
            NixHash::Sha1(Nix32Digest::new([2; 20])),
            NixHash::Sha256(Nix32Digest::new([3; 32])),
            NixHash::Sha512(Nix32Digest::new([4; 64])),
            NixHash::Blake3(Nix32Digest::new([5; 32])),
        ];

        for hash in hashes {
            assert_eq!(hash.to_string().parse::<NixHash>().unwrap(), hash);
        }
    }

    #[test]
    fn nix_hash_from_str_unknown_algorithm() {
        assert_eq!(
            "sha384:0000".parse::<NixHash>(),
            Err(NixHashFromStrError::InvalidHashAlgorithm)
        );
    }
}
