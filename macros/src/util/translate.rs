//! Provider of [translate].

use proc_macro as pm;
use proc_macro2::TokenStream;
use syn::parse::Parser;
use syn::Error;

/// Translate macro input with parser.
pub fn translate<F>(input: pm::TokenStream, f: F) -> pm::TokenStream
where
    F: Parser<Output = TokenStream>,
{
    let input = input.into();
    let result = f.parse2(input);
    let result = result.unwrap_or_else(Error::into_compile_error);
    result.into()
}
