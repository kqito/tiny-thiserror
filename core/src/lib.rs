extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

mod expand;

use expand::expand;
use proc_macro::TokenStream;

#[proc_macro_derive(Error, attributes(error))]
pub fn derive_tiny_error(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    expand(&input).into()
}
