use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemFn};
use subtest_impl::expand_subtest;

#[proc_macro_attribute]
pub fn subtest(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemFn);
    let output = expand_subtest(input);
    output.into()
}
