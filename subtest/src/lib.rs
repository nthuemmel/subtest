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
/// ### Example
///
/// ```rust
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
