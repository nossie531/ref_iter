//! Provider of [`ForTree`].

use crate::util::{ParseBufferExt, ScanResult};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::ParseStream;
use syn::token::{Brace, In};
use syn::{braced, Expr, Pat, Result, Stmt, Token};

/// for-in loop syntax tree.
pub struct ForTree {
    /// Loop item.
    pub item: ScanResult<Pat>,
    /// Loop iterator.
    pub iter: ScanResult<Expr>,
    /// Loop body.
    pub body: TokenStream,
}

impl ForTree {
    pub fn parse(input: ParseStream) -> Result<Self> {
        let item = input.scan(Pat::parse_single, Token![in]);
        let _in = input.parse::<In>()?;
        let iter = input.scan(Expr::parse_without_eager_brace, Brace);
        let body = Self::parse_body(input)?;
        Ok(Self { item, iter, body })
    }

    fn parse_body(input: ParseStream) -> Result<TokenStream> {
        let mut result = TokenStream::new();
        let content;

        braced!(content in input);
        while !content.is_empty() {
            let orig = content.fork().cursor().token_stream();
            let stmt = content.parse::<Stmt>();
            let trns = stmt.as_ref().map_or(orig, ToTokens::into_token_stream);

            result.extend(trns);

            if stmt.is_err() {
                break;
            }
        }

        Ok(quote! {{#result}})
    }
}
