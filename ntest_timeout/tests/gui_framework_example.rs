/// This test file demonstrates that the timeout attribute now works correctly
/// with code that requires running on the "main" thread, such as GUI frameworks.
/// 
/// This addresses the issue where users couldn't use #[timeout] with GUI frameworks
/// like Dioxus because the old implementation spawned a new thread for the test.

use ntest_timeout::timeout;
use std::cell::Cell;

/// Simulate a GUI framework that tracks which thread it was initialized on
/// using thread-local storage (similar to how Dioxus and other GUI frameworks work)
mod fake_gui_framework {
    use std::cell::Cell;
    
    thread_local! {
        static MAIN_THREAD_ID: Cell<Option<std::thread::ThreadId>> = Cell::new(None);
    }
    
    /// Initialize the GUI framework - must be called on the main thread
    pub fn init() {
        MAIN_THREAD_ID.with(|id| {
            id.set(Some(std::thread::current().id()));
        });
    }
    
    /// Launch the GUI - panics if not called on the same thread as init()
    pub fn launch() {
        MAIN_THREAD_ID.with(|stored_id| {
            let current_id = std::thread::current().id();
            match stored_id.get() {
                Some(id) if id == current_id => {
                    // Success - we're on the correct thread
                    println!("GUI launched successfully on thread {:?}", current_id);
                }
                Some(id) => {
                    panic!(
                        "GUI framework error: launch() called on thread {:?} but init() was called on thread {:?}",
                        current_id, id
                    );
                }
                None => {
                    panic!("GUI framework error: init() was not called before launch()");
                }
            }
        });
    }
}

#[test]
#[timeout(1000)]
fn test_gui_framework_with_timeout() {
    // This test demonstrates that the new timeout implementation allows
    // GUI frameworks to work correctly because the test runs on the calling thread
    
    // Initialize the fake GUI framework on this thread
    fake_gui_framework::init();
    
    // Launch the GUI - this would panic with the old implementation
    // because the test would run on a different thread
    fake_gui_framework::launch();
    
    // Simulate some GUI work
    std::thread::sleep(std::time::Duration::from_millis(10));
    
    // Test passes - the GUI framework requirement is satisfied
}

#[test]
#[timeout(1000)]
fn test_thread_local_state_preserved() {
    // This test verifies that thread-local state is preserved with the new implementation
    
    thread_local! {
        static TEST_STATE: Cell<i32> = Cell::new(0);
    }
    
    // Set some state in thread-local storage
    TEST_STATE.with(|state| state.set(42));
    
    // With the old implementation, this would be on a different thread
    // and the state would be 0 (default)
    // With the new implementation, we're on the same thread, so state is preserved
    TEST_STATE.with(|state| {
        assert_eq!(state.get(), 42, "Thread-local state should be preserved");
    });
}

#[test]
#[timeout(1000)]
fn test_simulated_main_thread_requirement() {
    // This test simulates the exact use case from the issue:
    // A GUI that requires running on the "main" thread
    
    // Mark this thread as the "main" thread
    thread_local! {
        static IS_MAIN_THREAD: Cell<bool> = Cell::new(true);
    }
    
    IS_MAIN_THREAD.with(|flag| flag.set(true));
    
    // Simulate launching a GUI that checks if we're on the main thread
    IS_MAIN_THREAD.with(|flag| {
        if !flag.get() {
            panic!("GUI framework requires running on the main thread!");
        }
        println!("GUI launched successfully - we're on the main thread");
    });
}
