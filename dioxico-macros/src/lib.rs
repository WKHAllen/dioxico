//! Macros for Dioxico.

#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![warn(unused_mut)]
#![warn(clippy::missing_docs_in_private_items)]
#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::cargo)]
#![allow(clippy::wildcard_imports)]
#![allow(clippy::if_not_else)]
#![allow(clippy::ignored_unit_patterns)]
#![allow(clippy::needless_borrows_for_generic_args)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::multiple_crate_versions)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::uninlined_format_args)]
#![allow(clippy::option_if_let_else)]

use convert_case::ccase;
use proc_macro::TokenStream;
use proc_macro_crate::{FoundCrate, crate_name};
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use std::env;
use syn::{Fields, ItemEnum, LitStr, parse_macro_input};

/// Returns a token stream that references the given crate name.
fn get_dioxico_crate_name() -> TokenStream2 {
    match (
        crate_name("dioxico"),
        env::var("CARGO_CRATE_NAME").as_deref(),
    ) {
        (Ok(FoundCrate::Itself), Ok("dioxico")) => quote! { crate },
        (Ok(FoundCrate::Name(name)), _) => {
            let crate_ident = format_ident!("{}", name);
            quote! { ::#crate_ident }
        }
        _ => quote! { ::dioxico },
    }
}

/// Derives the `UnitEnum` trait for an enum, enabling it to be used to
/// represent the state of a select dropdown component.
#[proc_macro_derive(UnitEnum)]
pub fn derive_unit_enum(input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as ItemEnum);

    let variants = match item
        .variants
        .iter()
        .map(|variant| match variant.fields {
            Fields::Unit => Ok((
                variant.ident.clone(),
                LitStr::new(&variant.ident.to_string(), variant.ident.span()),
            )),
            Fields::Named(_) | Fields::Unnamed(_) => {
                Err((variant.fields.clone(), "enum must have only unit variants"))
            }
        })
        .collect::<Result<Vec<_>, _>>()
    {
        Ok(variants) => variants,
        Err((tokens, message)) => {
            return syn::Error::new_spanned(tokens, message)
                .into_compile_error()
                .into();
        }
    };

    if !item.generics.params.is_empty() {
        return syn::Error::new_spanned(item.generics, "unit enum cannot have generics")
            .into_compile_error()
            .into();
    }

    let ident = item.ident;

    let variant_names = variants.iter().map(|(_, lit)| lit);

    let to_name_impl = variants.iter().map(|(ident, lit)| {
        quote! {
            Self::#ident => #lit,
        }
    });

    let from_name_impl = variants.iter().map(|(ident, lit)| {
        quote! {
            #lit => ::core::option::Option::Some(Self::#ident),
        }
    });

    let crate_name = get_dioxico_crate_name();

    quote! {
        impl #crate_name::UnitEnum for #ident {
            const VARIANT_NAMES: &[&'static str] = &[#(#variant_names),*];

            fn variant_name(&self) -> &'static str {
                match self {
                    #(#to_name_impl)*
                }
            }

            fn from_variant_name(name: &str) -> ::core::option::Option<Self> {
                match name {
                    #(#from_name_impl)*
                    _ => None,
                }
            }
        }
    }
    .into()
}

/// Derives the `CssRepr` trait for an enum, enabling it to be used as part of a
/// CSS class name.
#[proc_macro_derive(CssRepr)]
pub fn derive_css_repr(input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as ItemEnum);

    let variant_impls = item
        .variants
        .iter()
        .map(|variant| {
            let variant_ident = &variant.ident;
            let variant_match = match &variant.fields {
                Fields::Unit => quote! { Self::#variant_ident },
                Fields::Unnamed(_) => quote! { Self::#variant_ident(..) },
                Fields::Named(_) => quote! { Self::variant_ident { .. } },
            };
            let variant_str = ccase!(kebab, variant_ident.to_string());

            quote! {
                #variant_match => #variant_str,
            }
        })
        .collect::<Vec<_>>();

    let ident = item.ident;

    let crate_name = get_dioxico_crate_name();

    quote! {
        impl #crate_name::CssRepr for #ident {
            fn css_repr(&self) -> &'static str {
                match self {
                    #(#variant_impls)*
                }
            }
        }
    }
    .into()
}
