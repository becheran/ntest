use std::{thread, time};
use ntest::timeout;
use ntest::test_case;

#[test_case(200)]
#[timeout(100)]
#[should_panic]
#[test_case(10)]
#[timeout(100)]
fn test_function(i : u32) {
    let sleep_time = time::Duration::from_millis(i);
    thread::sleep(sleep_time);
}