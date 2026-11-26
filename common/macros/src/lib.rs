//! TODO: docs.

mod path_params;
mod response;
mod statuses;

use proc_macro::TokenStream;
use syn::parse_macro_input;

/// Derives `PathParams` from a `#[route(..)]` attribute.
///
/// The attribute spells out the path as a sum of segments, each either a
/// type implementing `PathParams` or a string literal, e.g.
/// `#[route(CachePath + "/nar/{nar_file_name}")]`. The struct's fields must
/// follow the segments in order:
///
/// - a type takes the next field, which must be of that type;
/// - a string literal takes one field per `{name}` in it, which must be called
///   `name`, and whose type must implement `Display` and `FromStr`.
///
/// With the `axum` feature, it also implements axum's `FromRequestParts`, so
/// the struct can be extracted from requests directly. The expanded code
/// reaches axum through `sub60-endpoints-common`, so that crate's `axum`
/// feature must be enabled too, which is what enabling it does.
///
/// The expanded code names `sub60-endpoints-common` by its crate name, so it
/// only compiles in crates that depend on it directly.
#[proc_macro_derive(PathParams, attributes(route))]
pub fn path_params(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);
    path_params::expand(input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

/// Derives `response::Alternatives` for an enum, with one alternative per
/// variant.
///
/// Each variant holds a single response, e.g. `Found(Ok<Json<Cache>>)`, and its
/// doc comment describes that response.
#[proc_macro_derive(Alternatives)]
pub fn alternatives(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);
    response::expand(input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

/// Derives `response::Statuses` for an enum.
///
/// Each variant holds a single type implementing `Status`, e.g.
/// `Internal(error::Internal)` or `NameTaken(status::Conflict)`, and its doc
/// comment describes its response.
#[proc_macro_derive(Statuses)]
pub fn statuses(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);
    statuses::expand(input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
