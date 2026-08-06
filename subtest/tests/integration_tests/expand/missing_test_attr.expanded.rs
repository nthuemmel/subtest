fn parent() {}
mod parent_subtests {
    use super::*;
    fn child() {}
}
#[rustc_main]
#[coverage(off)]
#[doc(hidden)]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[])
}
