use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Data, DeriveInput, Fields};

use crate::response::doc;

#[expect(clippy::too_many_lines)]
pub(crate) fn expand(input: DeriveInput) -> syn::Result<TokenStream> {
    let Data::Enum(data) = &input.data else {
        return Err(syn::Error::new(
            Span::call_site(),
            "`Statuses` can only be derived for enums",
        ));
    };

    let krate = quote!(::sub60_endpoints_common);

    let mut statuses = Vec::new();
    let mut arms = Vec::new();
    let mut visits = Vec::new();

    for (index, variant) in data.variants.iter().enumerate() {
        let ty = match &variant.fields {
            Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
                &fields.unnamed[0].ty
            },
            _ => {
                return Err(syn::Error::new_spanned(
                    variant,
                    "each variant must hold a single type implementing \
                     `Status`, e.g. `Internal(error::Internal)` or \
                     `NameTaken(status::Conflict)`",
                ));
            },
        };

        let ident = &variant.ident;
        let status = quote!(<#ty as #krate::response::Status>::STATUS);

        let description = match doc(&variant.attrs) {
            Some(description) => {
                quote!(::core::option::Option::Some(#description))
            },
            None => quote!(::core::option::Option::None),
        };

        arms.push(quote!(Self::#ident(_) => #status));
        visits.push(quote! {
            #krate::response::ResponseVisitor::visit::<
                #krate::__private::Variant<__B, #index>,
            >(visitor, #status, #description);
        });
        statuses.push(status);
    }

    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) =
        input.generics.split_for_impl();

    Ok(quote! {
        impl #impl_generics #krate::response::Statuses for #name #ty_generics
            #where_clause
        {
            const STATUSES: &'static [#krate::__private::StatusCode] =
                &[#(#statuses),*];

            #[inline]
            fn status(&self) -> #krate::__private::StatusCode {
                match *self {
                    #(#arms,)*
                }
            }

            #[inline]
            fn visit<
                __B: #krate::body::Body,
                __V: #krate::response::ResponseVisitor,
            >(
                visitor: &mut __V,
            ) {
                #(#visits)*
            }
        }
    })
}
