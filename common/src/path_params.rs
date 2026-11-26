use core::fmt;
use core::str::FromStr;

use percent_encoding::{AsciiSet, NON_ALPHANUMERIC, utf8_percent_encode};
pub use sub60_endpoints_macros::PathParams;

/// The parameters in an endpoint's path, along with the path's template.
///
/// It's usually derived, with the path given by a `#[route(..)]` attribute.
pub trait PathParams: Sized {
    /// The template of the path, in which each `{name}` stands for the
    /// parameter with that name.
    const PATH: &'static str;

    /// Appends the path with the parameters filled in, each percent-encoded.
    fn write_path(&self, path: &mut String);

    /// Parses the parameters from the ones matched in a request's path, as
    /// (name, value) pairs in the order they appear in the path, with their
    /// values percent-decoded.
    ///
    /// It only takes the pairs it needs, so the ones after them are left for
    /// whatever comes next in the path.
    ///
    /// # Errors
    ///
    /// Returns an error if a pair is missing or isn't the one expected, or if
    /// a value doesn't parse.
    fn parse<'a>(
        params: &mut impl Iterator<Item = (&'a str, &'a str)>,
    ) -> Result<Self, PathParamsError>;

    /// Appends each parameter's name and schema to `schemas`, in the order
    /// they appear in the path.
    fn param_schemas(
        generator: &mut schemars::SchemaGenerator,
        schemas: &mut Vec<(&'static str, schemars::Schema)>,
    );

    /// Returns the path with the parameters filled in, each percent-encoded.
    #[inline]
    fn to_path(&self) -> String {
        let mut path = String::new();
        self.write_path(&mut path);
        path
    }
}

/// The type of error that can occur when parsing [`PathParams`].
#[derive(Debug, Clone, PartialEq, Eq, derive_more::Display, cauchy::Error)]
pub enum PathParamsError {
    /// There was no parameter where one was expected.
    #[display("missing path parameter {_0:?}")]
    Missing(&'static str),

    /// The parameter where one was expected had a different name.
    #[display("expected path parameter {expected:?}, found {found:?}")]
    UnexpectedName {
        /// The name of the expected parameter.
        expected: &'static str,
        /// The name of the parameter that was found instead.
        found: String,
    },

    /// A parameter's value didn't parse.
    #[display("invalid path parameter {name:?}: {message}")]
    Invalid {
        /// The parameter's name.
        name: &'static str,
        /// Why the value didn't parse.
        message: String,
    },
}

/// Items the derive macro expands to.
#[doc(hidden)]
pub mod __private {
    #[cfg(feature = "axum")]
    pub use ::axum;
    pub use const_format::concatcp;
    pub use http::StatusCode;
    pub use schemars;

    use super::*;
    #[cfg(feature = "axum")]
    pub use crate::axum::from_request_parts;
    pub use crate::body::Variant;
    pub use crate::response::{Borrowed, Described, Descriptions};

    /// The characters that get percent-encoded in a path parameter, i.e. all
    /// of them but the unreserved ones.
    const PATH_PARAM: &AsciiSet =
        &NON_ALPHANUMERIC.remove(b'-').remove(b'.').remove(b'_').remove(b'~');

    #[inline]
    pub fn push_param(path: &mut String, value: &impl fmt::Display) {
        path.extend(utf8_percent_encode(&value.to_string(), PATH_PARAM));
    }

    #[inline]
    pub fn parse_param<'a, T>(
        params: &mut impl Iterator<Item = (&'a str, &'a str)>,
        name: &'static str,
    ) -> Result<T, PathParamsError>
    where
        T: FromStr<Err: fmt::Display>,
    {
        let (found, value) =
            params.next().ok_or(PathParamsError::Missing(name))?;

        if found != name {
            return Err(PathParamsError::UnexpectedName {
                expected: name,
                found: found.to_owned(),
            });
        }

        value.parse().map_err(|err: T::Err| PathParamsError::Invalid {
            name,
            message: err.to_string(),
        })
    }
}
