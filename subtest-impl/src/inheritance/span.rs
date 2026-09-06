use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse::Parse;

pub trait Spanned {
    fn mark_as_macro_generated(self) -> Self;
}

impl<T> Spanned for T
where
    T: ToTokens + Parse,
{
    fn mark_as_macro_generated(self) -> Self {
        todo!()
    }
}

fn mark_tokens_as_macro_generated(tokens: TokenStream) -> TokenStream {
    todo!()
}
