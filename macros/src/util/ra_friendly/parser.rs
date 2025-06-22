//! Provider of [`Parser`].

use crate::util::ra_friendly::SoftResult;
use proc_macro2::{Delimiter, Group, TokenStream, TokenTree};
use quote::{ToTokens, TokenStreamExt, quote};
use syn::parse::{Parse, ParseBuffer, ParseStream};
use syn::{Block, Expr, Pat, Result};

/// Rust analyzer friendly parser.
pub struct Parser<'a> {
    /// Base object.
    base: &'a ParseBuffer<'a>,
}

impl<'a> Parser<'a> {
    /// Creates a new value.
    pub fn new(base: &'a ParseBuffer<'a>) -> Self {
        Self { base }
    }

    /// Returns base object.
    pub fn base(&self) -> &ParseBuffer<'_> {
        self.base
    }

    /// Proceed parser to end.
    pub fn parse_to_end(&self) {
        let input = self.base;
        while !input.is_empty() {
            let _ = input.parse::<TokenTree>();
        }
    }

    /// Proceed parser until given parser function succeeds.
    pub fn parse_to<T>(&self, f: fn(ParseStream) -> Result<T>) -> Result<T> {
        let input = self.base;
        let result = input.call(f);

        if let Err(x) = result {
            while !input.is_empty() && input.call(f).is_err() {
                let _ = input.call(TokenTree::parse);
            }

            return Result::Err(x);
        }

        result
    }

    /// Call given parser function.
    pub fn parse<T: ToTokens>(&self, f: fn(ParseStream) -> Result<T>) -> SoftResult<T> {
        let result = self.base().call(f);
        SoftResult::new(result)
    }

    /// Call given parser function for pattern.
    pub fn parse_pat(&self, f: fn(ParseStream) -> Result<Pat>) -> SoftResult<Pat> {
        self.parse(f).set_alt(quote! {_})
    }

    /// Call given parser function for expression.
    pub fn parse_expr(&self, f: fn(ParseStream) -> Result<Expr>) -> SoftResult<Expr> {
        let input = self.base;
        let begin = input.cursor();
        let syn = input.call(f);
        let end = input.cursor();
        if syn.is_ok() {
            return SoftResult::new(syn);
        }

        let mut part = TokenStream::new();
        let mut cur = begin;
        while cur < end {
            let next = cur.token_tree().unwrap();
            part.append(next.0);
            cur = next.1;
        }

        SoftResult::new(syn).set_alt(Self::trim_tail(&part))
    }

    /// Call given parser function for block.
    pub fn parse_block(&self) -> SoftResult<TokenStream> {
        let input = self.base;
        let group = input.fork().call(Group::parse);
        if !group.is_ok_and(|x| matches!(x.delimiter(), Delimiter::Brace)) {
            let err = input.call(Block::parse).err().unwrap();
            return SoftResult::new(Err(err)).set_alt(quote! {{}});
        }

        let braced = input.call(Group::parse).unwrap();
        SoftResult::new(Ok(braced.to_token_stream()))
    }

    /// Trim tokens tail puctuations.
    fn trim_tail(input: &TokenStream) -> TokenStream {
        let mut ret = TokenStream::new();
        let input = input.to_token_stream().into_iter().collect::<Vec<_>>();
        let last_idx = input
            .iter()
            .rposition(|x| !matches!(x, TokenTree::Punct(_)));

        if let Some(last_idx) = last_idx {
            ret.append_all(input.into_iter().take(last_idx + 1));
        }

        ret
    }
}
