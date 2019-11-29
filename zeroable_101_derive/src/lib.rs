//! An implementation detail of `zeroable_101`.
#![deny(unreachable_patterns)]
#![deny(unused_doc_comments)]
#![deny(unconditional_recursion)]

extern crate proc_macro;

mod datastructure;
mod zeroable_macro;

use proc_macro::TokenStream as TokenStream1;
use proc_macro2::TokenStream as TokenStream2;

#[proc_macro_derive(Zeroable, attributes(zero))]
pub fn derive_zeroable(input: TokenStream1) -> TokenStream1 {
    parse_or_compile_err(input, zeroable_macro::derive).into()
}

////////////////////////////////////////////////////////////////////////////////

fn parse_or_compile_err<P, F>(input: TokenStream1, f: F) -> TokenStream2
where
    P: syn::parse::Parse,
    F: FnOnce(P) -> Result<TokenStream2, syn::Error>,
{
    syn::parse::<P>(input)
        .and_then(f)
        .unwrap_or_else(|e| e.to_compile_error())
}
