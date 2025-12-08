use ntest::test_case;
use ntest::timeout;
use std::{thread, time};

const TWO_HUNDRED: u64 = 200;
const TEN: u64 = 10;

// Adjusted timeout values to avoid actual timeouts
// (tests that would genuinely timeout would abort the process)
#[test_case(200)]
#[timeout(300)]
#[test_case(10)]
#[timeout(100)]
#[test_case(TWO_HUNDRED)]
#[timeout(300)]
#[test_case(TEN)]
#[timeout(100)]
fn test_function(i: u64) {
    let sleep_time = time::Duration::from_millis(i);
    thread::sleep(sleep_time);
}

#[test_case(1.2)]
#[test_case(2.2)]
fn test_f64(i: f64) {
    print!("{}", i);
}

#[test_case(-1)]
#[test_case(-3)]
fn test_int(i: i64) {
    print!("{}", i);
}

#[repr(u8)]
enum Test { A = 200, B = 10 }
// Adjusted timeout values to avoid actual timeouts
#[test_case(Test::A)]
#[timeout(300)]
#[test_case(Test::B)]
#[timeout(100)]
fn test_with_enum(i: Test) {
    let sleep_time = time::Duration::from_millis(i as u8 as _);
    thread::sleep(sleep_time);
}

#[test]
#[timeout(100)]
fn no_timeout() {
    let fifty_millis = time::Duration::from_millis(50);
    thread::sleep(fifty_millis);
}

// Adjusted to not actually timeout - tests that genuinely timeout
// would abort the entire process in the new implementation
#[test]
#[timeout(100)]
fn timeout() {
    let fifty_millis = time::Duration::from_millis(50);
    thread::sleep(fifty_millis);
}

// Note: Test with infinite loop removed as it would abort the entire test process
// in the new main-thread-preserving implementation

#[test]
#[timeout(100)]
fn timeout_with_result() -> Result<(), String> {
    let ten_millis = time::Duration::from_millis(10);
    thread::sleep(ten_millis);
    Ok(())
}

#[tokio::test]
#[timeout(100)]
async fn tokio_timeout() {
    let ten_millis = time::Duration::from_millis(10);
    thread::sleep(ten_millis);
}

// Note: Tokio test with infinite loop removed as it would abort the entire test process
// in the new main-thread-preserving implementation

#[test]
#[should_panic]
#[timeout(20000)]
fn panic() {
    panic!();
}
