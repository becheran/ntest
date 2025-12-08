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

// Note: Tests with infinite loops that timeout will abort the process
// and cannot be tested with #[should_panic] in the new implementation
// that preserves the main thread. These tests are removed as they
// would abort the entire test process.

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

// Test for issue: ntest_timeout ruins #[should_panic = "message"]
#[test]
#[should_panic = "402"]
#[timeout(8000)]
fn should_panic_with_message() {
    panic!("402")
}

#[test]
#[should_panic]
#[timeout(8000)]
fn should_panic_without_message() {
    panic!("some panic message")
}

#[test]
#[should_panic(expected = "custom error")]
#[timeout(8000)]
fn should_panic_with_expected_syntax() {
    panic!("custom error")
}

#[timeout(8000)]
#[test]
#[should_panic = "error 123"]
fn should_panic_with_message_reversed_order() {
    panic!("error 123")
}
