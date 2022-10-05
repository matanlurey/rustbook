# Getting Started[^1]

[^1]: Source: <https://doc.rust-lang.org/book/ch01-00-getting-started.html>

To run the sample ("Hello World") program:

```sh
$ cargo run --bin getting-started
   Compiling getting-started v0.1.0 ...
Hello World
```

## Lessons Learned

_Because of my sus decision to include every chapter in a monorepo, I had to
spend a non-trivial amount of time configuring `[workspace]` settings and
various `Cargo.toml` files before I could run a single line of code and have
VSCode work._

Also, `println!` is a _macro_:

```rs
fn main() {
    println!("Hello World")
}
```

If you look at how this macro is implemented:

```rs
macro_rules! println {
    () => {
        $crate::print!("\n")
    };
    ($($arg:tt)*) => {{
        $crate::io::_print($crate::format_args_nl!($($arg)*));
    }};
}
```

Essentially:

- If no arguments are provided, a new line is printed.
- If one or more arguments are provided, they are formatted, and printed.
