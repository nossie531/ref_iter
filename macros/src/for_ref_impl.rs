//! Provider of [`for_ref_impl`].

use crate::ForTree;
use proc_macro2::TokenStream;
use quote::quote;
use syn::parse::{ParseStream, Parser};
use syn::{Error, Result};

/// [`for_ref`](crate::for_ref!) macro implementation.
pub fn for_ref_impl(input: TokenStream) -> TokenStream {
    parse
        .parse2(input)
        .unwrap_or_else(Error::into_compile_error)
}

/// Parse inputs.
fn parse(input: ParseStream) -> Result<TokenStream> {
    let compo = ForTree::parse(input)?;
    let item = compo.item;
    let iter = compo.iter;
    let body = compo.body;
    let output = quote! {{
        use ref_iter::prelude::*;
        let mut iter = IntoRefIterator::into_ref_iter(#iter);
        loop {
            match RefIterator::next(&mut iter) {
                None => break,
                Some(#item) => #body
            }
        }
    }};

    Ok(output)
}
