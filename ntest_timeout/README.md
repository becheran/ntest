# NTest Timeout

Part of the [NTest library](https://crates.io/crates/ntest). Add the timeout attribute to the rust test framework using
[procedural macros](https://doc.rust-lang.org/reference/procedural-macros.html).

Use `#[timeout(milliseconds)]` to panic when a test exceeds its timeout. To
terminate the process when a timeout occurs, opt in with `abort`:

```rust
#[test]
#[ntest::timeout(2000, abort)]
fn test_using_a_shared_resource() {
 // A timed-out test cannot continue using the resource after the process aborts.
}
```

The default `#[timeout(milliseconds)]` behavior is unchanged. The `abort`
option is intended for tests where leaving the worker thread running after a
timeout could corrupt shared state or consume resources indefinitely.
