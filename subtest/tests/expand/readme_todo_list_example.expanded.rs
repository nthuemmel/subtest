use std::fmt;
pub enum TaskStatus {
    Pending,
    Completed,
    Cancelled,
}
#[automatically_derived]
impl ::core::fmt::Debug for TaskStatus {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(
            f,
            match self {
                TaskStatus::Pending => "Pending",
                TaskStatus::Completed => "Completed",
                TaskStatus::Cancelled => "Cancelled",
            },
        )
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for TaskStatus {}
#[automatically_derived]
impl ::core::cmp::PartialEq for TaskStatus {
    #[inline]
    fn eq(&self, other: &TaskStatus) -> bool {
        let __self_discr = ::core::intrinsics::discriminant_value(self);
        let __arg1_discr = ::core::intrinsics::discriminant_value(other);
        __self_discr == __arg1_discr
    }
}
#[automatically_derived]
#[doc(hidden)]
unsafe impl ::core::clone::TrivialClone for TaskStatus {}
#[automatically_derived]
impl ::core::clone::Clone for TaskStatus {
    #[inline]
    fn clone(&self) -> TaskStatus {
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for TaskStatus {}
impl fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            TaskStatus::Pending => "Pending",
            TaskStatus::Completed => "Completed",
            TaskStatus::Cancelled => "Cancelled",
        };
        f.write_fmt(format_args!("{0}", label))
    }
}
pub struct Task {
    pub id: u32,
    pub description: String,
    pub status: TaskStatus,
}
impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(
            format_args!("[{0}] ({1}) {2}", self.id, self.status, self.description),
        )
    }
}
pub enum TodoError {
    TaskNotFound(u32),
    InvalidTransition { id: u32, from: TaskStatus, to: TaskStatus },
}
#[automatically_derived]
impl ::core::fmt::Debug for TodoError {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            TodoError::TaskNotFound(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(
                    f,
                    "TaskNotFound",
                    &__self_0,
                )
            }
            TodoError::InvalidTransition {
                id: __self_0,
                from: __self_1,
                to: __self_2,
            } => {
                ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "InvalidTransition",
                    "id",
                    __self_0,
                    "from",
                    __self_1,
                    "to",
                    &__self_2,
                )
            }
        }
    }
}
impl fmt::Display for TodoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TodoError::TaskNotFound(id) => {
                f.write_fmt(format_args!("task {0} not found", id))
            }
            TodoError::InvalidTransition { id, from, to } => {
                f.write_fmt(
                    format_args!(
                        "cannot transition task {0} from {1} to {2}", id, from, to,
                    ),
                )
            }
        }
    }
}
impl std::error::Error for TodoError {}
pub struct TodoList {
    tasks: Vec<Task>,
    next_id: u32,
}
impl TodoList {
    pub fn new() -> Self {
        TodoList {
            tasks: Vec::new(),
            next_id: 1,
        }
    }
    pub fn add(&mut self, description: impl Into<String>) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        self.tasks
            .push(Task {
                id,
                description: description.into(),
                status: TaskStatus::Pending,
            });
        id
    }
    /// Marks a task as completed.
    ///
    /// Fails if the task doesn't exist or is already completed/cancelled.
    pub fn complete(&mut self, id: u32) -> Result<(), TodoError> {
        self.set_status(id, TaskStatus::Completed)
    }
    /// Marks a task as cancelled.
    ///
    /// Fails if the task doesn't exist or is already completed/cancelled.
    pub fn cancel(&mut self, id: u32) -> Result<(), TodoError> {
        self.set_status(id, TaskStatus::Cancelled)
    }
    /// Internal helper: transitions a task to `new_status`, only if it is
    /// currently `Pending`.
    fn set_status(&mut self, id: u32, new_status: TaskStatus) -> Result<(), TodoError> {
        let task = self
            .tasks
            .iter_mut()
            .find(|t| t.id == id)
            .ok_or(TodoError::TaskNotFound(id))?;
        if task.status != TaskStatus::Pending {
            return Err(TodoError::InvalidTransition {
                id,
                from: task.status,
                to: new_status,
            });
        }
        task.status = new_status;
        Ok(())
    }
    /// Returns a reference to a task by id, if it exists.
    pub fn get(&self, id: u32) -> Option<&Task> {
        self.tasks.iter().find(|t| t.id == id)
    }
}
mod tests {
    use super::*;
    use rstest::rstest;
    use subtest::subtest;
    extern crate test;
    #[rustc_test_marker = "tests::add_creates_pending_task"]
    #[doc(hidden)]
    pub const add_creates_pending_task: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("tests::add_creates_pending_task"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "./tests/expand/readme_todo_list_example.rs",
            start_line: 129usize,
            start_col: 8usize,
            end_line: 129usize,
            end_col: 32usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::Unknown,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(add_creates_pending_task()),
        ),
    };
    fn add_creates_pending_task() {
        let mut list = TodoList::new();
        let id = list.add("Buy milk");
        let task = list.get(id).unwrap();
        match (&task.status, &TaskStatus::Pending) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
    }
    mod add_creates_pending_task_subtests {
        use super::*;
        extern crate test;
        #[rustc_test_marker = "tests::add_creates_pending_task_subtests::complete_marks_task_completed"]
        #[doc(hidden)]
        pub const complete_marks_task_completed: test::TestDescAndFn = test::TestDescAndFn {
            desc: test::TestDesc {
                name: test::StaticTestName(
                    "tests::add_creates_pending_task_subtests::complete_marks_task_completed",
                ),
                ignore: false,
                ignore_message: ::core::option::Option::None,
                source_file: "./tests/expand/readme_todo_list_example.rs",
                start_line: 134usize,
                start_col: 12usize,
                end_line: 134usize,
                end_col: 41usize,
                compile_fail: false,
                no_run: false,
                should_panic: test::ShouldPanic::No,
                test_type: test::TestType::Unknown,
            },
            testfn: test::StaticTestFn(
                #[coverage(off)]
                || test::assert_test_result(complete_marks_task_completed()),
            ),
        };
        fn complete_marks_task_completed() {
            #[allow(unused_variables)]
            let mut list = TodoList::new();
            #[allow(unused_variables)]
            let id = list.add("Buy milk");
            list.complete(id).unwrap();
            match (&list.get(id).unwrap().status, &TaskStatus::Completed) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        let kind = ::core::panicking::AssertKind::Eq;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::None,
                        );
                    }
                }
            };
        }
        extern crate test;
        #[rustc_test_marker = "tests::add_creates_pending_task_subtests::cancel_marks_task_cancelled"]
        #[doc(hidden)]
        pub const cancel_marks_task_cancelled: test::TestDescAndFn = test::TestDescAndFn {
            desc: test::TestDesc {
                name: test::StaticTestName(
                    "tests::add_creates_pending_task_subtests::cancel_marks_task_cancelled",
                ),
                ignore: false,
                ignore_message: ::core::option::Option::None,
                source_file: "./tests/expand/readme_todo_list_example.rs",
                start_line: 140usize,
                start_col: 12usize,
                end_line: 140usize,
                end_col: 39usize,
                compile_fail: false,
                no_run: false,
                should_panic: test::ShouldPanic::No,
                test_type: test::TestType::Unknown,
            },
            testfn: test::StaticTestFn(
                #[coverage(off)]
                || test::assert_test_result(cancel_marks_task_cancelled()),
            ),
        };
        fn cancel_marks_task_cancelled() {
            #[allow(unused_variables)]
            let mut list = TodoList::new();
            #[allow(unused_variables)]
            let id = list.add("Buy milk");
            list.cancel(id).unwrap();
            match (&list.get(id).unwrap().status, &TaskStatus::Cancelled) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        let kind = ::core::panicking::AssertKind::Eq;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::None,
                        );
                    }
                }
            };
        }
        mod cancel_marks_task_cancelled_subtests {
            use super::*;
            extern crate test;
            #[rustc_test_marker = "tests::add_creates_pending_task_subtests::cancel_marks_task_cancelled_subtests::cannot_complete_already_cancelled_task"]
            #[doc(hidden)]
            pub const cannot_complete_already_cancelled_task: test::TestDescAndFn = test::TestDescAndFn {
                desc: test::TestDesc {
                    name: test::StaticTestName(
                        "tests::add_creates_pending_task_subtests::cancel_marks_task_cancelled_subtests::cannot_complete_already_cancelled_task",
                    ),
                    ignore: false,
                    ignore_message: ::core::option::Option::None,
                    source_file: "./tests/expand/readme_todo_list_example.rs",
                    start_line: 145usize,
                    start_col: 16usize,
                    end_line: 145usize,
                    end_col: 54usize,
                    compile_fail: false,
                    no_run: false,
                    should_panic: test::ShouldPanic::No,
                    test_type: test::TestType::Unknown,
                },
                testfn: test::StaticTestFn(
                    #[coverage(off)]
                    || test::assert_test_result(cannot_complete_already_cancelled_task()),
                ),
            };
            fn cannot_complete_already_cancelled_task() {
                #[allow(unused_variables)]
                let mut list = TodoList::new();
                #[allow(unused_variables)]
                let id = list.add("Buy milk");
                list.cancel(id).unwrap();
                match (&list.get(id).unwrap().status, &TaskStatus::Cancelled) {
                    (left_val, right_val) => {
                        if !(*left_val == *right_val) {
                            let kind = ::core::panicking::AssertKind::Eq;
                            ::core::panicking::assert_failed(
                                kind,
                                &*left_val,
                                &*right_val,
                                ::core::option::Option::None,
                            );
                        }
                    }
                };
                let err = list.complete(id).unwrap_err();
                if !#[allow(non_exhaustive_omitted_patterns)]
                match err {
                    TodoError::InvalidTransition { .. } => true,
                    _ => false,
                } {
                    ::core::panicking::panic(
                        "assertion failed: matches!(err, TodoError::InvalidTransition { .. })",
                    )
                }
            }
        }
    }
    fn insert_finished_task(status: TaskStatus) {
        {
            let mut list = TodoList::new();
            let id = 1;
            list.tasks
                .push(Task {
                    id,
                    description: "example".to_string(),
                    status,
                });
            match (&list.get(id).unwrap().status, &status) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        let kind = ::core::panicking::AssertKind::Eq;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::None,
                        );
                    }
                }
            };
        }
    }
    mod insert_finished_task {
        use super::*;
        extern crate test;
        #[rustc_test_marker = "tests::insert_finished_task::case_1_completed"]
        #[doc(hidden)]
        pub const case_1_completed: test::TestDescAndFn = test::TestDescAndFn {
            desc: test::TestDesc {
                name: test::StaticTestName(
                    "tests::insert_finished_task::case_1_completed",
                ),
                ignore: false,
                ignore_message: ::core::option::Option::None,
                source_file: "./tests/expand/readme_todo_list_example.rs",
                start_line: 159usize,
                start_col: 8usize,
                end_line: 159usize,
                end_col: 28usize,
                compile_fail: false,
                no_run: false,
                should_panic: test::ShouldPanic::No,
                test_type: test::TestType::Unknown,
            },
            testfn: test::StaticTestFn(
                #[coverage(off)]
                || test::assert_test_result(case_1_completed()),
            ),
        };
        fn case_1_completed() {
            let status = TaskStatus::Completed;
            insert_finished_task(status)
        }
        extern crate test;
        #[rustc_test_marker = "tests::insert_finished_task::case_2_cancelled"]
        #[doc(hidden)]
        pub const case_2_cancelled: test::TestDescAndFn = test::TestDescAndFn {
            desc: test::TestDesc {
                name: test::StaticTestName(
                    "tests::insert_finished_task::case_2_cancelled",
                ),
                ignore: false,
                ignore_message: ::core::option::Option::None,
                source_file: "./tests/expand/readme_todo_list_example.rs",
                start_line: 159usize,
                start_col: 8usize,
                end_line: 159usize,
                end_col: 28usize,
                compile_fail: false,
                no_run: false,
                should_panic: test::ShouldPanic::No,
                test_type: test::TestType::Unknown,
            },
            testfn: test::StaticTestFn(
                #[coverage(off)]
                || test::assert_test_result(case_2_cancelled()),
            ),
        };
        fn case_2_cancelled() {
            let status = TaskStatus::Cancelled;
            insert_finished_task(status)
        }
    }
    mod insert_finished_task_subtests {
        use super::*;
        fn cannot_complete_already_finished_task(
            #[allow(unused_variables)]
            status: TaskStatus,
        ) {
            {
                #[allow(unused_variables)]
                let mut list = TodoList::new();
                #[allow(unused_variables)]
                let id = 1;
                list.tasks
                    .push(Task {
                        id,
                        description: "example".to_string(),
                        status,
                    });
                match (&list.get(id).unwrap().status, &status) {
                    (left_val, right_val) => {
                        if !(*left_val == *right_val) {
                            let kind = ::core::panicking::AssertKind::Eq;
                            ::core::panicking::assert_failed(
                                kind,
                                &*left_val,
                                &*right_val,
                                ::core::option::Option::None,
                            );
                        }
                    }
                };
                let err = list.complete(id).unwrap_err();
                if !#[allow(non_exhaustive_omitted_patterns)]
                match err {
                    TodoError::InvalidTransition { .. } => true,
                    _ => false,
                } {
                    ::core::panicking::panic(
                        "assertion failed: matches!(err, TodoError::InvalidTransition { .. })",
                    )
                }
            }
        }
        mod cannot_complete_already_finished_task {
            use super::*;
            extern crate test;
            #[rustc_test_marker = "tests::insert_finished_task_subtests::cannot_complete_already_finished_task::case_1_completed"]
            #[doc(hidden)]
            pub const case_1_completed: test::TestDescAndFn = test::TestDescAndFn {
                desc: test::TestDesc {
                    name: test::StaticTestName(
                        "tests::insert_finished_task_subtests::cannot_complete_already_finished_task::case_1_completed",
                    ),
                    ignore: false,
                    ignore_message: ::core::option::Option::None,
                    source_file: "./tests/expand/readme_todo_list_example.rs",
                    start_line: 172usize,
                    start_col: 12usize,
                    end_line: 172usize,
                    end_col: 49usize,
                    compile_fail: false,
                    no_run: false,
                    should_panic: test::ShouldPanic::No,
                    test_type: test::TestType::Unknown,
                },
                testfn: test::StaticTestFn(
                    #[coverage(off)]
                    || test::assert_test_result(case_1_completed()),
                ),
            };
            fn case_1_completed() {
                let status = TaskStatus::Completed;
                cannot_complete_already_finished_task(status)
            }
            extern crate test;
            #[rustc_test_marker = "tests::insert_finished_task_subtests::cannot_complete_already_finished_task::case_2_cancelled"]
            #[doc(hidden)]
            pub const case_2_cancelled: test::TestDescAndFn = test::TestDescAndFn {
                desc: test::TestDesc {
                    name: test::StaticTestName(
                        "tests::insert_finished_task_subtests::cannot_complete_already_finished_task::case_2_cancelled",
                    ),
                    ignore: false,
                    ignore_message: ::core::option::Option::None,
                    source_file: "./tests/expand/readme_todo_list_example.rs",
                    start_line: 172usize,
                    start_col: 12usize,
                    end_line: 172usize,
                    end_col: 49usize,
                    compile_fail: false,
                    no_run: false,
                    should_panic: test::ShouldPanic::No,
                    test_type: test::TestType::Unknown,
                },
                testfn: test::StaticTestFn(
                    #[coverage(off)]
                    || test::assert_test_result(case_2_cancelled()),
                ),
            };
            fn case_2_cancelled() {
                let status = TaskStatus::Cancelled;
                cannot_complete_already_finished_task(status)
            }
        }
        fn cannot_cancel_already_finished_task(
            #[allow(unused_variables)]
            status: TaskStatus,
        ) {
            {
                #[allow(unused_variables)]
                let mut list = TodoList::new();
                #[allow(unused_variables)]
                let id = 1;
                list.tasks
                    .push(Task {
                        id,
                        description: "example".to_string(),
                        status,
                    });
                match (&list.get(id).unwrap().status, &status) {
                    (left_val, right_val) => {
                        if !(*left_val == *right_val) {
                            let kind = ::core::panicking::AssertKind::Eq;
                            ::core::panicking::assert_failed(
                                kind,
                                &*left_val,
                                &*right_val,
                                ::core::option::Option::None,
                            );
                        }
                    }
                };
                let err = list.cancel(id).unwrap_err();
                if !#[allow(non_exhaustive_omitted_patterns)]
                match err {
                    TodoError::InvalidTransition { .. } => true,
                    _ => false,
                } {
                    ::core::panicking::panic(
                        "assertion failed: matches!(err, TodoError::InvalidTransition { .. })",
                    )
                }
            }
        }
        mod cannot_cancel_already_finished_task {
            use super::*;
            extern crate test;
            #[rustc_test_marker = "tests::insert_finished_task_subtests::cannot_cancel_already_finished_task::case_1_completed"]
            #[doc(hidden)]
            pub const case_1_completed: test::TestDescAndFn = test::TestDescAndFn {
                desc: test::TestDesc {
                    name: test::StaticTestName(
                        "tests::insert_finished_task_subtests::cannot_cancel_already_finished_task::case_1_completed",
                    ),
                    ignore: false,
                    ignore_message: ::core::option::Option::None,
                    source_file: "./tests/expand/readme_todo_list_example.rs",
                    start_line: 178usize,
                    start_col: 12usize,
                    end_line: 178usize,
                    end_col: 47usize,
                    compile_fail: false,
                    no_run: false,
                    should_panic: test::ShouldPanic::No,
                    test_type: test::TestType::Unknown,
                },
                testfn: test::StaticTestFn(
                    #[coverage(off)]
                    || test::assert_test_result(case_1_completed()),
                ),
            };
            fn case_1_completed() {
                let status = TaskStatus::Completed;
                cannot_cancel_already_finished_task(status)
            }
            extern crate test;
            #[rustc_test_marker = "tests::insert_finished_task_subtests::cannot_cancel_already_finished_task::case_2_cancelled"]
            #[doc(hidden)]
            pub const case_2_cancelled: test::TestDescAndFn = test::TestDescAndFn {
                desc: test::TestDesc {
                    name: test::StaticTestName(
                        "tests::insert_finished_task_subtests::cannot_cancel_already_finished_task::case_2_cancelled",
                    ),
                    ignore: false,
                    ignore_message: ::core::option::Option::None,
                    source_file: "./tests/expand/readme_todo_list_example.rs",
                    start_line: 178usize,
                    start_col: 12usize,
                    end_line: 178usize,
                    end_col: 47usize,
                    compile_fail: false,
                    no_run: false,
                    should_panic: test::ShouldPanic::No,
                    test_type: test::TestType::Unknown,
                },
                testfn: test::StaticTestFn(
                    #[coverage(off)]
                    || test::assert_test_result(case_2_cancelled()),
                ),
            };
            fn case_2_cancelled() {
                let status = TaskStatus::Cancelled;
                cannot_cancel_already_finished_task(status)
            }
        }
    }
}
#[rustc_main]
#[coverage(off)]
#[doc(hidden)]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(
        &[
            &add_creates_pending_task,
            &cancel_marks_task_cancelled,
            &cannot_complete_already_cancelled_task,
            &complete_marks_task_completed,
            &case_1_completed,
            &case_2_cancelled,
            &case_1_completed,
            &case_2_cancelled,
            &case_1_completed,
            &case_2_cancelled,
        ],
    )
}
