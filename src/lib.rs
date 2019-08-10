extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro_rules::rules;
extern crate syn;
use quote::quote;

/// Expects a true expresion. Otherwise panics.
///
/// Is an alias for the [assert! macro](https://doc.rust-lang.org/std/macro.assert.html).
///
/// # Examples
///
/// ```rust
/// use ntest::assert_true;
/// assert_true!(true);
///```
///
/// ```rust
/// use ntest::assert_true;
/// fn main() {
///     assert_true!(false);
/// }
/// ```
#[proc_macro]
pub fn assert_true(input: TokenStream) -> TokenStream {
    rules!(input.into() => {
        ($x:expr) => { quote! {
            assert!(#x);
        }}
    }).into()
}
/*
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
/// fn main() {
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
/// ```
///#[test]
/// #[should_panic]
///fn test_assert_panics_fail() {
///    // This call should fail
///    assert_panics!({println!("I am not panicing")});
///}
/// ```
#[macro_export]
macro_rules! assert_panics {
    ($x:block) => ({
        let result = std::panic::catch_unwind(||$x);
        assert!(result.is_err());
    });
}
*/