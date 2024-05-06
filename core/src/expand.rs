extern crate proc_macro;
extern crate syn;
use crate::{
    ast::{Enum, Input, Struct},
    attr::{extract_error_formatter, DyamicField},
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
                let field_name = fields
                    .named
                    .iter()
                    .map(|field| field.ident.as_ref().unwrap())
                    .map(|ident| format_ident!("{}", ident))
                    .collect::<Vec<_>>();

                quote! {
                  #name::#ident {#(#field_name),*} => {
                      write!(f, "{} {{", stringify!(#ident))?;
                      #(
                          write!(f, "{}: {},", stringify!(#field_name), #field_name)?;
                      )*
                      write!(f, "}}")
                  }
                }
            }
            syn::Fields::Unnamed(fields) => {
                let field_name = (0..fields.unnamed.len())
                    .map(|index| format_ident!("field_{}", index))
                    .collect::<Vec<_>>();
                match maybe_formatter {
                    Some(formatter) => {
                        let format = formatter.format;
                        let format_field_name = formatter
                            .dynamic_fields
                            .iter()
                            .map(|field| match field {
                                DyamicField::Named(named) => {
                                    format_ident!("{}", named.ident)
                                }
                                DyamicField::Unnamed(unnamed) => {
                                    format_ident!("{}", field_name[unnamed.ident])
                                }
                            })
                            .collect::<Vec<_>>();
                        quote! {
                            #name::#ident(#(#field_name),*) => {
                                write!(f, #format, #(#format_field_name),*)
                            }
                        }
                    }
                    None => {
                        quote! {
                            #name::#ident(#(#field_name),*) => {
                                write!(f, "{}({})", stringify!(#ident), #(#field_name),*)
                            }
                        }
                    }
                }
            }
            syn::Fields::Unit => match maybe_formatter {
                Some(formatter) => {
                    let format = formatter.format;
                    quote! {
                        #name::#ident => {
                            write!(f, #format)
                        }
                    }
                }
                None => {
                    quote! {
                        #name::#ident => {
                            write!(f, "{}", stringify!(#ident))
                        }
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
fn parse_struct(_input: Struct) -> TokenStream {
    todo!()
}
