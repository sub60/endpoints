//! TODO: docs.

mod path_params;
mod response;

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

/// Derives `Response` for an enum.
///
/// With the `axum` feature, it also implements axum's `IntoResponse` for the
/// enums whose variants all hold types implementing it, or have a
/// `#[status(..)]`.
#[proc_macro_derive(Response, attributes(status))]
pub fn response(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);
    response::expand(input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
