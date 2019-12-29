# NTest

[![](http://meritbadge.herokuapp.com/ntest)](https://crates.io/crates/ntest)
[![](https://badgen.net/crates/d/ntest)](https://crates.io/crates/ntest)
[![Build Status](https://gitlab.com/becheran/ntest_ci/badges/master/pipeline.svg)](https://gitlab.com/becheran/ntest_ci/pipelines)
[![](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Testing framework for rust which enhances the built-in library with some useful features. Inspired by the *.Net* unit-testing framework [NUnit](https://github.com/nunit/nunit).

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
ntest = "*"
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

- `#[timeout()]` Attribute used for timeouts in tests.
- `#[test_case()]` Attribute used to define multiple test cases for a test function.
- `assert_about_equal!()` Compare two floating point values or vectors for equality.
- `assert_false!()` Expects false argument for test case.
- `assert_true!()` Expects true argument for test case.
- `assert_panics!()` Expects block to panic. Otherwise the test fails.

For more information read the [documentation](https://docs.rs/ntest/).

## Changelog

Checkout the [changelog file](https://github.com/becheran/ntest/blob/master/CHANGELOG.md) to see the changes between different versions.

## Contribution

All contributions and comments welcome! Open an issue or create a Pull Request whenever you find a bug or have an idea to improve this crate.

## License

This project is licensed under the MIT License - see the [LICENSE](https://github.com/becheran/ntest/blob/master/LICENSE) file for details.
