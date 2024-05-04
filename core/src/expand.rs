extern crate proc_macro;
extern crate syn;

use crate::{
    ast::{Enum, Input, Struct},
    attr::extract_error_formatter,
};
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

        let maybe_formatter = extract_error_formatter(&variant.attrs);

        match &variant.fields {
            syn::Fields::Named(fields) => {
                let field_name = fields.named.iter().map(|field| field.ident.as_ref().unwrap()).collect::<Vec<_>>();
                quote! { #name::#ident { #(#field_name),* } => write!(f, stringify!(#ident), " {{ #(#field_name),* }}") }
            }

             syn::Fields::Unnamed(fields) => {
                let field_name = (0..fields.unnamed.len())
                    .map(|index| format_ident!("field_{}", index))
                    .collect::<Vec<_>>();
                let write = if let Some(formatter) = maybe_formatter {
                    quote! { write!(f, #formatter, #(#field_name),*) }
                } else {
                    quote! { write!(f, "{}({})", stringify!(#ident), #(#field_name),*) }
                };
                quote! {
                    #name::#ident(#(#field_name),*) => {
                      #write
                    }
                }
            }
            syn::Fields::Unit => {
              let write = if let Some(formatter) = maybe_formatter {
                quote! { write!(f, #formatter) }
              } else {
                quote! { write!(f, "{}", stringify!(#ident)) }
              };

              quote! {
                #name::#ident => {
                  #write
                }
              }
            },
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
