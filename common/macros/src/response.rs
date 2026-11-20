use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Data, DeriveInput, Fields, parse_quote};

#[expect(clippy::too_many_lines)]
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

    // The type of each variant's response, and the arm matching the variant
    // in `status()`.
    let mut types = Vec::<syn::Type>::new();
    let mut arms = Vec::new();

    for variant in &data.variants {
        let ident = &variant.ident;

        let mut statuses =
            variant.attrs.iter().filter(|attr| attr.path().is_ident("status"));

        let status = statuses.next();

        if let Some(attr) = statuses.next() {
            return Err(syn::Error::new_spanned(
                attr,
                "a variant can only have one `#[status(..)]`",
            ));
        }

        match (&variant.fields, status) {
            (Fields::Unnamed(fields), None) if fields.unnamed.len() == 1 => {
                types.push(fields.unnamed[0].ty.clone());
                arms.push(quote! {
                    Self::#ident(response) => {
                        #krate::Response::status(response)
                    }
                });
            },
            (Fields::Unnamed(fields), Some(attr))
                if fields.unnamed.len() == 1 =>
            {
                return Err(syn::Error::new_spanned(
                    attr,
                    "a variant holding a type takes its status code from the \
                     type",
                ));
            },
            (_, Some(attr)) => {
                let status = attr.parse_args::<syn::Ident>()?;
                let status = quote!(#krate::__private::StatusCode::#status);
                types.push(parse_quote! {
                    #krate::WithStatus<{ #status.as_u16() }>
                });
                arms.push(quote!(Self::#ident { .. } => #status));
            },
            (_, None) => {
                return Err(syn::Error::new_spanned(
                    variant,
                    "each variant must either hold a single type implementing \
                     `Response`, e.g. `Found(Ok<NarBody>)`, or have a \
                     `#[status(..)]`, e.g. `#[status(CONFLICT)]`",
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

    let indices = 0..types.len();

    Ok(quote! {
        impl #impl_generics #krate::Response for #name #ty_generics
            #where_clause
        {
            #[inline]
            fn status(&self) -> #krate::__private::StatusCode {
                match self {
                    #(#arms,)*
                }
            }

            #[inline]
            fn visit<V: #krate::ResponseVisitor>(visitor: &mut V) {
                #(<#types as #krate::Response>::visit(visitor);)*
            }

            #[inline]
            fn visit_variants<V: #krate::__private::VariantVisitor>(
                visitor: &mut V,
            ) {
                #(#krate::__private::VariantVisitor::visit::<
                    #types,
                    #indices,
                >(visitor);)*
            }
        }
    })
}
