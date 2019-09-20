use ntest_test_cases::test_case;
use std::{thread, time};
use ntest::*;

#[test_case(200)]
#[timeout(100)]
#[should_panic]
#[test_case(10)]
#[timeout(100)]
fn no_timeout(i : u32) {
    let fifty_millis = time::Duration::from_millis(i);
    thread::sleep(fifty_millis);
}