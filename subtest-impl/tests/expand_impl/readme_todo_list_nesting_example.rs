#[subtest]
#[test]
fn add_creates_pending_task() {
    let mut list = TodoList::new();
    let id = list.add("Buy milk");

    #[subtest]
    fn cancel_marks_task_cancelled() {
        list.cancel(id).unwrap();
        assert_eq!(list.get(id).unwrap().status, TaskStatus::Cancelled);

        #[subtest]
        fn cannot_complete_already_cancelled_task() {
            let err = list.complete(id).unwrap_err();
            assert!(matches!(err, TodoError::InvalidTransition { .. }));
        }
    }
}
