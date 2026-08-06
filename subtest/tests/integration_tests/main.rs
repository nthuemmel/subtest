//! Integration tests for the `subtest` macro.
//!
//! * [`expand`] holds the fixtures in `tests/integration_tests/expand/`, which are both
//!   snapshot-tested for their macro expansion and compiled and run as ordinary tests
//! * [`run`] observes actual test runs of those fixtures: a failing one has to be reported as a
//!   failure, a passing one as a success

mod expand;
mod run;
