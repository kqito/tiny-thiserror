extern crate proc_macro;
extern crate syn;

use proc_macro2::TokenStream;
use syn::{Data, DataEnum, DataStruct, DeriveInput, Result};

pub fn expand(input: &DeriveInput) -> TokenStream {
    match try_expand(input) {
        Ok(tokens) => tokens,
        Err(_) => fallback(input),
    }
}

fn try_expand(input: &DeriveInput) -> Result<TokenStream> {
    let input = input.clone();

    match input.data {
        Data::Enum(data) => Ok(parse_enum(data)),
        Data::Struct(data) => Ok(parse_struct(data)),
        // Nothing else
        _ => Err(syn::Error::new_spanned(
            input,
            "Only enums and structs are supported for #[derive(Error)]",
        )),
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

fn parse_enum(input: DataEnum) -> TokenStream {
    todo!()
}

fn parse_struct(input: DataStruct) -> TokenStream {
    todo!()
}
