//! Procedual macros for crate `ref_iter`.
//!
//! *The author of this crate is not good at English.*
//! *Forgive me if the document is hard to read.*

#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]

mod tree;
mod util;

use proc_macro as pm;
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse::{ParseBuffer, Parser};
use tree::ForRef;

/// Immutable for-in loop macro.
#[proc_macro]
pub fn for_ref(input: pm::TokenStream) -> pm::TokenStream {
    let parser = create_for_loop_parser(false);
    util::translate(input, parser)
}

/// Mutable for-in loop macro.
#[proc_macro]
pub fn for_ref_mut(input: pm::TokenStream) -> pm::TokenStream {
    let parser = create_for_loop_parser(true);
    util::translate(input, parser)
}

/// Create for-in loop parser.
fn create_for_loop_parser(mutable: bool) -> impl Parser<Output = TokenStream> {
    move |x: &ParseBuffer<'_>| {
        let ret = ForRef::parse(x, mutable);
        Ok(ret.to_token_stream())
    }
}
