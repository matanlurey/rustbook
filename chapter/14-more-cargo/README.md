# More about Cargo and Crates.io[^1]

[^1]: Source: <https://doc.rust-lang.org/book/ch14-00-more-about-cargo.html>

> **Note**: There is no program included in this chapter, as it's notes only.

## Customizing Builds with Release Profiles

Cargo has two main profiles, `dev` and `release`

- `dev`: `cargo build`; good defaults for development.
- `release`: `cargo build --release`; good defaults for release builds.

However, it's possible to customize them:

```toml
# Cargo.toml
# These are the default values, as this is just an example.

[profile.dev]
# Compile faster, even if the code runs slower.
opt-level = 0

[profile.release]
# Compile slower, in order to make the code run faster.
opt-level = 3
```

See also: <https://doc.rust-lang.org/cargo/reference/profiles.html>.

## Publishing a Crate to Crates.io

The site [crates.io][] is used to publish and pull (mostly) open source code.

[crates.io]: https://crates.io

### Making Useful Documentation Comments

Using `///` is a _documentation comment_, to generate HTML documentation:

````rs
/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}
````

`cargo doc --open`: will open the HTML for your current crates' documentation.

Other commonly used sections, other than `# Examples`, are:

- **Panics**: Which scenarios the function documented could panic.
- **Errors**: If the function returns a `Result`, what kinds of errors might
  occur and what conditions might cause those errors to be returned.
- **Safety**: If the function is `unsafe`, why the function is unsafe and
  covering hte invariants that the function expects callers to uphold.

One nice aspect is you can use `cargo test` to run code examples as tests!

> **NOTE**: That's why many code snippets have asserts in the examples.

To describe the _crate_, use `//!` syntax:

```rs
//! # My Crate
//!
//! `my_crate` is a collection of utilities.

/// Adds one to the number given.
pub fn add_one(x: i32) -> i32 { /* ... */ }
```

### Exporting a Convenient Public API with `pub use`

In order to avoid having to add direct dependencies on other crates in order
to use a crate, `pub use` will both `use` (import) and _re-export_ the paths:

```rs
//! # Art
//!
//! A library for modeling artistic concepts.

pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

pub mod kinds {
    // --snip--
}

pub mod utils {
    // --snip--
}
```

### Setting Up a Crates.io Account

You create an account at [crates.io][] with a GitHub account, and then login:

```sh
# API key comes from https://crates.io/me/
# Will authenticate and store a SECRET token in in ~/.cargo/credentials
cargo login abcdefghijklmnopqrstuvwxyz012345
```

### Adding Metadata to a New Crate

More metadata is required to _publish_ a crate:

```toml
# Cargo.toml

[package]
name = "guessing_game"
version = "0.1.0"
edition = "2021"
description = "A fun game where you guess what number the computer has chosen."
license = "MIT OR Apache-2.0"
```

See also: <https://doc.rust-lang.org/cargo/>

### Publishing to Crates.io

tl;dr: `cargo publish`.

### Deprecating Versions from Crates.io with `cargo yank`

tl;dr: `cargo yank --vers 1.0.1`.

> **NOTE**: This doesn't remove it, but will refuse being added to new projects.

Or, to undo: `cargo yank --vers 1.0.1 --undo`.

## Cargo Worksapce

A _workspace_ is a set of packages that share the same _Cargo.lock_ and outputs.

> **NOTE**: This repository itself is a workspace containing all book chapters!

```yaml
# /Cargo.toml

[workspace]

members = [
  "adder",
  "add_one",
]
```

```txt
├── Cargo.lock
├── Cargo.toml
├── add_one
│   ├── Cargo.toml
│   └── src
│       └── lib.rs
├── adder
│   ├── Cargo.toml
│   └── src
│       └── main.rs
└── target
```

```rs
//! adder/src/main.rs
use add_one;

fn main() {
    let num = 10;
    println!("Hello, world! {num} plus one is {}!", add_one::add_one(num));
}
```

## Installing Binaries with `cargo install`

tl;dr: Use binary crates locally (i.e. install tools from [crates.io][]).

```sh
$ cargo install ripgrep
    Updating crates.io index
  Downloaded ripgrep v11.0.2
  Downloaded 1 crate (243.3 KB) in 0.88s
  Installing ripgrep v11.0.2
--snip--
   Compiling ripgrep v11.0.2
    Finished release [optimized + debuginfo] target(s) in 3m 10s
  Installing ~/.cargo/bin/rg
   Installed package `ripgrep v11.0.2` (executable `rg`)
```

## Extending Cargo with Custom Commands

Cargo is designed so you can extend it without having to modify Cargo.

If a binary in your `$PATH` is `cargo-something`, run using `cargo something`.

> **TIP**: List custom commands by running `cargo --list`.
>
> You can even use `cargo install` to install extensions and run them like this!
