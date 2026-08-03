/// Test that all fixtures in the directory 'expands' expand to an expected output snapshot when
/// thrown at the subtest macro. Uses macrotest for snapshot management and assertions.
#[test]
pub fn test_snapshots() {
    let pwd = std::env::current_dir().unwrap();
    let pwd = pwd.to_str().unwrap();

    // without the '--tests' flag, `cargo expand` would just remove functions annotated with #[test]
    // without the '--remap-path-prefix', absolute paths to the input files would show up in the generated output snapshots
    macrotest::expand_without_refresh_args(
        "tests/expand/*.rs",
        [
            "--tests".to_string(),
            "--config".to_string(),
            format!("build.rustflags=[\"--remap-path-prefix={pwd}=.\"]"),
        ],
    );
}

/// Also actually run the tests for which we do snapshot testing above
mod expand {
    mod async_tests;
    mod doc_comments;
    #[expect(
        dead_code,
        reason = "since #[test] is missing, the function should lead to a dead code warning, \
                  like normal test functions which you forgot to annotate with #[test]"
    )]
    mod missing_test_attr;
    mod no_subtests;
    mod readme_ambiguous_assert_import_solution1;
    mod readme_ambiguous_assert_import_solution2;
    mod readme_async_example;
    mod readme_inheritance_example;
    mod readme_todo_list_example;
    mod readme_unused_variables_solution;
    mod rstest_tests;
    mod two_subtests;
}
