# Managing Growing PRojects with Packages, Crates, and Modules[^1]

[^1]: Source: <https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html>

To run the program:

```sh
$ cargo run --bin growing-projects
   Compiling growing-projects v0.1.0 ...
```

## Lessons learned

The programs I wrote so far have been one module, in one file.

As a project grows, I organize code by:

- Splitting it into multiple modules
- And then multiple files.

A _package_ can contain multiple _binary_ cartes and optionally one _library_
crate.

As a package grows, I extract parts into separate crates that become external
dependencies.

> **NOTE**: For very large projects comprising a set of interrelated packages
> that evolve together, Cargo also provides _workspaces_, but that isn't covered
> until Chapter 14.

In general, the features that are supported to organize code include:

1. **Packages**: A Cargo feature that lets you build, test, and share crates.
2. **Crates**: A tree of modules that produces a libray or executable.
3. **Modules** and `use`: Let you control the organization, scope, privacy of
   _paths_.
4. **Paths**: A way of naming an item, such as a struct, function, or module.

### Packages and Crates

#### Crates

A _crate_ is the smallest amount of code that the Rust compiler considers at a
time.

For example if you run:

```sh
rustc main.rust
```

... the compiler considers that single file to be a crate.

_Crates_ can contain _modules_, and _modules_ may be defined in other files.

A crate comes in one of two forms: a _binary_ crate or a _library_ crate:

- _Binary crates_ are programs that are compiled and run, i.e. a CLI or server.
  - Each must have a function called `main`.
- _Library crates_ don't have a `main`, and don't compile to an executable.
  - Instead, they define functionality shared with multiple projects.

> **NOTE**: In practice, the word _crate_ often means _library crate_.

#### Packages

A _package_ is a bundle of one or more crates that provide a set of
functionality. A package contains a `Cargo.toml`[^2] file that describes how
to build those crates. Cargo is a package that contains the _binary crate_ for
the command-line tool I've been using up until this point.

> **NOTE**: Cargo also provides a _library crate_ that other packages can use!

When you create a new _package_, i.e. with `cargo new`, which creates a:

- `Cargo.toml` configuration file.
- A _crate root file_, either `src/main.rs` or `src/lib.rs`.

```sh
$ cargo new my-project
    Created binary (application) `my-project` package

$ ls my-project
Cargo.toml
src

# If cargo new my-project --lib, this file would be main.lib
$ ls my-project/src
main.rs
```

> **NOTE**: Remember that crates can only contain a single library crate.
>
> However multiple _binary_ creates are allowed by creating a `src/bin`:
>
> ```txt
> src/
>   bin/
>     a.rs
>     b.rs
> ```

[^2]:
    Why this file starts with a uppercase character, just to throw off my
    file naming schemes, you can [read about here](https://github.com/rust-lang/cargo/issues/45#issuecomment-47018233).

### Defining Modules to Control Scope and Privacy

The module system is roughly:

- _Paths_ allow you to name items
- The `use` keyword brings a path into scope
- The `pub` keyword makes items public

The typical workflow is:

- The compiler starts at the _root crate_'s _crate root_ (typically
  `src/{lib,main}.rs`).
- In the crate root, you can declare new modules. Let's say the `garden` module:

  - It will look inline (in the crate root) for:

    ```rs
    mod garden {
      // ...
    }
    ```

    _or_ (these examples are semantically the same):

    ```rs
    mod garden;

    // ...
    ```

  - If not found, it will look in `src/garden.rs`.
  - If not found, it will look in `src/garden/mod.rs`.

- In any other file (other than crate root), declare submodules, i.e.
  `vegetables`:

  - Inline, directly following `mod vegetables;` or within `mod vegetables {}`.
  - If not found, in the file `src/garden/vegetables.rs`.
  - If not found, in the ffile `src/garden/vegetables/mod.rs`.

- Once a module is part of your crate, and it's visible to you, you refer to it:

  ```rs
  // For example, an `Asparagus` type in the garden/vegetables module:
  crate::garden::vegetables::Asparagus
  ```

- Code within a module is _private_ by default.

  - To make a module public, declare it with `pub mod` instead of `mod`.
  - To make an item _within_ a public module public, use `pub`, i.e. `pub enum`.

- Using the `use` keyword to create scope shortcuts:

  ```rs
  use crate::garden::vegetables::Asparagus;

  // You can now refer directly to `Asparagus`.
  ```

```txt
backyard
├── Cargo.lock
├── Cargo.toml
└── src
    ├── garden
    │   └── vegetables.rs
    ├── garden.rs
    └── main.rs
```

To write a _library crate_ that provides functionality of a restaurant:

```rs
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}
```

```txt
crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment
```

### Paths for Referring to an Item in the Module Tree

To navigate the module tree (similar to a filesystem), we need a _path_:

- An _absolute path_ is the full path starting from a crate root

  - For an external crate, it's `{name}::...`.
  - For the current create, it's `crate::...`.

- A _relative path_ starts from the current module and uses either:

  - `self`
  - `super` (similar to `../` in a filesystem path)
  - an identifier in the current module

The preference in the Rust Book is to:

- Use absolute paths to make refactors easy.
- Import the _module_ (not the full path) to call functions:

  ```rs
  // BAD.
  use crate::front_of_house::hosting::add_to_waitlist {
    pub fn eat_at_restaurant() {
      add_to_waitlist();
    }
  }

  // GOOD.
  use crate::front_of_house::hosting {
    pub fn eat_at_restaurant() {
      hosting::add_to_waitlist();
    }
  }
  ```

- Use full names, i.e. `front_of_house.rs` over `front_of_house/mod.rs`.

- Import the full path to use structs, enums, and other items:

  ```rs
  // GOOD.
  use std::collections::HashMap;

  fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
  }
  ```

- As an _exception_, use the module name or `as` rename if names would conflict:

  ```rs
  // GOOD.
  use std::fmt;
  use std::io;

  fn f1() -> fmt::Result { /* ... */ }
  fn f2() -> io::Result<()> { /* ... */ }

  // GOOD.
  use std::fmt::Result;
  use std::io::Result as IoResult;

  fn f1() -> Result { /* ... */ }
  fn f2() -> IoResult<()> { /* ... */ }
  ```

Can also use _nested paths_ to clean-up large `use` lists:

```rs
use std::cmp::Ordering;
use std::io;

// Semantically the same:
use std::{cmp::Ordering, io};
```

Another example:

```rs
use std::io;
use std::io::Write;

// Semanticaly the same:
use std::io::{self, Write};
```

There is also a _glob_ operator, which is typically used by a `tests` module:

```rs
// See also: https://doc.rust-lang.org/std/prelude/index.html#other-preludes
use std::collections::*;
```

Can also write `pub use` to re-export code (the book will expand in chapter 14).

### Using External Packages

For example, using `rand` (<https://crates.io/crates/rand>):

```sh
cargo add rand
```

```toml
[dependencies]
rand = "0.8.5"
```

```rs
use rand::Rng;

fn main() {
  let secret_number = rand::thread_rng().gen_range(1..=100);
}
```

## Notes

Though not specifically covered by this section, some `cargo` CLI steps:

```sh
# Resolves and adds the "rand" package (from crates.io) to "growing-projects".
cargo add rand -p growing-projects
```
