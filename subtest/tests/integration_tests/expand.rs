/// Test that all fixtures in the directory 'expands' expand to an expected output snapshot when
/// thrown at the subtest macro. Uses macrotest for snapshot management and assertions.
#[test]
pub fn test_snapshots() {
    let pwd = std::env::current_dir().unwrap();
    let pwd = pwd.to_str().unwrap();

    // without the '--tests' flag, `cargo expand` would just remove functions annotated with #[test]
    // without the '--remap-path-prefix', absolute paths to the input files would show up in the generated output snapshots
    macrotest::expand_without_refresh_args(
        "tests/integration_tests/expand/*.rs",
        [
            "--tests".to_string(),
            "--config".to_string(),
            format!("build.rustflags=[\"--remap-path-prefix={pwd}=.\"]"),
        ],
    );
}

// Also actually run the tests for which we do snapshot testing above
mod async_tests;
mod doc_comments;
mod helper_fn;
mod helper_fn_attrs;
#[deny(
    unfulfilled_lint_expectations,
    reason = "an #[expect] is passed down to nested subtests as an #[allow], make sure that\
                  works by denying unfulfilled #[expect]s"
)]
mod inherited_lint_expectation;
#[expect(
    dead_code,
    reason = "since #[test] is missing, the function should lead to a dead code warning, \
                  like normal test functions which you forgot to annotate with #[test]"
)]
mod missing_test_attr;
#[expect(
    dead_code,
    reason = "since #[test] is missing, the function should lead to a dead code warning, \
                  like normal test functions which you forgot to annotate with #[test]"
)]
mod missing_test_attr_on_nested_subtest;
mod no_subtests;
mod readme_ambiguous_assert_import_solution1;
mod readme_ambiguous_assert_import_solution2;
mod readme_async_example;
mod readme_inheritance_example;
mod readme_inherited_ignore_example;
mod readme_inherited_ignore_solution;
mod readme_parallel_setup_solution1;
mod readme_parallel_setup_solution2;
mod readme_todo_list_example;
mod readme_unused_variables_solution;
mod rstest_tests;
#[deny(
    clippy::too_many_lines,
    reason = "the pedantic lint has to be enabled for the fixture's #[expect] attributes to \
                  be fulfilled - and a subtest must not be reported for lines it merely inherits"
)]
#[deny(
    unfulfilled_lint_expectations,
    reason = "an #[expect(clippy::too_many_lines)] on a long test function must not be \
                  inherited by a shorter subtest, where it could never be fulfilled"
)]
mod too_many_lines_expectation;
mod two_subtests;
#[deny(
    unused_assignments,
    reason = "values assigned in the parent test function must not be reported as never read \
                  in a subtest which does not read them"
)]
#[allow(
    clippy::needless_late_init,
    reason = "the fixture declares variables without a value on purpose - that is what makes \
                  the inherited statements assignments in the first place"
)]
mod unused_assignments_in_subtest;
#[deny(
    unused_variables,
    reason = "variables inherited from the parent test function must not be reported as \
                  unused in a subtest which does not use them"
)]
#[deny(
    clippy::allow_attributes_without_reason,
    reason = "the generated #[allow(unused_variables)] states no reason, which must stay \
                  acceptable in a project requiring one - clippy does not apply the lint to \
                  attributes coming out of a macro"
)]
#[deny(
    clippy::allow_attributes,
    reason = "the generated #[allow(unused_variables)] must stay acceptable in a project \
                  requiring #[expect] instead - clippy does not apply the lint to attributes \
                  coming out of a macro"
)]
mod unused_variables_in_subtest;
