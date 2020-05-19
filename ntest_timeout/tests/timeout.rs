extern crate ntest_timeout;
use ntest_timeout::timeout;
use std::{thread, time};

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
#[ntest::timeout(10)]
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
