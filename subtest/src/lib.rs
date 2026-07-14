use proc_macro::TokenStream;
use subtest_impl::expand_subtest_main_fn;

#[proc_macro_attribute]
pub fn subtest(args: TokenStream, input: TokenStream) -> TokenStream {
    expand_subtest_main_fn(args.into(), input.into()).into()
}
