use std::fmt::Display;

use quote::ToTokens;

///////////////////////////////////////////////////////////////////////////////

pub(crate) fn spanned_err(tokens: &dyn ToTokens, display: &dyn Display) -> syn::Error {
    syn::Error::new_spanned(tokens, display)
}
