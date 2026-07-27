# subtest

[![crates.io][crate-image]][crate-link]
[![docs.rs][docs-image]][docs-link]
[![Build Status][ci-image]][ci-link]
[![Apache 2.0 Licensed][license-apache-image]][license-apache-link]
[![MIT Licensed][license-mit-image]][license-mit-link]

*Just continue writing tests.*

Ever got to a point, were you felt extending a test would be much easier than writing a new one?
But the thing you want to test is actually a different feature, and would warrant its own test?
You *could* copy-paste the existing test's setup code, or you *could* refactor it to a shared test fixture, but you could also...

... use `#[subtest]`: a Rust macro to easily use test setup code from one test function in another one.

## Example

**Let's say you have a TODO list app.**

```rust
use subtest::subtest;

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
```

([Link to full example](subtest/tests/expand/readme_todo_list_example.rs))

## How it works

* Statements preceding a nested `#[subtest]` function are **copied** into the nested function's body
* This means you can freely use and mutate any local variables from the parent function in the nested function...
* ... without affecting the parent function or sibling test functions

**The above example gets expanded to:**

```rust
#[test]
fn add_creates_pending_task() {
    let mut list = TodoList::new();
    let id = list.add("Buy milk");
    let task = list.get(id).unwrap();
    assert_eq!(task.status, TaskStatus::Pending);
}
mod add_creates_pending_task_subtests {
    use super::*;
    #[test]
    fn complete_marks_task_completed() {
        let mut list = TodoList::new();
        let id = list.add("Buy milk");
        list.complete(id).unwrap();
        assert_eq!(list.get(id).unwrap().status, TaskStatus::Completed);
    }
    #[test]
    fn cancel_marks_task_cancelled() {
        let mut list = TodoList::new();
        let id = list.add("Buy milk");
        list.cancel(id).unwrap();
        assert_eq!(list.get(id).unwrap().status, TaskStatus::Cancelled);
    }
}
```

## What you can do

### Arbitrarily Nest Test Functions

You can nest `#[subtest]` functions arbitrarily deeply:

```rust
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
```

Each `#[subtest]` inherits the code of all parent functions, in order, as its setup code.

<details>

<summary>Click to see expansion</summary>

```rust
#[test]
fn add_creates_pending_task() {
    let mut list = TodoList::new();
    let id = list.add("Buy milk");
}
mod add_creates_pending_task_subtests {
    use super::*;
    #[test]
    fn cancel_marks_task_cancelled() {
        let mut list = TodoList::new();
        let id = list.add("Buy milk");
        list.cancel(id).unwrap();
        assert_eq!(list.get(id).unwrap().status, TaskStatus::Cancelled);
    }
    mod cancel_marks_task_cancelled_subtests {
        use super::*;
        #[test]
        fn cannot_complete_already_cancelled_task() {
            let mut list = TodoList::new();
            let id = list.add("Buy milk");
            list.cancel(id).unwrap();
            assert_eq!(list.get(id).unwrap().status, TaskStatus::Cancelled);
            let err = list.complete(id).unwrap_err();
            assert!(matches!(err, TodoError::InvalidTransition { .. }));
        }
    }
}
```

</details>

* use async
* use rstest
* upgrade sync -> async
* omit attr, params, return type

## What you can't do

* omit top-level test attr
* make async non-async again
* ambiguous assert macro import
* unused variables

## Changelog

## License

Licensed under either of

* Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license
  ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

[//]: # (links)

[crate-image]: https://img.shields.io/crates/v/subtest.svg

[crate-link]: https://crates.io/crates/subtest

[docs-image]: https://docs.rs/subtest/badge.svg

[docs-link]: https://docs.rs/subtest/

[ci-image]: https://github.com/nthuemmel/subtest/actions/workflows/ci.yml/badge.svg?branch=master

[ci-link]: https://github.com/nthuemmel/subtest/actions/workflows/ci.yml?query=branch%3Amaster

[license-apache-image]: https://img.shields.io/badge/license-Apache2.0-blue.svg

[license-apache-link]: LICENSE-APACHE

[license-mit-image]: https://img.shields.io/badge/license-MIT-blue.svg

[license-mit-link]: LICENSE-MIT
