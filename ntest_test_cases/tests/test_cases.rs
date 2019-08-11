extern crate ntest_test_cases;
#[doc(hidden)]
pub use ntest_test_cases::test_case;

#[test_case(1)]
#[test_case(2)]
#[test_case(3)]
#[test_case(42)]
fn one_arg(x: u32) {
    assert_eq!(x, 42)
}


#[test_case(13)]
fn one_arg_fail(x: u32) {
    assert_eq!(x, 13)
}

//#[test_case(m=21,13, 42)]
//fn two_args(x: u32, y: u32) {
//    assert_eq!(x, 13);
//    assert_eq!(y, 42);
//}
