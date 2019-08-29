extern crate ntest_test_cases;
use ntest_test_cases::test_case;

#[test_case(42)]
fn one_arg(x: u32) {
    assert_eq!(x, 42)
}


#[test_case(1, 42)]
#[test_case(9, 18)]
#[test_case(5, 20)]
fn two_args(x: u32, y: u32) {
    assert!(x < 10);
    assert!(y > 10);
}
