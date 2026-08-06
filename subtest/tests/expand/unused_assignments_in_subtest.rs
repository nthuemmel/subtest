use subtest::subtest;

#[subtest]
#[test]
fn values_are_assigned() {
    let deferred;
    deferred = "a";

    let nested;
    {
        nested = "b";
    }

    let branched;
    if deferred == "a" {
        branched = "c";
    } else {
        branched = "d";
    }

    let mut counter = 0;
    counter += 1;

    #[subtest]
    fn subtest_reads_none_of_them() {
        // the subtest inherits the assignments above, but reads none of the assigned values -
        // which must not be reported, because the parent test function below does read them
        assert_eq!(1 + 1, 2);
    }

    assert_eq!((deferred, nested, branched, counter), ("a", "b", "c", 1));
}
