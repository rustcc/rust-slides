#![recursion_limit = "128"]

extern crate proc_macro;
use proc_macro::TokenStream;

mod to_css;

#[proc_macro_derive(ToCss, attributes(css))]
pub fn derive_to_css(input: TokenStream) -> TokenStream {
    to_css::derive(syn::parse_macro_input!(input)).into()
}

/*
use heck::KebabCase;
use proc_macro2::TokenStream;
use quote::quote;
use std::iter;
use synstructure::{decl_derive, Structure};

fn derive_to_css(input: Structure) -> TokenStream {
    let body = input.each_variant(|vi| {
        let bindings = vi.bindings();
        if bindings.is_empty() {
            let value = vi.ast().ident.to_string().to_kebab_case();
            return quote! {
                std::fmt::Write::write_str(dest, #value)
            };
        }
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
    });
    input.gen_impl(quote! {
        gen impl style_traits::ToCss for @Self {
            fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
            where
                W: std::fmt::Write,
            {
                match self {
                    #body
                }
            }
        }
    })
}

decl_derive!([ToCss, attributes(css)] => derive_to_css);
*/
