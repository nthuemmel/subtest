//! Observe actual test runs of the fixtures, by re-executing this very test binary in a child
//! process: a test which fails has to be reported as a failure, a test which passes as a success.

/// Run a single test called `name` of this very test binary in a child process, and return whether
/// the run succeeded together with the test runner output.
/// Set `is_ignored` *only* if the test called `name` is actually `#[ignore]`d.
fn run_test_in_child_process(name: &str, is_ignored: bool) -> (bool, String) {
    let test_binary = std::env::current_exe().unwrap();
    let mut command = std::process::Command::new(test_binary);
    command.args(["--exact", name]);

    if is_ignored {
        command.arg("--ignored");
    }

    let output = command.output().unwrap();
    let report = String::from_utf8(output.stdout).unwrap();

    (output.status.success(), report)
}

/// Assert that the test with the given `name` is reported as a failure, and return the report for
/// the caller to assert on the reason.
/// The test with the given `name` MUST be `#[ignore]`d!
fn report_of_failing_test(name: &str) -> String {
    let (succeeded, report) = run_test_in_child_process(name, true);

    assert!(
        !succeeded,
        "expected the test run to fail, but it succeeded:\n{report}"
    );

    assert!(
        report.contains("test result: FAILED. 0 passed; 1 failed"),
        "{report}"
    );

    report
}

fn assert_test_is_reported_as_a_success(name: &str) {
    let (succeeded, report) = run_test_in_child_process(name, false);

    assert!(
        succeeded,
        "expected the test run to succeed, but it failed:\n{report}"
    );
    assert!(
        report.contains("test result: ok. 1 passed; 0 failed"),
        "{report}"
    );
}

#[test]
fn a_panicking_test_function_is_reported_as_a_failure() {
    let report = report_of_failing_test("expand::failing_subtest::a_panicking_test_function");

    assert!(
        report.contains("the test function itself fails"),
        "{report}"
    );
}

#[test]
fn a_panicking_subtest_is_reported_as_a_failure() {
    let report = report_of_failing_test(
        "expand::failing_subtest::a_test_function_with_a_panicking_subtest_subtests::\
         a_subtest_panicking",
    );

    assert!(
        report.contains("assertion `left == right` failed"),
        "{report}"
    );
}

#[test]
fn a_subtest_returning_an_error_is_reported_as_a_failure() {
    let report = report_of_failing_test(
        "expand::failing_subtest::a_test_function_with_a_subtest_returning_an_error_subtests::\
         a_subtest_returning_an_error",
    );

    assert!(report.contains("the list is empty"), "{report}");
}

#[test]
fn a_passing_test_function_is_reported_as_a_success() {
    assert_test_is_reported_as_a_success("expand::two_subtests::two_subtests");
}

#[test]
fn a_passing_subtest_is_reported_as_a_success() {
    assert_test_is_reported_as_a_success("expand::two_subtests::two_subtests_subtests::add");
}

/// `#[ignore]` is not inherited, so the subtest of an ignored test function has to run.
#[test]
fn a_subtest_of_an_ignored_test_function_is_not_ignored() {
    assert_test_is_reported_as_a_success(
        "expand::uninherited_test_attrs::an_ignored_test_function_subtests::\
         a_subtest_of_an_ignored_test_function",
    );
}
