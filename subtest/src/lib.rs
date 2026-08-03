//! *Just continue writing tests.*
//!
//! Ever got to a point, were you felt extending a test would be much easier than writing a new one?
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
//! # How it works
//!
//! * Statements preceding a nested `#[subtest]` function are **copied** into the nested function's body
//! * This means you can freely use and mutate any local variables from the parent function in the nested function...
//! * ... without affecting the parent function or sibling test functions
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
//!         let mut list = TodoList::new();
//!         let id = list.add("Buy milk");
//!         list.complete(id).unwrap();
//!         assert_eq!(list.get(id).unwrap().status, TaskStatus::Completed);
//!     }
//!     #[test]
//!     fn cancel_marks_task_cancelled() {
//!         let mut list = TodoList::new();
//!         let id = list.add("Buy milk");
//!         list.cancel(id).unwrap();
//!         assert_eq!(list.get(id).unwrap().status, TaskStatus::Cancelled);
//!     }
//! }
//! ```
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
//!         let mut list = TodoList::new();
//!         let id = list.add("Buy milk");
//!         list.cancel(id).unwrap();
//!         assert_eq!(list.get(id).unwrap().status, TaskStatus::Cancelled);
//!     }
//!     mod cancel_marks_task_cancelled_subtests {
//!         use super::*;
//!         #[test]
//!         fn cannot_complete_already_cancelled_task() {
//!             let mut list = TodoList::new();
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
//!     #[subtest]
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
//!     fn cannot_complete_already_finished_task(#[case] status: TaskStatus) {
//!         let mut list = TodoList::new();
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
//!     fn cannot_cancel_already_finished_task(#[case] status: TaskStatus) {
//!         let mut list = TodoList::new();
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
//! ## Omit or override attributes, parameters, return types
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
//!     #[subtest]
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
//! You may also override any of attributes, parameters and return types as needed, for example by adding `#[ignore]` or `#[should_panic]`:
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
//!     #[test] // <-- make sure to include test attribute when overriding attributes
//!     #[should_panic(expected = "called `Result::unwrap()` on an `Err` value: Empty")]
//!     fn value_cannot_be_received_a_second_time() {
//!         receiver.try_recv().unwrap();
//!     }
//!
//!     #[subtest]
//!     #[test] // <-- make sure to include test attribute when overriding attributes
//!     #[ignore]
//!     fn value_can_be_sent_a_second_time() {
//!         unimplemented!()
//!     }
//!
//!     drop(receiver);
//! }
//! ```
//!
//! **Note:** Overriding is all-or-nothing. If you add an attribute, you have to repeat the remaining relevant attributes from the parent function, in this case `#[test]`.
//!
//! # Things to be aware of
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
//!     fn inherits_attributes() {
//!         // ...
//!     }
//!   
//!     #[subtest]
//!     #[test] // <-- make sure to include #[test] attribute when overriding attributes, like adding #[should_panic]
//!     #[should_panic]
//!     fn overrides_attributes() {
//!         // ...
//!     }
//! }
//! ```
//!
//! Just follow these rules:
//!
//! * If it is a top-level function with a `#[subtest]` attribute: **Specify a `#[test]` attribute as well!**
//! * If it is a nested function with **just** a `#[subtest]` attribute: You can omit the `#[test]` attribute, it is inherited (see [Omit or Override Attributes, Parameters, Return Types](#omit-or-override-attributes-parameters-return-types))
//! * If it is a nested function with added attributes like `#[should_panic]`: **Specify a `#[test]` attribute as well!** (see [Omit or Override Attributes, Parameters, Return Types](#omit-or-override-attributes-parameters-return-types))
//! * Always put other attributes **after** `#[subtest]`
//!
//! In case you do ever forget the `#[test]` attribute, you will get the same compiler warning as if you forgot to annotate a regular test function with `#[test]`:
//!
//! ```text
//! warning: function `parent` is never used
//!  --> subtest/tests/expand/missing_test_attr.rs:2:4
//!   |
//! 2 | fn parent() {
//!   |    ^^^^^^
//!   |
//!   = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default
//! ```
//!
//! I recommend automatically running clippy with deny-warnings turned on (`cargo clippy --locked --all-targets --all-features -- -D warnings`), which catches these issues fairly quickly.
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
//! * Either import the macro within the subtest itself:
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
//! ## Unused variables in parent test function
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
//!         let value = receiver.recv().unwrap();
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
//!         drop(receiver);
//!     }
//!     ```

use proc_macro::TokenStream;
use subtest_impl::expand_subtest_main_fn;

/// Specify `#[subtest]` in two places:
///
/// 1. On the top-level test function which contains the subtests
///    * after `#[subtest]`, you **have to** add a test attribute of the testing framework you intend to use, such as regular `#[test]`, `#[tokio::test]`, `#[rstest]`, etc.
/// 2. On each individual subtest function
///    * here, you can omit test attributes, parameters, and return types - they are inherited from the parent test function
///
/// `#[subtest]` takes no arguments.
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
