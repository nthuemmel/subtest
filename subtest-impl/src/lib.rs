use proc_macro2::TokenStream;
use quote::quote;
use syn::ItemFn;

pub fn expand_subtest(input: ItemFn) -> TokenStream {
    quote! {
        #input
    }
}
