# An I/O PRoject: Building a Command Line Program[^1]

[^1]: Source: <https://doc.rust-lang.org/book/ch12-00-an-io-project.html>

To run the program:

```sh
$ cargo run --bin cli-program <QUERY> <FILE_PATH>
   Compiling cli-program v0.1.0 ...
```

To run the tests:

```sh
$ cargo test --package cli-program --lib -- tests --nocapture
   running X tests
```

## Lessons Learned

This project is a recreation of the command-line tool, `grep` (**g**lobally
search a **r**egular **e**xpression and **p**rint). It takes as arguments:

- a file path
- a string

... then it reads the file, finds files that contain the string, and prints
those lines.

This project will use:

- `stderr` and `stdout`
- organizing code
- vectors and strings
- handling errors
- using traits and lifetimes
- writing tests

... and briefly introduce closures, iterators, and trait objects.
