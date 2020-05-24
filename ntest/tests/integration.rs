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

#[test]
#[timeout(100)]
fn no_timeout() {
    let fifty_millis = time::Duration::from_millis(50);
    thread::sleep(fifty_millis);
}

#[test]
#[timeout(10)]
#[should_panic]
fn timeout() {
    let fifty_millis = time::Duration::from_millis(50);
    thread::sleep(fifty_millis);
}

#[test]
#[timeout(10)]
#[should_panic]
fn timeout_inf_loop() {
    loop {}
}

#[test]
#[timeout(100)]
fn timeout_with_result() -> Result<(), String> {
    let ten_millis = time::Duration::from_millis(10);
    thread::sleep(ten_millis);
    Ok(())
}