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
/// ```
/// # use ntest::assert_false;
/// #[test]
/// fn test_assert_false() {
///     assert_false!(false);
/// }
/// ```
///
/// ```
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