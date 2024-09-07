use proc_macro2::TokenStream;
use quote::ToTokens;

pub enum ScanResult<T> {
    Ok(T),
    Parsing(TokenStream),
    Error,
}

impl<T> ToTokens for ScanResult<T>
where
    T: ToTokens,
{
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ts = match self {
            ScanResult::Ok(x) => x.into_token_stream(),
            ScanResult::Parsing(x) => x.into_token_stream(),
            ScanResult::Error => TokenStream::new(),
        };

        tokens.extend(ts);
    }
}
