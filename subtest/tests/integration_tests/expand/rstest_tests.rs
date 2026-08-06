use rstest::rstest;
use subtest::subtest;

#[subtest]
#[rstest]
#[case(0, 0)]
#[case(1, 1)]
#[case(2, 1)]
#[case(3, 2)]
#[case(4, 3)]
#[case(5, 5)]
#[case(6, 8)]
fn fibonacci_test(#[case] input: u32, #[case] expected: u32) {
    let number = fibonacci(input);

    assert_eq!(expected, number);

    #[subtest]
    fn next(#[case] input: u32, #[case] expected: u32) {
        let next = number + 1;
        assert_eq!(next, fibonacci(input) + 1);
    }
}

fn fibonacci(input: u32) -> u32 {
    match input {
        0 => 0,
        1 => 1,
        n => fibonacci(n - 2) + fibonacci(n - 1),
    }
}
