//! Provider of [`ForRef`].

use crate::tree::ForRefKind;
use crate::util::ra_friendly::{errors, Parser, SoftResult};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::token::In;
use syn::{Expr, Pat};

/// Syntax node of for-in loop macro.
pub struct ForRef {
    /// Mutable loop flag.
    kind: ForRefKind,
    /// Loop item pattern.
    item: Pat,
    /// Loop iterator expression.
    iter: Expr,
    /// Loop body tokens.
    body: TokenStream,
}

impl ForRef {
    /// Parse input and create this instance.
    pub fn parse(input: ParseStream, kind: ForRefKind) -> SoftResult<Self> {
        let parser = Parser::new(input);
        let item = parser.parse_pat(Pat::parse_single);
        let r#in = parser.parse_to(In::parse);
        let iter = parser.parse_expr(Expr::parse_without_eager_brace);
        let body = parser.parse_block();
        let errs = errors::collect_errs([&item, &r#in, &iter, &body]);

        if !errs.is_empty() {
            let err = errors::combine_errs(errs).unwrap();
            let item = item.to_token_stream();
            let iter = iter.to_token_stream();
            let body = body.to_token_stream();
            let alt = Self::out(kind, item, iter, body);
            parser.parse_to_end();
            return SoftResult::new(Err(err)).set_alt(alt);
        }

        SoftResult::new(Ok(Self {
            kind,
            item: item.value(),
            iter: iter.value(),
            body: body.value(),
        }))
    }

    /// Create token stream from components.
    fn out(kind: ForRefKind, item: TokenStream, iter: TokenStream, body: TokenStream) -> TokenStream {
        let ns = quote! {ref_iter::macros::private};
        let loop_fn = match (kind.mutable(), kind.for_map()) {
            (false, false) => quote! {#ns::exec_for_ref},
            (false, true) => quote! {#ns::exec_for_ref_kv},
            (true, false) => quote! {#ns::exec_for_ref_mut},
            (true, true) => quote! {#ns::exec_for_ref_mut_kv},
        };

        quote! {{
            #loop_fn(#iter, |x| {
                let #item = x;
                #body;
            });
        }}
    }
}

impl ToTokens for ForRef {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let kind = self.kind;
        let item = self.item.to_token_stream();
        let iter = self.iter.to_token_stream();
        let body = self.body.to_token_stream();
        let output = Self::out(kind, item, iter, body);
        tokens.extend(output);
    }
}
