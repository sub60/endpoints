use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Data, DeriveInput, Fields};

#[expect(clippy::too_many_lines)]
pub(crate) fn expand(input: DeriveInput) -> syn::Result<TokenStream> {
    let Data::Enum(data) = &input.data else {
        return Err(syn::Error::new(
            Span::call_site(),
            "`Alternatives` can only be derived for enums",
        ));
    };

    if data.variants.is_empty() {
        return Err(syn::Error::new(
            Span::call_site(),
            "`Alternatives` can't be derived for an enum without variants",
        ));
    }

    let krate = quote!(::sub60_endpoints_common);

    let count = data.variants.len();

    let mut owned_types = Vec::new();
    let mut borrowed_types = Vec::new();
    let mut as_arms = Vec::new();
    let mut from_arms = Vec::new();
    let mut descriptions = Vec::new();

    for (index, variant) in data.variants.iter().enumerate() {
        let ident = &variant.ident;

        let ty = match &variant.fields {
            Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
                &fields.unnamed[0].ty
            },
            _ => {
                return Err(syn::Error::new_spanned(
                    variant,
                    "each variant must hold a single type implementing \
                     `Response`, e.g. `Found(Ok<NarBody>)`",
                ));
            },
        };

        let description = doc(&variant.attrs);

        let (owned, pattern) = match &description {
            Some(_) => (
                quote!(#krate::__private::Described<Self, #index, #ty>),
                quote!(#krate::__private::Described(response, _)),
            ),
            None => (quote!(#ty), quote!(response)),
        };

        let as_alternative = nest(
            index,
            count,
            quote!(#krate::__private::Borrowed(response)),
            &krate,
        );

        let pattern = nest(index, count, pattern, &krate);

        owned_types.push(owned);
        borrowed_types.push(quote!(#krate::__private::Borrowed<'__a, #ty>));
        as_arms.push(quote!(Self::#ident(response) => #as_alternative));
        from_arms.push(quote!(#pattern => Self::#ident(response)));
        descriptions.push(match description {
            Some(description) => {
                quote!(::core::option::Option::Some(#description))
            },
            None => quote!(::core::option::Option::None),
        });
    }

    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) =
        input.generics.split_for_impl();
    let owned = either(owned_types, &krate);
    let borrowed = either(borrowed_types, &krate);

    Ok(quote! {
        impl #impl_generics #krate::response::Alternatives
            for #name #ty_generics
            #where_clause
        {
            type Owned = #owned;

            type Ref<'__a> = #borrowed
            where
                Self: '__a;

            #[inline]
            fn as_alternative(&self) -> Self::Ref<'_> {
                match self {
                    #(#as_arms,)*
                }
            }

            #[inline]
            fn from_alternative(alternative: Self::Owned) -> Self {
                match alternative {
                    #(#from_arms,)*
                }
            }
        }

        impl #impl_generics #krate::__private::Descriptions
            for #name #ty_generics
            #where_clause
        {
            const DESCRIPTIONS: &'static [
                ::core::option::Option<&'static str>
            ] = &[#(#descriptions),*];
        }
    })
}

/// Returns the type of the alternatives `types`, nested to the right in
/// `Either`s.
fn either(types: Vec<TokenStream>, krate: &TokenStream) -> TokenStream {
    let mut types = types.into_iter().rev();
    let last = types.next().expect("the enum has variants");
    types
        .fold(last, |nested, ty| quote!(#krate::response::Either<#ty, #nested>))
}

/// Returns the expression, or the pattern, of `inner` as the `index`th of
/// `count` alternatives nested to the right in `Either`s.
fn nest(
    index: usize,
    count: usize,
    inner: TokenStream,
    krate: &TokenStream,
) -> TokenStream {
    let either = quote!(#krate::response::Either);

    let mut nested =
        if index + 1 < count { quote!(#either::Left(#inner)) } else { inner };

    for _ in 0..index {
        nested = quote!(#either::Right(#nested));
    }

    nested
}

/// Returns the text of the doc comment in `attrs`, or `None` if there's none.
pub(crate) fn doc(attrs: &[syn::Attribute]) -> Option<String> {
    let mut lines = Vec::new();

    for attr in attrs.iter().filter(|attr| attr.path().is_ident("doc")) {
        if let syn::Meta::NameValue(meta) = &attr.meta
            && let syn::Expr::Lit(syn::ExprLit {
                lit: syn::Lit::Str(line), ..
            }) = &meta.value
        {
            let line = line.value();
            lines.push(line.strip_prefix(' ').unwrap_or(&line).to_owned());
        }
    }

    let doc = lines.join("\n");
    let doc = doc.trim();
    (!doc.is_empty()).then(|| doc.to_owned())
}
