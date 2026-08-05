//! *Just continue writing tests.*
//!
//! Ever got to a point where you felt extending a test would be much easier than writing a new one?
//! But the thing you want to test is actually a different feature, and would warrant its own test?
//! You *could* copy-paste the existing test's setup code, or you *could* refactor it to a shared test fixture, but you could also...
//!
//! ... use `#[subtest]`: a Rust macro to easily use test setup code from one test function in another one.
//!
//! # Example
//!
//! **Let's say you have a TODO list app.**
//!
//! ```no_run
//! use subtest::subtest;
//!
//! #[subtest]
//! #[test]
//! fn add_creates_pending_task() {
//!     let mut list = TodoList::new();
//!     let id = list.add("Buy milk");
//!
//!     #[subtest]
//!     fn complete_marks_task_completed() {
//!         list.complete(id).unwrap();
//!         assert_eq!(list.get(id).unwrap().status, TaskStatus::Completed);
//!     }
//!
//!     #[subtest]
//!     fn cancel_marks_task_cancelled() {
//!         list.cancel(id).unwrap();
//!         assert_eq!(list.get(id).unwrap().status, TaskStatus::Cancelled);
//!     }
//!
//!     let task = list.get(id).unwrap();
//!     assert_eq!(task.status, TaskStatus::Pending);
//! }
//! ```
//!
//! ([Link to full example](https://github.com/nthuemmel/subtest/blob/master/subtest/tests/expand/readme_todo_list_example.rs))
//!
//! # Installation
//!
//! ```sh
//! cargo add --dev subtest
//! ```
//!
//! or, in your `Cargo.toml`:
//!
//! ```toml
//! [dev-dependencies]
//! subtest = "0.0.1"
//! ```
//!
//! `subtest` requires Rust 1.85 or newer.
//!
//! # How it works
//!
//! * Statements *preceding* a nested `#[subtest]` function are **copied** into the nested function's body
//! * This means you can freely use and mutate any local variables from the parent function in the nested function...
//! * ... without affecting the parent function or sibling test functions
//! * Statements *following* a nested `#[subtest]` function are **not** copied - they only run in the parent function
//! * The parent function stays a test of its own, and every subtest becomes a new test - so the setup code is run once per test (see [Setup code runs once per test, in parallel](#setup-code-runs-once-per-test-in-parallel))
//!
//! **The above example gets expanded to:**
//!
//! ```no_run
//! #[test]
//! fn add_creates_pending_task() {
//!     let mut list = TodoList::new();
//!     let id = list.add("Buy milk");
//!     let task = list.get(id).unwrap();
//!     assert_eq!(task.status, TaskStatus::Pending);
//! }
//! mod add_creates_pending_task_subtests {
//!     use super::*;
//!     #[test]
//!     fn complete_marks_task_completed() {
//! #        #[allow(unused_variables)]
//!         let mut list = TodoList::new();
//! #        #[allow(unused_variables)]
//!         let id = list.add("Buy milk");
//!         list.complete(id).unwrap();
//!         assert_eq!(list.get(id).unwrap().status, TaskStatus::Completed);
//!     }
//!     #[test]
//!     fn cancel_marks_task_cancelled() {
//! #        #[allow(unused_variables)]
//!         let mut list = TodoList::new();
//! #        #[allow(unused_variables)]
//!         let id = list.add("Buy milk");
//!         list.cancel(id).unwrap();
//!         assert_eq!(list.get(id).unwrap().status, TaskStatus::Cancelled);
//!     }
//! }
//! ```
//!
//! **and therefore runs as three tests:**
//!
//! ```text
//! running 3 tests
//! test add_creates_pending_task ... ok
//! test add_creates_pending_task_subtests::complete_marks_task_completed ... ok
//! test add_creates_pending_task_subtests::cancel_marks_task_cancelled ... ok
//! ```
//!
//! Since subtests live in a `<parent function>_subtests` module, `cargo test add_creates_pending_task` runs the parent test together with all of its subtests, while `cargo test add_creates_pending_task_subtests::complete_marks_task_completed` runs just that single subtest.
//!
//! # What you can do
//!
//! ## Arbitrarily nest test functions
//!
//! You can nest `#[subtest]` functions arbitrarily deeply:
//!
//! ```no_run
//! # use subtest::subtest;
//! #[subtest]
//! #[test]
//! fn add_creates_pending_task() {
//!     let mut list = TodoList::new();
//!     let id = list.add("Buy milk");
//!
//!     #[subtest]
//!     fn cancel_marks_task_cancelled() {
//!         list.cancel(id).unwrap();
//!         assert_eq!(list.get(id).unwrap().status, TaskStatus::Cancelled);
//!
//!         #[subtest]
//!         fn cannot_complete_already_cancelled_task() {
//!             let err = list.complete(id).unwrap_err();
//!             assert!(matches!(err, TodoError::InvalidTransition { .. }));
//!         }
//!     }
//! }
//! ```
//!
//! Each `#[subtest]` inherits the code of all parent functions, in order, as its setup code.
//!
//! <details>
//!
//! <summary>Click to see expansion</summary>
//!
//! ```no_run
//! #[test]
//! fn add_creates_pending_task() {
//!     let mut list = TodoList::new();
//!     let id = list.add("Buy milk");
//! }
//! mod add_creates_pending_task_subtests {
//!     use super::*;
//!     #[test]
//!     fn cancel_marks_task_cancelled() {
//! #        #[allow(unused_variables)]
//!         let mut list = TodoList::new();
//! #        #[allow(unused_variables)]
//!         let id = list.add("Buy milk");
//!         list.cancel(id).unwrap();
//!         assert_eq!(list.get(id).unwrap().status, TaskStatus::Cancelled);
//!     }
//!     mod cancel_marks_task_cancelled_subtests {
//!         use super::*;
//!         #[test]
//!         fn cannot_complete_already_cancelled_task() {
//! #            #[allow(unused_variables)]
//!             let mut list = TodoList::new();
//! #            #[allow(unused_variables)]
//!             let id = list.add("Buy milk");
//!             list.cancel(id).unwrap();
//!             assert_eq!(list.get(id).unwrap().status, TaskStatus::Cancelled);
//!             let err = list.complete(id).unwrap_err();
//!             assert!(matches!(err, TodoError::InvalidTransition { .. }));
//!         }
//!     }
//! }
//! ```
//!
//! </details>
//!
//! ## Use async tests
//!
//! Example:
//!
//! ```no_run
//! # use subtest::subtest;
//! #[subtest]
//! #[tokio::test]
//! async fn value_can_be_sent_async() {
//!     let (sender, receiver) = tokio::sync::mpsc::channel(5);
//!     sender.send("Hello!").await.unwrap();
//!
//!     #[subtest]
//!     async fn value_can_be_received() {
//!         let mut receiver = receiver;
//!         let value = receiver.recv().await.unwrap();
//!         assert_eq!(value, "Hello!");
//!     }
//!
//!     drop(receiver);
//! }
//! ```
//!
//! Make sure to mark nested `#[subtest]` functions `async` as well. You cannot downgrade from `async` back to sync.
//!
//! You can, however, upgrade from sync to `async`!
//!
//! <details>
//!
//! <summary>Click to show example</summary>
//!
//! ```no_run
//! # use subtest::subtest;
//! #[subtest]
//! #[test]
//! fn value_can_be_sent_sync() {
//!     let (sender, receiver) = tokio::sync::mpsc::channel(5);
//!     sender.try_send("Hello!").unwrap();
//!
//!     #[subtest(inherit_attributes = false)]
//!     #[tokio::test]
//!     async fn value_can_be_received() {
//!         let mut receiver = receiver;
//!         let value = receiver.recv().await.unwrap();
//!         assert_eq!(value, "Hello!");
//!     }
//!
//!     drop(receiver);
//! }
//! ```
//!
//! </details>
//!
//! ## Use rstest (or other testing frameworks)
//!
//! Example:
//!
//! ```no_run
//! # use subtest::subtest;
//! #[subtest]
//! #[rstest::rstest]
//! #[case::completed(TaskStatus::Completed)]
//! #[case::cancelled(TaskStatus::Cancelled)]
//! fn insert_finished_task(#[case] status: TaskStatus) {
//!     let mut list = TodoList::new();
//!     let id = 1;
//!
//!     list.tasks.push(Task {
//!         id,
//!         description: "example".to_string(),
//!         status,
//!     });
//!
//!     assert_eq!(list.get(id).unwrap().status, status);
//!
//!     #[subtest]
//!     fn cannot_complete_already_finished_task() {
//!         let err = list.complete(id).unwrap_err();
//!         assert!(matches!(err, TodoError::InvalidTransition { .. }));
//!     }
//!
//!     #[subtest]
//!     fn cannot_cancel_already_finished_task() {
//!         let err = list.cancel(id).unwrap_err();
//!         assert!(matches!(err, TodoError::InvalidTransition { .. }));
//!     }
//! }
//! ```
//!
//! <details>
//!
//! <summary>Click to see expansion</summary>
//!
//! ```no_run
//! # use rstest::rstest;
//! #[test]
//! #[rstest::rstest]
//! #[case::completed(TaskStatus::Completed)]
//! #[case::cancelled(TaskStatus::Cancelled)]
//! fn insert_finished_task(#[case] status: TaskStatus) {
//!     let mut list = TodoList::new();
//!     let id = 1;
//!     list.tasks
//!         .push(Task {
//!             id,
//!             description: "example".to_string(),
//!             status,
//!         });
//!     assert_eq!(list.get(id).unwrap().status, status);
//! }
//! mod insert_finished_task_subtests {
//!     use super::*;
//!     #[rstest::rstest]
//!     #[case::completed(TaskStatus::Completed)]
//!     #[case::cancelled(TaskStatus::Cancelled)]
//!     fn cannot_complete_already_finished_task(
//!         #[case]
//! #        #[allow(unused_variables)]
//!         status: TaskStatus,
//!     ) {
//! #        #[allow(unused_variables)]
//!         let mut list = TodoList::new();
//! #        #[allow(unused_variables)]
//!         let id = 1;
//!         list.tasks
//!             .push(Task {
//!                 id,
//!                 description: "example".to_string(),
//!                 status,
//!             });
//!         assert_eq!(list.get(id).unwrap().status, status);
//!         let err = list.complete(id).unwrap_err();
//!         assert!(matches!(err, TodoError::InvalidTransition { .. }));
//!     }
//!     #[rstest::rstest]
//!     #[case::completed(TaskStatus::Completed)]
//!     #[case::cancelled(TaskStatus::Cancelled)]
//!     fn cannot_cancel_already_finished_task(
//!         #[case]
//! #        #[allow(unused_variables)]
//!         status: TaskStatus,
//!     ) {
//! #        #[allow(unused_variables)]
//!         let mut list = TodoList::new();
//! #        #[allow(unused_variables)]
//!         let id = 1;
//!         list.tasks
//!             .push(Task {
//!                 id,
//!                 description: "example".to_string(),
//!                 status,
//!             });
//!         assert_eq!(list.get(id).unwrap().status, status);
//!         let err = list.cancel(id).unwrap_err();
//!         assert!(matches!(err, TodoError::InvalidTransition { .. }));
//!     }
//! }
//! ```
//!
//! </details>
//!
//! The `#[case]`s you define in the top-level test function are applied to nested `#[subtest]`s as well.
//! Make sure to specify the `#[subtest]` attribute first, before `#[rstest]`.
//! The same way you can use `rstest`, you can use any other testing framework as well.
//!
//! ## Omit, add or override attributes, parameters, return types
//!
//! Nested `#[subtest]`s' function attributes, parameters and return type are inherited from the parent test function by default.
//!
//! The following:
//!
//! ```no_run
//! # use subtest::subtest;
//! #[subtest]
//! #[tokio::test]
//! async fn value_can_be_sent_async() -> anyhow::Result<()> {
//!     let (sender, receiver) = tokio::sync::mpsc::channel(5);
//!     sender.send("Hello!").await?;
//!
//!     #[subtest]
//!     async fn value_can_be_received() { // <-- attribute and return type inherited from parent function
//!         let mut receiver = receiver;
//!         let value = receiver.try_recv()?;
//!         assert_eq!(value, "Hello!");
//!         Ok(())
//!     }
//!
//!     drop(receiver);
//!     Ok(())
//! }
//! ```
//!
//! is semantically equivalent to:
//!
//! ```no_run
//! # use subtest::subtest;
//! #[subtest]
//! #[tokio::test]
//! async fn value_can_be_sent_async() -> anyhow::Result<()> {
//!     let (sender, receiver) = tokio::sync::mpsc::channel(5);
//!     sender.send("Hello!").await?;
//!
//!     #[subtest(inherit_attributes = false)] // <-- do not copy attributes from the parent function
//!     #[tokio::test] // <-- same attribute as parent function
//!     async fn value_can_be_received() -> anyhow::Result<()> {  // <-- same return type as parent function
//!         let mut receiver = receiver;
//!         let value = receiver.try_recv()?;
//!         assert_eq!(value, "Hello!");
//!         Ok(())
//!     }
//!
//!     drop(receiver);
//!     Ok(())
//! }
//! ```
//!
//! You may also add attributes, for example `#[ignore]` or `#[should_panic]`:
//!
//! ```no_run
//! # use subtest::subtest;
//! #[subtest]
//! #[test]
//! fn value_can_be_sent_and_received() {
//!     let (sender, mut receiver) = tokio::sync::mpsc::channel(5);
//!     sender.try_send("Hello!").unwrap();
//!     receiver.try_recv().unwrap();
//!
//!     #[subtest]
//!     #[should_panic(expected = "called `Result::unwrap()` on an `Err` value: Empty")]
//!     fn value_cannot_be_received_a_second_time() {
//!         receiver.try_recv().unwrap();
//!     }
//!
//!     #[subtest]
//!     #[ignore]
//!     fn value_can_be_sent_a_second_time() {
//!         unimplemented!()
//!     }
//!
//!     drop(receiver);
//! }
//! ```
//!
//! # Things to be aware of
//!
//! ## Setup code runs once per test, in parallel
//!
//! Every subtest inherits the setup code preceding it.
//! This means that the setup code runs once for the parent test, and once *again* for *every* subtest.
//! Rust's test harness runs tests in parallel by default.
//!
//! This breaks setup code which acquires a shared resource like a fixed TCP port or a file at a fixed path.
//! Example:
//!
//! ```no_run
//! # use subtest::subtest;
//! #[subtest]
//! #[test]
//! fn server_starts() {
//!     let listener = std::net::TcpListener::bind("127.0.0.1:39118").unwrap(); // <-- fixed port
//!     let port = listener.local_addr().unwrap().port();
//!     assert_eq!(port, 39118);
//!
//!     #[subtest]
//!     fn server_accepts_a_connection() {
//!         std::net::TcpStream::connect(("127.0.0.1", port)).unwrap();
//!     }
//! }
//! ```
//!
//! Both tests bind the same port, if they run at the same time one fails:
//!
//! ```text
//! running 2 tests
//! test server_starts ... ok
//! test server_starts_subtests::server_accepts_a_connection ... FAILED
//!
//! failures:
//!
//! ---- server_starts_subtests::server_accepts_a_connection stdout ----
//! called `Result::unwrap()` on an `Err` value: Os { code: 98, kind: AddrInUse, message: "Address already in use" }
//! ```
//!
//! The solution is to make the setup code safe to run concurrently with itself:
//!
//! * prefer resources which are unique per test: port `0` (letting the OS pick a free port) instead of a fixed port or a temporary directory instead of a fixed path. Example:
//!
//!     ```no_run
//!     # use subtest::subtest;
//!     #[subtest]
//!     #[test]
//!     fn server_starts() {
//!         let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap(); // <-- OS-assigned port
//!         let port = listener.local_addr().unwrap().port();
//!         assert_ne!(port, 0);
//!
//!         #[subtest]
//!         fn server_accepts_a_connection() {
//!             std::net::TcpStream::connect(("127.0.0.1", port)).unwrap();
//!         }
//!     }
//!     ```
//!
//! * or, if the resource cannot be made unique, serialize the affected tests with a crate such as [`serial_test`](https://crates.io/crates/serial_test):
//!
//!     ```no_run
//!     # use serial_test::serial;
//!     # use subtest::subtest;
//!     #[subtest]
//!     #[test]
//!     #[serial] // <-- inherited by the subtest, so the two never run at the same time
//!     fn server_starts() {
//!         let listener = std::net::TcpListener::bind("127.0.0.1:39118").unwrap(); // <-- still a fixed port
//!         let port = listener.local_addr().unwrap().port();
//!         assert_eq!(port, 39118);
//!
//!         #[subtest]
//!         fn server_accepts_a_connection() {
//!             std::net::TcpStream::connect(("127.0.0.1", port)).unwrap();
//!         }
//!     }
//!     ```
//!
//! * or, as a last resort, run the whole test binary single-threaded via `cargo test -- --test-threads=1`
//!
//! ## Do not omit test attribute altogether
//!
//! Even when using `#[subtest]`, you still have to specify an "actual" test attribute - typically `#[test]`, or alternatively `#[tokio::test]` or `#[rstest]` (or whatever testing framework you intend to use) - at least for the top-level test function:
//!
//! ```no_run
//! # use subtest::subtest;
//! #[subtest]
//! #[test] // <-- Do not omit this!
//! fn top_level() {
//!     // ...
//!
//!     #[subtest] // <-- here it is fine to omit #[test], since it is inherited
//!     #[should_panic]
//!     fn adds_attributes() {
//!         // ...
//!     }
//!
//!     #[subtest(inherit_attributes = false)]
//!     #[tokio::test] // <-- make sure to include a test attribute when disabling attribute inheritance
//!     async fn overrides_attributes() {
//!         // ...
//!     }
//! }
//! ```
//!
//! Just follow these rules:
//!
//! * If it is a top-level function with a `#[subtest]` attribute: **Specify a `#[test]` attribute as well!**
//! * If it is a nested function with a `#[subtest]` attribute (and optionally more attributes like `#[should_panic]`): You can omit the `#[test]` attribute, it is inherited (see [Omit, add or override attributes, parameters, return types](#omit-add-or-override-attributes-parameters-return-types))
//! * If it is a nested function with a `#[subtest(inherit_attributes = false)]` attribute: **Specify a `#[test]` attribute as well!**
//! * Always put other attributes **after** `#[subtest]`
//!
//! In case you do ever forget the `#[test]` attribute, you will get a compiler error:
//!
//! ```text
//! error: function is missing a test attribute, such as #[test], #[tokio::test] or #[rstest]
//!        add one below #[subtest] - attributes written above it are not visible to this macro
//!        if this function is meant to be a nested subtest, add #[subtest] to the enclosing test function instead
//!  --> tests/ui/fail/missing_test_attr.rs:4:4
//!   |
//! 4 | fn parent() {
//!   |    ^^^^^^
//! ```
//!
//! `subtest` determines test attributes by checking whether they end with `test`.
//! If your test attribute of choice does not, you can opt out of the check via:
//!
//! ```
//! # use subtest::subtest;
//! #[subtest(allow_missing_test_attribute)]
//! // your weirdly-named test attribute goes here!
//! fn top_level() {
//!     // ...
//! }
//! ```
//!
//! ## Attributes apply to the whole subtest tree
//!
//! Attribute inheritance (see [Omit, add or override attributes, parameters, return types](#omit-add-or-override-attributes-parameters-return-types)) is transitive: every attribute of a test function is passed on to its subtests, to *their* subtests, and so on.
//! This includes `#[ignore]` and `#[should_panic]`!
//!
//! So marking a test `#[ignore]` ignores all nested subtests as well:
//!
//! ```no_run
//! # use subtest::subtest;
//! #[subtest]
//! #[test]
//! #[ignore]
//! fn parent() {
//!     let value = 1;
//!     assert_eq!(value, 1);
//!
//!     #[subtest]
//!     fn child() { // <-- inherits #[ignore], so it does not run either
//!         assert_eq!(value + 1, 2);
//!     }
//! }
//! ```
//!
//! ```text
//! running 2 tests
//! test parent ... ignored
//! test parent_subtests::child ... ignored
//! ```
//!
//! The same applies to `#[should_panic]`: a subtest inheriting it fails unless it panics as well.
//!
//! To apply an attribute to the parent only, disable attribute inheritance for the nested subtest and re-specify the attributes you do want:
//!
//! ```no_run
//! # use subtest::subtest;
//! #[subtest]
//! #[test]
//! #[ignore]
//! fn parent() {
//!     let value = 1;
//!     assert_eq!(value, 1);
//!
//!     #[subtest(inherit_attributes = false)]
//!     #[test] // <-- no longer inherited, so specify it explicitly
//!     fn child() { // <-- runs, even though the parent is ignored
//!         assert_eq!(value + 1, 2);
//!     }
//! }
//! ```
//!
//! Note that this only turns off *attribute* inheritance - setup code is still inherited as usual!
//!
//! ## Ambiguous macro import
//!
//! If you happen to use `assert2`'s `assert` macro, or any other macro that has a name similar to a macro from the stdlib's auto-imported prelude, you will get a conflict compiler error when using the macro in a nested `#[subtest]`.
//!
//! Example: The following code:
//!
//! ```no_run
//! use assert2::assert;
//! use subtest::subtest;
//!
//! #[subtest]
//! #[test]
//! fn value_can_be_sent() {
//!     let (sender, receiver) = std::sync::mpsc::channel();
//!     sender.send("Hello!").unwrap();
//!
//!     #[subtest]
//!     fn value_can_be_received() {
//!         let value = receiver.recv().unwrap();
//!         assert!(value == "Hello!");
//!     }
//!
//!     drop(receiver);
//! }
//! ```
//!
//! will lead to the following compiler error:
//!
//! ```text
//! error[E0659]: `assert` is ambiguous
//!   --> tests/ui/fail/readme_ambiguous_assert_import_example.rs:15:9
//!    |
//! 15 |         assert!(value == "Hello!");
//!    |         ^^^^^^ ambiguous name
//!    |
//!    = note: ambiguous because of a conflict between a name from a glob import and an outer scope during import or macro resolution
//! note: `assert` could refer to the macro imported here
//!   --> tests/ui/fail/readme_ambiguous_assert_import_example.rs:6:1
//!    |
//!  6 | #[subtest]
//!    | ^^^^^^^^^^
//!    = help: consider adding an explicit import of `assert` to disambiguate
//!    = help: or use `self::assert` to refer to this macro unambiguously
//! note: `assert` could also refer to the macro defined here
//!   --> $RUST/std/src/prelude/mod.rs
//!    |
//!    |     pub use super::v1::*;
//!    |             ^^^^^^^^^
//!    = note: this error originates in the attribute macro `subtest` (in Nightly builds, run with -Z macro-backtrace for more info)
//! ```
//!
//! This error is caused by the `use super::*` glob import in submodules generated for nested `#[subtest]` functions:
//!
//! <details>
//!
//! <summary>Click to see expansion</summary>
//!
//! ```no_run
//! use assert2::assert;
//! #[test]
//! fn value_can_be_sent() {
//!     let (sender, receiver) = std::sync::mpsc::channel();
//!     sender.send("Hello!").unwrap();
//!     drop(receiver);
//! }
//! mod value_can_be_sent_subtests {
//!     use super::*; // <-- this causes the conflict
//!     #[test]
//!     fn value_can_be_received() {
//! #        #[allow(unused_variables)]
//!         let (sender, receiver) = std::sync::mpsc::channel();
//!         sender.send("Hello!").unwrap();
//!         let value = receiver.recv().unwrap();
//!         assert!(value == "Hello!");
//!     }
//! }
//!
//! ```
//!
//! </details>
//!
//! There are two possible solutions for this:
//!
//! * Either import the macro within the subtest itself (or in the parent, if the parent also uses the macro or you have many subtests using it):
//!
//!     ```no_run
//!     # use assert2::assert;
//!     # use subtest::subtest;
//!     #
//!     # #[subtest]
//!     # #[test]
//!     # fn value_can_be_sent() {
//!     #    let (sender, receiver) = std::sync::mpsc::channel();
//!     #    sender.send("Hello!").unwrap();
//!         #[subtest]
//!         fn value_can_be_received() {
//!             use assert2::assert;
//!             let value = receiver.recv().unwrap();
//!             assert!(value == "Hello!");
//!         }
//!     # }
//!     ```
//!
//! * Or qualify the invocation with `super`:
//!
//!     ```no_run
//!     # use assert2::assert;
//!     # use subtest::subtest;
//!     #
//!     # #[subtest]
//!     # #[test]
//!     # fn value_can_be_sent() {
//!     #    let (sender, receiver) = std::sync::mpsc::channel();
//!     #    sender.send("Hello!").unwrap();
//!         #[subtest]
//!         fn value_can_be_received() {
//!             let value = receiver.recv().unwrap();
//!             super::assert!(value == "Hello!");
//!         }
//!     # }
//!     ```
//!
//! ## Unused variables
//!
//! If you define a variable which is only used in nested subtests, but not in the parent test function, you will get an "unused variables" warning.
//! Example:
//!
//! ```no_run
//! # use subtest::subtest;
//! #[subtest]
//! #[test]
//! fn value_can_be_sent() {
//!     let (sender, receiver) = std::sync::mpsc::channel();
//!     sender.send("Hello!").unwrap();
//!
//!     #[subtest]
//!     fn value_can_be_received() {
//!         let value = receiver.recv().unwrap(); // <-- receiver is used here, but not in the parent
//!         assert_eq!(value, "Hello!");
//!     }
//! }
//! ```
//!
//! will lead to
//!
//! ```text
//! warning: unused variable: `receiver`
//!   --> tests/ui/fail/readme_unused_variables_example.rs:10:18
//!    |
//! 10 |     let (sender, receiver) = std::sync::mpsc::channel();
//!    |                  ^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_receiver`
//!    |
//! ```
//!
//! The solution is to
//!
//! * either avoid declaring unused variables in the parent test function, and move them directly into the subtest that needs them (preferred)
//! * or, if this is not possible (like in the example shown above), explicitly drop them at the end of the parent test function's scope:
//!
//!     ```no_run
//!     # use subtest::subtest;
//!     #[subtest]
//!     #[test]
//!     fn value_can_be_sent() {
//!         let (sender, receiver) = std::sync::mpsc::channel();
//!         sender.send("Hello!").unwrap();
//!   
//!         #[subtest]
//!         fn value_can_be_received() {
//!             let value = receiver.recv().unwrap();
//!             assert_eq!(value, "Hello!");
//!         }
//!   
//!         drop(receiver); // <-- drop here to silence unused variable warning
//!     }
//!     ```

use proc_macro::TokenStream;
use subtest_impl::expand_subtest_main_fn;

/// Specify `#[subtest]` in two places:
///
/// 1. On the top-level test function which contains the subtests
///    * after `#[subtest]`, you **have to** add a test attribute of the testing framework you intend to use, such as regular `#[test]`, `#[tokio::test]`, `#[rstest]`, etc.
///    * optional arguments:
///       * `#[subtest(allow_missing_test_attribute)]` - disables the compiler check for whether a test attribute is present
/// 2. On each individual subtest function
///    * here, you can omit test attributes, parameters, and return types - they are inherited from the parent test function
///    * optional arguments:
///       * `#[subtest(inherit_attributes = false)]` - disable attribute inheritance, now you **have to** add a test attribute of the testing framework you intend to use, similar to the top-level test function
///
/// For more information, refer to the [crate-level documentation](crate).
///
/// # Example
///
/// ```no_run
/// use subtest::subtest;
///
/// #[subtest]
/// #[test]
/// fn add_creates_pending_task() {
///     let mut list = TodoList::new();
///     let id = list.add("Buy milk");
///
///     #[subtest]
///     fn complete_marks_task_completed() {
///         list.complete(id).unwrap();
///         assert_eq!(list.get(id).unwrap().status, TaskStatus::Completed);
///     }
///
///     #[subtest]
///     fn cancel_marks_task_cancelled() {
///         list.cancel(id).unwrap();
///         assert_eq!(list.get(id).unwrap().status, TaskStatus::Cancelled);
///     }
///
///     let task = list.get(id).unwrap();
///     assert_eq!(task.status, TaskStatus::Pending);
/// }
/// ```
#[proc_macro_attribute]
pub fn subtest(args: TokenStream, input: TokenStream) -> TokenStream {
    expand_subtest_main_fn(args.into(), input.into()).into()
}
