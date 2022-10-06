# Enums and Pattern Matching[^1]

[^1]: Source: <https://doc.rust-lang.org/book/ch06-00-enums.html>

To run the program:

```sh
$ cargo run --bin enums-patterns
   Compiling enums-patterns v0.1.0 ...
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
