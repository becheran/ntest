//! The ntest lib enhances the rust test framework with useful functions and macros

// Reexport procedural macros
extern crate ntest_test_cases;
#[doc(inline)]
pub use ntest_test_cases::test_case;

/// Expects a true expression. Otherwise panics.
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
        if !$x {
            panic!("assertion failed: Expected 'true', but was 'false'");
        }
    });
    ($x:expr,) => ({
        assert_true!($x);
    });
}

/// Expects a false expression. Otherwise panics.
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
        if $x {
            panic!("assertion failed: Expected 'false', but was 'true'");
        }
    });
    ($x:expr,) => ({
        assert_false!($x);
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
        if !result.is_err(){
            panic!("assertion failed: code in block did not panic");
        }
    });
    ($x:block,) => ({
        assert_panics!($x);
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn assert_true() {
        assert_true!(true);
    }
    
    #[test]
    #[should_panic]
    fn assert_true_fails() {
        assert_true!(false);
    }
    
    #[test]
    fn assert_true_trailing_comma() {
        assert_true!(true,);
    }
    
    #[test]
    fn assert_false() {
        assert_false!(false);
    }
    
    #[test]
    #[should_panic]
    fn assert_false_fails() {
        assert_false!(true);
    }
    
    #[test]
    fn assert_false_trailing_comma() {
        assert_false!(false,);
    }
    
    #[test]
    fn assert_panics() {
        assert_panics!({panic!("I am panicing!")},);
    }
    
    #[test]
    #[should_panic]
    fn assert_panics_fails() {
        assert_panics!({println!("I am not panicing!")},);
    }
    
    #[test]
    fn assert_panics_trailing_comma() {
        assert_panics!({panic!("I am panicing!")},);
    }
}
