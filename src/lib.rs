/// Expects a true expresion. Otherwise panics.
///
/// Is an alias for the [assert! macro](https://doc.rust-lang.org/std/macro.assert.html).
///
/// # Examples
///
/// ```rust
/// # use ntest::assert_true;
/// #[test]
/// fn test_assert_true() {
///    assert_true!(true);
/// }
///```
///
/// ```rust
/// #[test]
/// #[should_panic]
/// fn test_assert_false_fail() {
///     assert_false!(true);
/// }
/// ```
#[macro_export]
macro_rules! assert_true {
    ($x:expr) => ({
        assert!($x);
    });
}

/// Expects a false expresion. Otherwise panics.
///
/// # Examples
///
/// ```rust
/// # use ntest::assert_false;
/// #[test]
/// fn test_assert_false() {
///     assert_false!(false);
/// }
/// ```
///
/// ```rust
/// # use ntest::assert_false;
/// #[test]
/// #[should_panic]
/// fn test_assert_false_fails() {
///     assert_false!(true);
/// }
/// ```
#[macro_export]
macro_rules! assert_false {
    ($x:expr) => ({
        assert!(!($x));
    });
}

/// A panic in Rust is not always implemented via unwinding, but can be implemented by aborting the
/// process as well. This function only catches unwinding panics, not those that abort the process.
/// See the catch unwind [documentation](https://doc.rust-lang.org/std/panic/fn.catch_unwind.html)
/// for more information.
///
/// # Examples
///
/// ```rust
/// #[test]
/// fn assert_panic() {
///    // Here panic can happen!
///    // ...
///    assert_panics!({panic!("I am panicing")});
/// }
/// ```
///
/// ```rust
///#[test]
/// #[should_panic]
///fn assert_panics_fail() {
///    // This call should fail
///    assert_panics!({println!("I am not panicing")});
///}
///```
///
///```rust
///#[test]
///fn assert_panics_with_text() {
///    // This call should fail
///    assert_panics!({panic!("I am panicing")}, "I am panicing");
///}
/// ```
#[macro_export]
macro_rules! assert_panics {
    ($x:block) => ({
        let result = std::panic::catch_unwind(||$x);
        assert!(result.is_err());
    });
    ($x:block, $e:expr) => ({
        use std::panic;

        fn test (){
            println!("TSt");
            assert!(false);
        }

        panic::set_hook(Box::new(|panic_info| {
            if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
                println!("panic occurred: {:?}", s);
                test();
            } else {
                println!("Second parameter of assert_panics call must be of type &str.");
            }
        }));

        let result = std::panic::catch_unwind(||$x);
        //assert!(result.is_err());
        let _ = panic::take_hook();
    });
}

#[cfg(test)]
mod tests {
    #[test]
    fn assert_panics_with_text() {
        // This call should fail
        assert_panics!({panic!("I am panicing")}, "I am panicing");
    }
}