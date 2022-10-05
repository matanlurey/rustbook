# Rust Book

This is a personally annotated copy of the "The Rust Programming Language"[^1].

<!--
  Fun Emoji I will use in this doc:

  ⚪ Draft
  🟢 Complete
  🟡 In-progress
  🔴 Blocked
-->

[^1]: Available at <https://doc.rust-lang.org/book/>, or `rustup docs --book`.

## Why Rust

For me, I've never really been exposed to low-level systems programming, having
majored in [Informatics](https://www.informatics.uci.edu/) at UC Irvine
precisely to avoid it (and well, math). My first language was Visual Basic 6,
and even though I've grown to love strong, static langauges, I've never really
used C, or anything like it, for anything substantial.

I'd love to Rust to learn more about:

- Lower-level system details, such as memory usage
- Using non-traditional languages
- Mastering the borrow checker

Rust, is well, also pretty cool:

![Rust gaining popularity from 2017 -> 2022](include/google-trends-2022.png)

## This repository

Every chapter in the book receives its own folder, i.e.
[`chapter/01-getting-started`](chapter/01-getting-started/README.md):

1. 🟢 [Getting Started](chapter/01-getting-started/README.md)
2. 🟢 [Programming a Guessing Game](chapter/02-guessing-game/README.md)
3. ⚪ Common Programming Concepts

Unless otherwise specified, the only tools _required_ to be installed are:

- An IDE ([VSCode][] with [`rust-analyzer`][] preferred, but any will do).
- Rustup and Cargo, often through the [official website][install-rust].
- A [linker][], such as `xcode-select`, `Clang`, or `MSVC`.

[vscode]: https://code.visualstudio.com
[`rust-analyzer`]: https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer
[install-rust]: https://www.rust-lang.org/tools/install
[linker]: https://doc.rust-lang.org/book/ch01-01-installation.html#installing-rustup-on-linux-or-macos

When possible, copies of other source material, such as PDFs or images, should
be cached in this repository so that the only thing required to interact with
this repository is an installation of Rust, an IDE, a linker, and a clone.

To verify, run `./CHECK.sh`.

### Creating a new chapter

```sh
cargo new chapter/00-chapter-name --name chapter-name --vcs none
```

Then, edit [Cargo.toml](./Cargo.toml) and add the new workspace member:

```toml
members = [
  # ...
  "chapter/00-chapter-name",
]
```

### Adding dependencies

```sh
cargo add package-name -p chapter-name
```