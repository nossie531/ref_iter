//! Procedual macros for crate `ref_iter`.
//!
//! *The author of this crate is not good at English.*
//! *Forgive me if the document is hard to read.*

#![warn(missing_docs)]

#[proc_macro]
#[allow(missing_docs)]
pub fn for_ref(input: TokenStream) -> TokenStream {
    for_ref_impl(input.into()).into()
}

#[proc_macro]
#[allow(missing_docs)]
pub fn for_ref_mut(input: TokenStream) -> TokenStream {
    for_ref_mut_impl(input.into()).into()
}

mod for_ref_impl;
mod for_ref_mut_impl;
mod for_tree;
mod util;

use for_ref_impl::*;
use for_ref_mut_impl::*;
use for_tree::*;
use proc_macro::TokenStream;
