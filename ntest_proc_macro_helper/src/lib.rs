//! The ntest lib enhances the rust test framework with some useful functions.
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub enum TimeoutResult<T : Send> {
    Timeout,
    Panic,
    Result(T)
}

#[doc(hidden)]
/// Timeout helper for proc macro timeout
pub fn execute_with_timeout<T: Send>(
    code: &'static (dyn Fn() -> T + Sync + 'static),
    timeout_ms: u64,
) -> TimeoutResult<T> {
    let (sender, receiver) = mpsc::channel();
    thread::spawn(move || {
        if let Ok(()) = sender.send(code()) {}
    });
    match receiver.recv_timeout(Duration::from_millis(timeout_ms)) {
        Ok(t) => TimeoutResult::Result(t),
        Err(mpsc::RecvTimeoutError::Timeout) => TimeoutResult::Timeout,
        Err(mpsc::RecvTimeoutError::Disconnected) => TimeoutResult::Panic,
    }
}
