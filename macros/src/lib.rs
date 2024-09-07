//! Procedual macros for crate `ref_iter`.
//!
//! *The author of this crate is not good at English.*
//! *Forgive me if the document is hard to read.*

#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]

mod raf_macro_util;
mod syn_node;

use proc_macro as pm;
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse::{ParseStream, Parser};
use syn::{Error, Result};
use syn_node::ForRef;

/// Immutable for-in loop macro.
#[proc_macro]
pub fn for_ref(input: pm::TokenStream) -> pm::TokenStream {
    let ret = parse_for_ref
        .parse2(input.into())
        .unwrap_or_else(Error::into_compile_error);
    ret.into()
}

/// Mutable for-in loop macro.
#[proc_macro]
pub fn for_ref_mut(input: pm::TokenStream) -> pm::TokenStream {
    let ret = parse_for_ref_mut
        .parse2(input.into())
        .unwrap_or_else(Error::into_compile_error);
    ret.into()
}

/// Parse immutable for-in loop macro.
fn parse_for_ref(input: ParseStream) -> Result<TokenStream> {
    parse_for_loop(input, false)
}

/// Parse mutable for-in loop macro.
fn parse_for_ref_mut(input: ParseStream) -> Result<TokenStream> {
    parse_for_loop(input, true)
}

/// Parse for-in loop macro.
fn parse_for_loop(input: ParseStream, mutable: bool) -> Result<TokenStream> {
    let mut ret = TokenStream::new();
    let syn_for = ForRef::parse(input, mutable);
    ret.extend(syn_for.to_token_stream());
    ret.extend(syn_for.err().map(Error::to_compile_error));
    Ok(syn_for.to_token_stream())
}
