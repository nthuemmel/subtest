use proc_macro2::TokenStream;
use subtest_impl::expand_subtest_main_fn;

/// Test that all fixtures in the directory 'expands' expand to an expected output snapshot when
/// thrown at the subtest macro. Uses cargo insta for snapshot management and assertions.
#[test]
fn test_snapshots() {
    insta::glob!("expand_impl/*.rs", |path| {
        let macro_input = std::fs::read_to_string(path).unwrap();
        let macro_input_without_attr = macro_input.trim_start_matches("#[subtest]");

        if macro_input_without_attr == &macro_input {
            panic!("Fixture at {path:?} must start with #[subtest]");
        }

        let macro_input_without_attr =
            syn::parse_str(&macro_input_without_attr).expect("macro input must be valid Rust");

        // Runs the actual macro:
        let macro_output = expand_subtest_main_fn(TokenStream::new(), macro_input_without_attr);

        let macro_output = syn::parse2(macro_output).expect("macro output must be valid Rust");
        let macro_output = prettyplease::unparse(&macro_output);
        insta::assert_snapshot!(macro_output);
    });
}
