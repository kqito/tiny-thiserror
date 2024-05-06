extern crate proc_macro;
extern crate syn;
use crate::{
    ast::Enum,
    attr::{extract_error_formatter, DyamicField, Formatter},
};
use proc_macro2::TokenStream;
use syn::Ident;

struct ParseContext {
    name: Ident,
    ident: Ident,
    maybe_formatter: Option<Formatter>,
}

fn parse_named_fields(fields: &syn::FieldsNamed, context: ParseContext) -> TokenStream {
    let name = context.name;
    let ident = context.ident;
    let maybe_formatter = context.maybe_formatter;

    let field_name = fields
        .named
        .iter()
        .map(|field| field.ident.as_ref().unwrap())
        .map(|ident| format_ident!("{}", ident))
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
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>();

            quote! {
                #name::#ident {#(#field_name),*} => {
                    write!(f, #format, #(#format_field_name),*)
                }
            }
        }
        None => {
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
    }
}

fn parse_unnamed_fields(fields: &syn::FieldsUnnamed, context: ParseContext) -> TokenStream {
    let name = context.name;
    let ident = context.ident;
    let maybe_formatter = context.maybe_formatter;

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
                    DyamicField::Unnamed(unnamed) => {
                        format_ident!("{}", field_name[unnamed.ident])
                    }
                    _ => unreachable!(),
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

fn parse_unit_fields(context: ParseContext) -> TokenStream {
    let name = context.name;
    let ident = context.ident;
    let maybe_formatter = context.maybe_formatter;

    match maybe_formatter {
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
    }
}

pub fn parse_enum(input: Enum) -> TokenStream {
    let name = input.ident;
    let arms = input.variants.iter().map(|variant| {
        let ident = &variant.ident;
        let maybe_formatter = extract_error_formatter(&variant.attrs);

        let context = ParseContext {
            name: name.clone(),
            ident: ident.clone(),
            maybe_formatter,
        };

        match &variant.fields {
            syn::Fields::Named(fields) => parse_named_fields(fields, context),
            syn::Fields::Unnamed(fields) => parse_unnamed_fields(fields, context),
            syn::Fields::Unit => parse_unit_fields(context),
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
