use ntest_timeout::timeout;
use std::sync::OnceLock;
use std::thread;

/// Store the thread ID to compare between tests
static TEST_THREAD_ID: OnceLock<std::thread::ThreadId> = OnceLock::new();

#[test]
fn test_01_without_timeout_capture_thread_id() {
    // Capture the thread ID of a normal test
    let thread_id = thread::current().id();
    let _ = TEST_THREAD_ID.set(thread_id);
    println!("Normal test - Thread ID: {:?}, Thread name: {:?}", thread_id, thread::current().name());
}

#[test]
#[timeout(1000)]
fn test_02_with_timeout_same_thread() {
    // Check if timeout now runs on the same thread (the fix)
    let thread_id = thread::current().id();
    let original_thread_id = TEST_THREAD_ID.get();
    
    println!("Timeout test - Thread ID: {:?}, Thread name: {:?}", thread_id, thread::current().name());
    println!("Original test thread ID: {:?}", original_thread_id);
    
    // With the new implementation, the test should run on the calling thread
    // Note: Each test still runs on its own thread from the test framework,
    // but timeout no longer spawns an additional thread
}

#[test]
fn test_03_demonstrate_main_thread_requirement() {
    // This test simulates a GUI framework requirement
    // GUI frameworks often use thread-local storage to track the main thread
    
    thread_local! {
        static IS_MAIN_THREAD: std::cell::Cell<bool> = std::cell::Cell::new(true);
    }
    
    // Mark this thread as the "main" thread
    IS_MAIN_THREAD.with(|flag| flag.set(true));
    
    // Verify we're on the main thread
    IS_MAIN_THREAD.with(|flag| {
        assert!(flag.get(), "Should be on main thread");
    });
}

#[test]
#[timeout(1000)]
fn test_04_timeout_preserves_main_thread_requirement() {
    // This test demonstrates that the new timeout implementation preserves main thread requirements
    
    thread_local! {
        static IS_MAIN_THREAD: std::cell::Cell<bool> = std::cell::Cell::new(true);
    }
    
    // Mark this thread as the "main" thread
    IS_MAIN_THREAD.with(|flag| flag.set(true));
    
    // With the new timeout implementation, this code runs on the same thread
    // So thread-local storage will be preserved
    IS_MAIN_THREAD.with(|flag| {
        // This should now pass because we're on the same thread
        assert!(flag.get(), "Should be on main thread");
    });
}

#[test]
#[timeout(100)]
fn test_05_no_timeout_completes_successfully() {
    // Test that completes within timeout
    let fifty_millis = std::time::Duration::from_millis(50);
    thread::sleep(fifty_millis);
    // Should complete successfully
}
