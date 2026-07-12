use proc_macro2::TokenStream;
use quote::quote;

pub fn expand_subtest_main_fn(input: TokenStream) -> TokenStream {
    quote! {
        #input
    }
}
