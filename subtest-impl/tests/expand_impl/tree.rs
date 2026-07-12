#[subtest]
#[test]
fn level_0() {
    let i = 1;

    #[subtest]
    fn level_1a() {
        let level1 = "a";

        #[subtest]
        fn level_1a_2a() {
            let level2 = "a";
        }

        #[subtest]
        fn level_1a_2b() {
            let level2 = "b";

            #[subtest]
            fn level_1a_2b_3() {
                let level3 = "X";
            }
        }
    }

    #[subtest]
    fn level_1b() {
        let level1 = "b";

        #[subtest]
        fn level_1b_2a() {
            let level2 = "1ba";
        }

        #[subtest]
        fn level_1b_2b() {
            let level2 = "1bb";
        }
    }
}
