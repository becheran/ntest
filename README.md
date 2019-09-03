# NTest

Testing framework for rust which enhances the built-in library with some useful features. Inspired by the *.Net* unit-testing framework 
[NUnit](https://github.com/nunit/nunit). 

- [documentation](https://docs.rs/ntest/)
- [library on crates.io](https://crates.io/crates/ntest)

## Getting Started
Some functions of *NTest* use [procedural macros](https://doc.rust-lang.org/reference/procedural-macros.html) which are stable for rust edition 2018. 
If you use the library make sure that you are using the *2018 version* of rust. Update the *Cargo.toml* file:

```toml
[package]
edition = "2018"
# ..
```

Add the *NTest library* to your developer dependencies in the *Cargo.toml* file:

```toml
[dev-dependencies]
ntest = "0.1"
```

Use the *NTest* functions you need. For example:

```rust
use test_case_derive::test_case;

#[test_case("https://doc.rust-lang.org.html")]
#[test_case("http://www.website.php")]
fn test_http_link_types(link: &str) {
    test_link(link, &LinkType::HTTP);
}
```

## Content

- `#[test_case()]` Attribute used to define multiple test cases for a test function.
- `assert_false!()` Expects false argument for test case.
- `assert_true!()` Expects true argument for test case.
- `assert_panics!()` Expects block to panic. Otherwise the test fails.

For more information read the [documentation](https://docs.rs/ntest/).

## Contribution
All contributions and comments welcome! Open an issue or create a Pull Request whenever you find a bug or have an idea to improve this crate.

## License
This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.