#[subtest]
#[test]
fn add_creates_pending_task() {
    let mut list = TodoList::new();
    let id = list.add("Buy milk");

    #[subtest]
    fn complete_marks_task_completed() {
        list.complete(id).unwrap();
        assert_eq!(list.get(id).unwrap().status, TaskStatus::Completed);
    }

    #[subtest]
    fn cancel_marks_task_cancelled() {
        list.cancel(id).unwrap();
        assert_eq!(list.get(id).unwrap().status, TaskStatus::Cancelled);
    }

    let task = list.get(id).unwrap();
    assert_eq!(task.status, TaskStatus::Pending);
}
