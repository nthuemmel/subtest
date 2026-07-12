#![cfg(test)]

use subtest::subtest;

#[subtest]
#[tokio::test]
async fn simple() {
    #[subtest]
    async fn nested() {}
}
