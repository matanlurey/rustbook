# Functional Language Features: Iterators and Closures[^1]

[^1]: Source: <https://doc.rust-lang.org/book/ch13-00-functional-features.html>

To run the program:

```sh
$ cargo run --bin functional-features
   Compiling cli-program v0.1.0 ...
```

## Lessons Learned

This section covered elements of _functional programming_ in Rust, including:

- _Closures_, a function-like construct you can store in a variable
- _Iterators_, a way of processing a series of elements

### Capturing the Environment with Closures

```rs
/// One example, creating a closure that in turn calls self.most_stocked.
fn give_away(&self, preference: Option<Color>) -> Color {
   preference.unwrap_or_else(|| self.most_stocked())
}
```

Compare closures to functions:

```rs
fn  add_one_v1   (x: u32) -> u32 { x + 1 }
let add_one_v2 = |x: u32| -> u32 { x + 1 };
```

And borrowing / borrowing for mutation:

```rs
let only_borrows = || println!("From closure: {:?}", list);
let mut borrows_mutably = || list.push(7);
```

To define what happens to captured variables, we use `Fn` traits:

- `FnOnce`: Moves captured variables out of the body only implement this trait.
  - All closures implement at least this trait.
- `FnMut`: Closures that don't move captured values, but might mutate captures.
  - Can be called more than once.
- `Fn`: Don't move captured values, and don't mutate captured values.
  - Can be called more than once.

### Processing a Series of Items with Iterators

In Rust, iterators are _lazy_:

```rs
// No effect! In fact, this will trigger a warning "unused_must_use".
vec![1, 2, 3].iter();
```

To combine with closures, for example:

```rs
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}
```

### Improving Our I/O Project

Knowing what I know now, the following improvements could be made to chapter 12:

#### Remove a `clone` Using an Iterator

```diff
- let query = args[1].clone();
- let path = args[2].clone();
+ struct Config {
+   query: String,
+   path: String,
+ }
+
+ impl Config {
+   fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
+     args.next();
+
+     let query = match args.next() {
+       Some(arg) => arg,
+       None      => return Err("Didn't get a query string")
+     };
+
+     // ...
+
+     Ok(Config {query, path})
+   }
+ }
```

#### Making Code Clearer with Iterator Adapters

```diff
fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
-    let mut results = Vec::new();
-
-    for line in contents.lines() {
-        if line.contains(query) {
-            results.push(line);
-        }
-    }
-
-    results
+    contents.lines().filter(|line| line.contains(query)).collect()
}
```

### Comparing Performance: Loops vs. Iterators

Apparently, iterators are a _zero-cost abstraction_ in Rust, or ~0 overhead.

> **NOTE**: See <https://doc.rust-lang.org/book/ch13-04-performance.html>.
