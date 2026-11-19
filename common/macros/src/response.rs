use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Data, DeriveInput, Fields, parse_quote};

pub(crate) fn expand(input: DeriveInput) -> syn::Result<TokenStream> {
    let krate = quote!(::sub60_endpoints_common);

    let Data::Enum(data) = &input.data else {
        return Err(syn::Error::new(
            Span::call_site(),
            "`Response` can only be derived for enums",
        ));
    };

    if data.variants.is_empty() {
        return Err(syn::Error::new(
            Span::call_site(),
            "`Response` can't be derived for an enum without variants",
        ));
    }

    let mut variants = Vec::new();
    let mut types = Vec::new();

    for variant in &data.variants {
        match &variant.fields {
            Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
                variants.push(&variant.ident);
                types.push(&fields.unnamed[0].ty);
            },
            _ => {
                return Err(syn::Error::new_spanned(
                    variant,
                    "each variant must hold a single type implementing \
                     `Response`, e.g. `Found(Ok<NarBody>)`",
                ));
            },
        }
    }

    let name = &input.ident;
    let mut generics = input.generics.clone();

    if !generics.params.is_empty() {
        let where_clause = generics.make_where_clause();
        for ty in &types {
            where_clause.predicates.push(parse_quote!(#ty: #krate::Response));
        }
    }

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    Ok(quote! {
        impl #impl_generics #krate::Response for #name #ty_generics
            #where_clause
        {
            #[inline]
            fn status(&self) -> #krate::__private::StatusCode {
                match self {
                    #(Self::#variants(response) => {
                        #krate::Response::status(response)
                    },)*
                }
            }

            #[inline]
            fn visit<V: #krate::ResponseVisitor>(visitor: &mut V) {
                #(<#types as #krate::Response>::visit(visitor);)*
            }
        }
    })
}
