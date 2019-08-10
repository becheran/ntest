/// Expects a true expresion. Otherwise panics.
///
/// Is an alias for the [assert! macro](https://doc.rust-lang.org/std/macro.assert.html).
///
/// # Examples
///
/// This call won't panic.
/// ```rust
/// # use ntest::assert_true;
/// # fn main() {
/// assert_true!(true);
/// # }
///```
///
/// This call will panic.
/// ```should_panic
/// # use ntest::assert_true;
/// # fn main() {
/// assert_true!(false);
/// # }
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
/// This call won't panic.
/// ```rust
/// # use ntest::assert_false;
/// # fn main() {
/// assert_false!(false);
/// # }
/// ```
///
/// This call will panic.
/// ```should_panic
/// # use ntest::assert_false;
/// # fn main() {
/// assert_false!(true);
/// # }
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
/// This call won't panic.
/// ```rust
/// # use ntest::assert_panics;
/// # fn main() {
/// // Other panics can happen before this call.
/// assert_panics!({panic!("I am panicing")});
/// # }
/// ```
///
/// This call will panic.
/// ```should_panic
/// # use ntest::assert_panics;
/// # fn main() {
/// assert_panics!({println!("I am not panicing")});
/// # }
/// ```
#[macro_export]
macro_rules! assert_panics {
    ($x:block) => ({
        let result = std::panic::catch_unwind(||$x);
        assert!(result.is_err());
    });
}