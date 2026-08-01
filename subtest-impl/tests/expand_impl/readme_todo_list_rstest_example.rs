#[subtest]
#[rstest]
#[case::completed(TaskStatus::Completed)]
#[case::cancelled(TaskStatus::Cancelled)]
fn insert_finished_task(#[case] status: TaskStatus) {
    let mut list = TodoList::new();
    let id = 1;

    list.tasks.push(Task {
        id,
        description: "example".to_string(),
        status,
    });

    assert_eq!(list.get(id).unwrap().status, status);

    #[subtest]
    fn cannot_complete_already_finished_task() {
        let err = list.complete(id).unwrap_err();
        assert!(matches!(err, TodoError::InvalidTransition { .. }));
    }

    #[subtest]
    fn cannot_cancel_already_finished_task() {
        let err = list.cancel(id).unwrap_err();
        assert!(matches!(err, TodoError::InvalidTransition { .. }));
    }
}
