# Error Handling[^1]

[^1]: Source: <https://doc.rust-lang.org/book/ch09-00-error-handling.html>

To run the program:

```sh
$ cargo run --bin error-handling
   Compiling error-handling v0.1.0 ...
```

## Lessons Learned

Rust groups errors into _recoverable_ and _unrecoverable_ errors:

- For a recoverable error, e.g. _file not found error_, we want to report it to
  the user and give the program (and the user) a chance to fix and retry the
  operation.
- Unrecoverable errors are always symptoms of bugs in the code, like trying to
  access a location beyond the end of an array, and so we want to immedaitely
  stop the program.

And so, Rust has two options:

- `Result<T, E>` for recoverable errors.
- `panic!` for an unrecoverable error.

## Unrecoverable Errors with `panic!`

A _panic_ is a _"bad thing, and there's nothing you can do about"_.

By default, the program _unwinds_ during a panic, and Rust walks back up the
stack and cleans up data from each function it encounters. It's possible to
change this behavior if your binary needs to be smaller/more efficient:

```toml
# Cargo.toml

[profile.release]
panic = 'abort'
```

## Recoverable Errors with `Result`

Result is a simple two-variant (`Ok`, `Err`) enum:

```rs
enum Result<T, E> {
  Ok(T),
  Err(E),
}
```

For example, opening a file:

```rs
use std::fs::File;

fn main() {
  match File::open("hello.txt") {
    Ok(file) => { /* ... */ },
    Err(error) => panic!("Problem opening the file: {:?}", error),
  }
}
```

Or, another way:

```rs
let std:fs::File;

fn main() {
  let file = File::open("hello.txt").unwrap_or_else(|error| {
    /* ... */
  });
}
```

It's easy to _convert_ a `Err` to a _panic_:

- `unwrap`: `File::open("hello.txt").unwrap()`
- `expect`: `File::open("hello.txt").expect("Did not find expected hello.txt")`

To propagate an error, use the `?` operator, an early return for `Err` cases:

```rs
use std::fs::File;
use std::io;
use std::io::Read;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}
```

One more further change:

```rs
use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    let greeting_file = File::open("hello.txt")?;

    Ok(())
}
```

### To `panic!` or Not to `panic!`

- In prototype code and tests, just `panic!` (or use `unwrap` and `expect`).
- Where you have more knowledge than the compiler, also prefer panics:

  ```rs
  let home: IpAddr = "127.0.0.1"
      .parse()
      .expect("Hardcoded IP address should be valid");
  ```

- When a failure is expected, it's more appropriate to return a `Result`:

  - A parser being given malformed data
  - An HTTP request returning a status code that indicates hitting a rate limit

- When invalid values could put the user at risk, panic.

### Creating Custom Types for Validation

Another option is using Rust's type system to ensure we have valid values.

```rs
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}
```
