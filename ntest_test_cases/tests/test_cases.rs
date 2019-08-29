extern crate ntest_test_cases;
use ntest_test_cases::test_case;

#[test_case(42)]
fn one_arg(x: u32) {
    assert_eq!(x, 42)
}

#[test_case(1, 42)]
#[test_case(9, 18)]
#[test_case(5, 20)]
fn two_args(x: u8, y: u32) {
    assert!(x < 10);
    assert!(y > 10);
}

#[test_case(42.42)]
fn float(x: f32) {
    assert_eq!(x, 42.42)
}

#[test_case("walter", "white")]
fn test_string(x: &str, y: &str) {
    assert_eq!(x, "walter");
    assert_eq!(y, "white");
}

#[test_case(true)]
fn test_bool(x: bool) {
    assert!(x);
}

#[test_case(true, "true", 1)]
fn test_mix(x: bool, y: &str, z: u16) {
    assert!(x);
    assert_eq!(y, "true");
    assert_eq!(z, 1);
}