//! Part of the ntest library. Add timeout attribute to the rust test framework.

extern crate proc_macro;
extern crate syn;

use proc_macro::TokenStream;
use quote::quote;

use syn::parse_macro_input;

/// The timeout attribute can be used for tests to let them fail if they exceed a certain execution time.
/// With the `#[timeout]` attribute a timeout in milliseconds is added to a test.
///
/// The function input must be of type `int`. For example `#[timeout(10)]` will fail if the test takes longer than 10 milliseconds.
///
/// ## Behavior
///
/// The test function runs on the calling thread (preserving thread-local state and allowing
/// use of APIs that require running on the "main" thread, such as GUI frameworks). A separate
/// watcher thread monitors the test execution. If the test completes (successfully or with a panic)
/// within the timeout period, the result is returned normally. If the test exceeds the timeout,
/// the process is aborted via `std::process::abort()`.
///
/// **Important**: Tests that genuinely exceed the timeout will abort the entire test process.
/// This means tests with infinite loops or very long sleeps that exceed the timeout cannot be
/// combined with `#[should_panic]` in the traditional sense, as the abort happens before the
/// test harness can catch a panic. This is a trade-off to allow tests to run on the calling thread.
///
/// # Examples
///
/// This example will not panic
///
/// ```
/// #[test]
/// #[timeout(100)]
/// fn no_timeout() {
///     let fifty_millis = time::Duration::from_millis(50);
///     thread::sleep(fifty_millis);
/// }
/// ```
///
/// This example will abort the process after exceeding the timeout (cannot use `#[should_panic]` with actual timeouts):
///
/// ```no_run
/// #[test]
/// #[timeout(10)]
/// fn timeout_example() {
///     loop {}; // This will abort the process after 10ms
/// }
/// ```
///
/// Also works with test functions using a Result:
///
/// ```
/// #[test]
/// #[timeout(100)]
/// fn timeout_with_result() -> Result<(), String> {
///     let ten_millis = time::Duration::from_millis(10);
///     thread::sleep(ten_millis);
///     Ok(())
/// }
/// ```
#[proc_macro_attribute]
pub fn timeout(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::ItemFn);
    let time_ms = get_timeout(&parse_macro_input!(attr as syn::AttributeArgs));
    let vis = &input.vis;
    let sig = &input.sig;
    let output = &sig.output;
    let body = &input.block;
    let attrs = &input.attrs;
    check_other_attributes(&input);
    let result = quote! {
        #(#attrs)*
        #vis #sig {
            fn ntest_callback() #output
            #body
            let ntest_timeout_now = std::time::Instant::now();
            
            // Use an atomic flag to signal completion to the watcher thread
            let completed = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
            let completed_clone = completed.clone();
            
            // Spawn a watcher thread that will abort the process if timeout is exceeded
            std::thread::spawn(move || {
                std::thread::sleep(std::time::Duration::from_millis(#time_ms));
                // Check if the test has completed
                if !completed_clone.load(std::sync::atomic::Ordering::SeqCst) {
                    // Test has not completed - print error message and abort
                    eprintln!("timeout: the function call took more than {} ms", #time_ms);
                    std::process::abort();
                }
            });
            
            // Run the test on the current thread (not in a spawned thread)
            // This allows tests that require the main thread to work correctly
            let panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                ntest_callback()
            }));
            
            // Mark as completed before handling the result
            completed.store(true, std::sync::atomic::Ordering::SeqCst);
            
            // Handle the result
            match panic_result {
                std::result::Result::Ok(t) => return t,
                std::result::Result::Err(panic_payload) => {
                    // Resume the panic with the original payload to preserve panic message
                    std::panic::resume_unwind(panic_payload);
                }
            }
        }
    };
    result.into()
}

fn check_other_attributes(input: &syn::ItemFn) {
    for attribute in &input.attrs {
        let meta = attribute.parse_meta();
        match meta {
            std::result::Result::Ok(m) => match m {
                syn::Meta::Path(p) => {
                    if p.segments.iter().any(|ps| ps.ident == "timeout") {
                        panic!("Timeout attribute is only allowed once");
                    }
                }
                syn::Meta::List(ml) => {
                    if ml.path.segments.iter().any(|ps| ps.ident == "timeout") {
                        panic!("Timeout attribute is only allowed once");
                    }
                }
                syn::Meta::NameValue(nv) => {
                    if nv.path.segments.iter().any(|ps| ps.ident == "timeout") {
                        panic!("Timeout attribute is only allowed once");
                    }
                }
            },
            Err(e) => panic!("Could not determine meta data. Error {}.", e),
        }
    }
}

fn get_timeout(attribute_args: &syn::AttributeArgs) -> u64 {
    if attribute_args.len() > 1 {
        panic!("Only one integer expected. Example: #[timeout(10)]");
    }
    match &attribute_args[0] {
        syn::NestedMeta::Meta(_) => {
            panic!("Integer expected. Example: #[timeout(10)]");
        }
        syn::NestedMeta::Lit(lit) => match lit {
            syn::Lit::Int(int) => int.base10_parse::<u64>().expect("Integer expected"),
            _ => {
                panic!("Integer as timeout in ms expected. Example: #[timeout(10)]");
            }
        },
    }
}
