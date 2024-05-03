extern crate proc_macro;
extern crate syn;

use crate::ast::{Enum, Input, Struct};
use proc_macro2::TokenStream;
use syn::{DeriveInput, Result};

pub fn expand(input: &DeriveInput) -> TokenStream {
    match try_expand(input) {
        Ok(tokens) => tokens,
        Err(_) => fallback(input),
    }
}

fn try_expand(derive_input: &DeriveInput) -> Result<TokenStream> {
    let input = Input::try_from(derive_input.clone())
        .map_err(|_| syn::Error::new_spanned(derive_input, "Unsupported type"))?;

    match input {
        Input::Enum(e) => Ok(parse_enum(e)),
        Input::Struct(s) => Ok(parse_struct(s)),
    }
}

fn fallback(input: &DeriveInput) -> TokenStream {
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let expanded = quote! {
      impl #impl_generics std::error::Error for #name #ty_generics #where_clause {}
    };

    expanded
}

fn parse_enum(input: Enum) -> TokenStream {
    let name = input.ident;
    let arms = input.variants.iter().map(|variant| {
        let ident = &variant.ident;
        quote! {
            Self::#ident => std::fmt::Display::fmt("TODO", f),
        }
    });

    quote! {
        impl std::fmt::Display for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                match self {
                    #(#arms)*
                }
            }
        }
    }
}

fn parse_struct(input: Struct) -> TokenStream {
    todo!()
}
