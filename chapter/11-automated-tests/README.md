# Writing Automated Tests[^1]

[^1]: Source: <https://doc.rust-lang.org/book/ch11-00-testing.html>

To run the tests:

```sh
$ cargo test --package automated-tests --lib -- tests --nocapture
   running 1 test
```

## Lessons Learned

As this chapter is mostly code intensive, I'll leave that to [`lib.rs`][].

[`lib.rs`]: ./src/lib.rs

### Controlling How Tests Are Run

To avoid running tests in parallel, or to control the number of threads:

```sh
cargo test -- --test-threads=1
```

To show function output (i.e. `println!`'s):

```sh
cargo test -- --show-output
```

To run a single test by name:

```sh
cargo test smaller_cannot_hold_larger
```

To run a subset of tests by name:

```sh
cargo test smaller_
```

To ignore some tests unless specifically requested:

```rs
#[test]
#[ignore]
fn expensive_test() {}
```

```sh
# Runs only ignored tests.
cargo test -- --ignored

# Runs all tests, including ignored.
cargo test -- --include-ignored
```

### Test Organization

Unit tests are written in the `src` directly with the code they are testing:

```rs
#[cfg(tests)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {}
}
```

Integration tests are written in a directory _next_ to `src,`, `tests`:

```txt
11-automated-tests
├── Cargo.lock
├── Cargo.toml
├── src
│   └── lib.rs
└── tests
    └── integration_test.rs
```

```sh
cargo test --test integration_test
```

As you add more integration tests, you'll want to create more files in `tests/`:

```txt
├── Cargo.lock
├── Cargo.toml
├── src
│   └── lib.rs
└── tests
    ├── common
    │   └── mod.rs
    └── integration_test.rs
```
