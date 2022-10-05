# Programming a Guessing Game[^1]

[^1]: Source: <https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html>

To run the program:

```sh
$ cargo run --bin guessing-game
   Compiling guessing-game v0.1.0 ...
```

## Lessons learned

- Sometimes (`extern crate rand`) you just need [stack overflow](https://stackoverflow.com/questions/30735490/unresolved-name-randthread-rng).
- A `String` is both an immutable _and_ mutable buffer of characters.
- You create a new mutable variable with `let mut`:

  ```rs
  let mut guess = String::new();
  ```

- And read into it as a reference, i.e. `&mut guess`:

  ```rs
  io::stdin().read_line(&mut guess);
  ```

- Many Rust APIs return enums, such as `Result`. You can use `.expect` to panic:

  ```rs
  result.expect("Failed to read line");
  ```

- Or (pattern) `match` against the result, such as:

  ```rs
  let guess:u32 = match guess.parse() {
    Ok(num) => num,
    Err(_) => {
      println!("Invalid!");
      continue;
    }
  }
  ```
