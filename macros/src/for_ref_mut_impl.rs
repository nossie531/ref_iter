//! Provider of [`for_ref_mut_impl`].

use crate::ForTree;
use proc_macro2::TokenStream;
use quote::quote;
use syn::parse::{ParseStream, Parser};
use syn::{Error, Result};

/// [`for_ref_mut`](crate::for_ref_mut!) macro implementation.
pub fn for_ref_mut_impl(input: TokenStream) -> TokenStream {
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
        let mut iter = IntoRefMutIterator::into_ref_mut_iter(#iter);
        loop {
            match RefMutIterator::next_mut(&mut iter) {
                None => break,
                Some(#item) => #body
            }
        }
    }};

    Ok(output)
}
