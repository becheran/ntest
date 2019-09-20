# NTest  Timeout

Part of the [NTest library](https://crates.io/crates/ntest). 
Do not use this lib on it's own since it depends on the [NTest library](https://crates.io/crates/ntest). 
Add the timeout attribute to the rust test framework using 
[procedural macros](https://doc.rust-lang.org/reference/procedural-macros.html).

## Examples

This example will not panic:

```rust
#[test]
#[timeout(100)]
fn no_timeout() {
    let fifty_millis = time::Duration::from_millis(50);
    thread::sleep(fifty_millis);
}
```

This example will panic. The function panics after 10 milliseconds:

```rust
#[test]
#[timeout(10)]
#[should_panic]
fn timeout() {    
	let fifty_millis = time::Duration::from_millis(50);    		
	thread::sleep(fifty_millis);}rust
}
```

For more examples and information read the [documentation](https://docs.rs/ntest_timeout/).
