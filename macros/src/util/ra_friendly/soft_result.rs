//! Provider of [`SoftResult`].

use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{Error, Result};

/// Result with alternative token stream for error case.
pub struct SoftResult<T> {
    /// Base object.
    base: Result<T>,
    /// Altenative token stream.
    alt: Option<TokenStream>,
}

impl<T> SoftResult<T> {
    /// Creates a new value.
    pub fn new(base: Result<T>) -> Self {
        Self { base, alt: None }
    }

    /// Returns base object.
    pub fn base(&self) -> &Result<T> {
        &self.base
    }

    /// Returns error.
    pub fn err(&self) -> Option<&Error> {
        self.base().as_ref().err()
    }

    /// Returns success value.
    ///
    /// # Panics
    ///
    /// Panics if this result has error.
    pub fn value(self) -> T {
        self.base.unwrap()
    }

    /// Sets altenative token stream.
    pub fn set_alt(mut self, alt: TokenStream) -> Self {
        let ok = self.base.is_ok();
        self.alt = (!ok).then_some(alt);
        self
    }
}

impl<T: ToTokens> ToTokens for SoftResult<T> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ts = self
            .base
            .as_ref()
            .map(ToTokens::to_token_stream)
            .unwrap_or_else(|_| self.alt.to_token_stream());
        tokens.extend(ts);
    }
}
