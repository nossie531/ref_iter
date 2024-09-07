use super::ScanResult;
use proc_macro2::{TokenStream, TokenTree};
use syn::parse::{ParseBuffer, ParseStream, Peek};
use syn::Result;

pub trait ParseBufferExt {
    fn scan<T, P: Peek>(
        &self,
        parse: fn(_: ParseStream<'_>) -> Result<T>,
        until: P,
    ) -> ScanResult<T>;
}

impl ParseBufferExt for ParseBuffer<'_> {
    fn scan<T, P: Peek>(
        &self,
        parse: fn(_: ParseStream<'_>) -> Result<T>,
        before: P,
    ) -> ScanResult<T> {
        let fork = self.fork();
        let target = self.call(parse);
        let peek = self.peek(before);
        match (target, peek) {
            (Ok(x), _) => ScanResult::Ok(x),
            (Err(_), true) => ScanResult::Parsing(collect(&fork, before)),
            (Err(_), false) => ScanResult::Error,
        }
    }
}

fn collect<P: Peek>(input: ParseStream, before: P) -> TokenStream {
    let mut result = TokenStream::new();
    while let Some(tt) = next_tt(input, before) {
        result.extend([tt]);
    }
    result
}

fn next_tt<P: Peek>(input: ParseStream, before: P) -> Option<TokenTree> {
    if input.peek(before) {
        None
    } else {
        input.parse::<TokenTree>().ok()
    }
}
