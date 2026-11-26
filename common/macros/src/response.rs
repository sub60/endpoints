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

    let mut types = Vec::<syn::Type>::new();
    let mut arms = Vec::new();
    let mut visits = Vec::new();
    let mut into_response_arms = Vec::new();

    let mut held_types = Vec::new();

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
                held_types.push(fields.unnamed[0].ty.clone());
                arms.push(quote! {
                    Self::#ident(response) => {
                        #krate::response::Response::status(response)
                    }
                });
                into_response_arms.push(quote! {
                    Self::#ident(response) => {
                        #krate::__private::axum::response::IntoResponse
                            ::into_response(response)
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
                let ty: syn::Type = parse_quote! {
                    #krate::response::WithStatus<{ #status.as_u16() }>
                };
                arms.push(quote!(Self::#ident { .. } => #status));
                into_response_arms.push(quote! {
                    Self::#ident { .. } => {
                        <#ty as #krate::__private::axum::response::IntoResponse>
                            ::into_response(#krate::response::WithStatus(()))
                    }
                });
                types.push(ty);
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

        let ty = types.last().expect("the variant's type was just pushed");

        visits.push(match doc(&variant.attrs) {
            Some(description) => quote! {
                <#ty as #krate::response::Response>::visit(
                    &mut #krate::__private::WithDescription {
                        visitor: &mut *visitor,
                        description: #description,
                    },
                );
            },
            None => {
                quote!(<#ty as #krate::response::Response>::visit(visitor);)
            },
        });
    }

    let name = &input.ident;
    let mut generics = input.generics.clone();

    if !generics.params.is_empty() {
        let where_clause = generics.make_where_clause();
        for ty in &types {
            where_clause
                .predicates
                .push(parse_quote!(#ty: #krate::response::Response));
        }
    }

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let indices = 0..types.len();

    let into_response_impl = if cfg!(feature = "axum") {
        let mut generics = input.generics.clone();
        let where_clause = generics.make_where_clause();

        // The bounds are higher-ranked because a bound on a concrete type that
        // doesn't hold is a compile error, and the types held by an error
        // enum's variants often don't implement `IntoResponse`.
        for ty in &held_types {
            where_clause.predicates.push(parse_quote! {
                for<'__r> #ty: #krate::__private::axum::response::IntoResponse
            });
        }

        let (impl_generics, ty_generics, where_clause) =
            generics.split_for_impl();

        quote! {
            impl #impl_generics #krate::__private::axum::response::IntoResponse
                for #name #ty_generics
                #where_clause
            {
                #[inline]
                fn into_response(
                    self,
                ) -> #krate::__private::axum::response::Response {
                    match self {
                        #(#into_response_arms,)*
                    }
                }
            }
        }
    } else {
        TokenStream::new()
    };

    Ok(quote! {
        #into_response_impl

        impl #impl_generics #krate::response::Response for #name #ty_generics
            #where_clause
        {
            #[inline]
            fn status(&self) -> #krate::__private::StatusCode {
                match self {
                    #(#arms,)*
                }
            }

            #[inline]
            fn visit<V: #krate::response::ResponseVisitor>(visitor: &mut V) {
                #(#visits)*
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

/// Returns the text of the doc comment in `attrs`, or `None` if there's none.
fn doc(attrs: &[syn::Attribute]) -> Option<String> {
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
