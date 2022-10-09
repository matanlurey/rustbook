# Generic Types, Traits, and Lifetimes[^1]

[^1]: Source: <https://doc.rust-lang.org/book/ch10-00-generics.html>

To run the program:

```sh
$ cargo run --bin generics-traits-lifetimes
   Compiling generics-traits-lifetimes v0.1.0 ...
```

## Lessons Learned

### Generic Data Types

Rust has generics, similar to other languages, to improve code reusability:

- `Option<T>`
- `Vec<T>`
- `HashMap<K, V>`
- `Result<T, T>`

Generics apply to both types and functions:

```rs
struct Point<T> {
  x: T;
  y: T;
}

fn first<T>(items: &[T]) -> &T {
  &items[0]
}
```

And method definitions:

```rs
impl<T> Point<T> {
    fn new(x: T, y: T) -> Point<T> {
        Point { x, y }
    }

    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &T {
        &self.y
    }
}
```

By adding _restrictions_ on what `T` can be, we can write `largest`:

```rs
fn largest<T: std::cmp::PartialOrd>(items: &[T]) -> &T {
  let mut largest = &items[0];

  for item in items {
      if item > largest {
          largest = item;
      }
  }

  largest
}
```

> **NOTE**: Rust uses monomorphization at compile-time, there is no _runtime_
> cost to generics.

### Traits: Defining Shared Behavior

A _trait_ defines functionlality that a type can share with other types. They
are similar to _interfaces_ in other languages, except that they are static
(compile-time):

```rs
trait Summary {
  fn summarize(&self) -> String;

  // Can be provided, but otherwise uses this fallback default implementation.
  fn promotion(&self) -> String {
      String::from("No promotions attached")
  }
}

struct Tweet {
    username: String,
    message: String,
}

/// Satisfies the Summary trait for Tweet.
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{} by {}", self.message, self.username)
    }
}

fn print_summary(summary: &impl Summary) { /* ... */ }
```

> **NOTE**: You can only declare traits on types _local to your crate_.

For generic bound traits and for _multiple_ traits:

```rs
fn notify1<T: Summary>(item1: &T, item2: &T) { /* ... */ }
fn notify2<T: Summary + Display>(item1: &T, item2: &T) { /* ... */ }
```

Can also use `where` clause to make the generics more readable:

```rs
fn notify3<T>(item1: &T, item2: &T)
where
  T: Summary + Display,
{
    /* ... */
}
```

There are some restrictions for return types, for example this doesn't work:

```rs
// Apparently this type of behavior is covered in Chapter 17.
fn returns(switch: bool) -> impl Summary {
  if switch {
    Article { ... }
  } else {
    Tweet { ... }
  }
}
```

Trait bounds can also _conditionally_ implement methods:

```rs
impl<T: Display + PartialOrd> Point<T> {
  fn cmp_display(&self) {
    if self.x >= self.y {
      println!("The largest value is x = {}", self.x);
    } else {
      println!("The largest value is y = {}", self.y);
    }
  }
}
```

Also conditionally implement a trait for any type that implements another trait:

```rs
impl<T: Display> ToString For T {
  /* ... */
}

// This is how things like this work in the SDK:
let s = 3.to_string();
```

### Validating References with Lifetimes

Every reference in Rust has a _lifetime_ (scope in which the reference is
valid). Most of the time, lifetimes are implicit and inferred (the same way that
most types are inferred). We only annotate with lifetimes when the lifetime of
references _could_ be related in a few different ways.

```rs
&i32        // a reference
&'a i32     // a reference with an explicit lifetime
&'a mut i32 // a mutable reference with an explicit lifetime
```

The main aim of lifetimes is to prevent _dangling references_, which otherwise
would cause a program to reference data other than the data it's intended to
reference:

```rs
fn main() {
  let r;

  {
    let x = 5;
    r = &x;
    //  ^^ Error: Borrowed value does not live long enough.
  }

  println!("r: {}" , r);
}
```

The _borrow checker_ tries to infer lifetimes, and determines it won't work:

```rs
/// As the `'b` lifetime is shorter than the `'a`, the project is rejected.
fn main() {
    let r;                  // ---------+-- 'a
                            //          |
    {                       //          |
        let x = 5;          // -+-- 'b  |
        r = &x;             //  |       |
    }                       // -+       |
                            //          |
    println!("r: {}" , r);  // ---------+
}
```

To fix the code:

```rs
fn main() {
    let x = 5;            // ----------+-- 'b
                          //           |
    let r = &x;           // --+-- 'a  |
                          //   |       |
    println!("r: {}", r); //   |       |
                          // --+       |
}                         // ----------+
```

For functions and structs, it's not as simple, so we use lifetime _generics_:

```rs
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

struct Excerpt<'a> {
    part: &'a str,
}
```

It's worth mentioning _some_ common inference patterns are programmed directly
into Rust's compiler, called _lifetime elison rules_. One simple example:

```rs
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
```

> **NOTE**: Pre Rust 1.0, you would have had to write the signature as:
>
> ```rs
> fn first_word<'a>(s: &'a str) -> &'a str { /* ... */ }
> ```

The rules are:

1. The compiler assigns a lifetime parameter to each reference parameter:

   ```rs
   // A function with 1 reference.
   fn foo<'a>(x: &'a i32) { /* ... */ }

   // A function with 2 references.
   fn foo<'a, 'b>(x: &'a i32, y: &'b i32) { /* ... */ }
   ```

2. If there is exactly one input lifetime parameter, assign to the output:

   ```rs
   fn foo<'a>(x: &'a i32) -> &'a i32 { /* ... */ }
   ```

3. If there are multiple input lifetime parameters, but one is `&self` or
   `&mut self` because this is a method, the lifetime of `self` is assigned to
   all output lifetime parameters:

   ```rs
   impl Foo {
       fn foo<'a, 'b>(&'a self), y: &'b i32) -> &'a 132 { /* ... */ }
   }
   ```

Another example of `&self`:

```rs
impl<'a> ImportantExcerpt<'a> {
    // No explicit lifetime annotations are required because of Rule 3.
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}
```

Finally, there is also a `'static` lifetime, or the entire program duration:

```rs
// All string literals have the 'static lifetime, but here is it explicitly:
let s: &'static str = "I have a static lifetime.";
```

And, putting it all together:

```rs
use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```
