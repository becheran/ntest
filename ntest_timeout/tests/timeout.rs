use ntest_timeout::timeout;
use std::{thread, time};

#[test]
#[timeout(100)]
fn no_timeout_1() {
    let fifty_millis = time::Duration::from_millis(50);
    thread::sleep(fifty_millis);
}

#[timeout(100)]
#[test]
fn no_timeout_2() {
    let fifty_millis = time::Duration::from_millis(50);
    thread::sleep(fifty_millis);
}

#[test]
#[timeout(10)]
#[should_panic]
fn timeout_1() {
    loop {}
}

#[timeout(10)]
#[should_panic]
#[test]
fn timeout_2() {
    loop {}
}

#[test]
#[timeout(100)]
fn timeout_with_result_1() -> Result<(), String> {
    let ten_millis = time::Duration::from_millis(10);
    thread::sleep(ten_millis);
    Ok(())
}

#[timeout(100)]
#[test]
fn timeout_with_result_2() -> Result<(), String> {
    let ten_millis = time::Duration::from_millis(10);
    thread::sleep(ten_millis);
    Ok(())
}
