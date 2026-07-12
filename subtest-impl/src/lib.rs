use proc_macro2::TokenStream;
use quote::quote;
use syn::ItemFn;

pub fn expand_subtest_main_fn(input: TokenStream) -> TokenStream {
    expand_subtest_main_fn_fallible(input).unwrap_or_else(|err| err.to_compile_error())
}

fn expand_subtest_main_fn_fallible(input: TokenStream) -> Result<TokenStream, syn::Error> {
    let input_fn: ItemFn = syn::parse2(input)?;

    Ok(quote! {
        #input_fn
    })
}
