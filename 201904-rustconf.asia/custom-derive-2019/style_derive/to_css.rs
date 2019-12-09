use darling::FromVariant;
use heck::KebabCase;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use std::iter;
use syn::{parse_quote, Data, DeriveInput, Fields, Ident};

pub fn derive(mut input: DeriveInput) -> TokenStream {
    let name = input.ident;
    let match_body = match input.data {
        Data::Struct(data) => derive_fields(&name, quote!(#name), data.fields, None),
        Data::Enum(data) => data
            .variants
            .into_iter()
            .flat_map(|variant| {
                let attrs = CssVariantAttrs::from_variant(&variant)
                    .expect("failed to parse variant attributes");
                let ident = variant.ident;
                derive_fields(&ident, quote!(#name::#ident), variant.fields, Some(attrs))
            })
            .collect(),
        _ => panic!("unsupported data structure"),
    };

    if !input.generics.params.is_empty() {
        let mut where_clause = input.generics.where_clause.take();
        let predicates = &mut where_clause.get_or_insert(parse_quote!(where)).predicates;
        for param in input.generics.type_params() {
            let ident = &param.ident;
            predicates.push(parse_quote!(#ident: style_traits::ToCss));
        }
        input.generics.where_clause = where_clause;
    }
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    quote! {
        impl#impl_generics style_traits::ToCss for #name#ty_generics
        #where_clause
        {
            fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
            where
                W: std::fmt::Write,
            {
                match self {
                    #match_body
                }
            }
        }
    }
}

fn derive_fields(
    ident: &Ident,
    pat: TokenStream,
    fields: Fields,
    attrs: Option<CssVariantAttrs>,
) -> TokenStream {
    match fields {
        Fields::Unit => {
            let value = attrs
                .and_then(|attrs| attrs.keyword)
                .unwrap_or_else(|| ident.to_string().to_kebab_case());
            quote! {
                #pat => std::fmt::Write::write_str(dest, #value),
            }
        }
        Fields::Unnamed(fields) => {
            let bindings = (0..fields.unnamed.len())
                .map(|i| Ident::new(&format!("v{}", i), Span::call_site()))
                .collect::<Vec<_>>();
            let body = derive_fields_body(&bindings);
            quote! {
                #pat(#(#bindings),*) => { #body }
            }
        }
        Fields::Named(fields) => {
            let field_names = fields
                .named
                .into_iter()
                .map(|field| field.ident.unwrap())
                .collect::<Vec<_>>();
            let body = derive_fields_body(&field_names);
            quote! {
                #pat { #(#field_names),* } => { #body }
            }
        }
    }
}

fn derive_fields_body(bindings: &[Ident]) -> TokenStream {
    let is_first = iter::once(true).chain(iter::repeat(false));
    quote! {
        #(
            if !#is_first {
                std::fmt::Write::write_char(dest, ' ')?;
            }
            style_traits::ToCss::to_css(#bindings, dest)?;
        )*
        Ok(())
    }
}

#[derive(Default, FromVariant)]
#[darling(attributes(css), default)]
struct CssVariantAttrs {
    keyword: Option<String>,
}
