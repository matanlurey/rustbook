# Patterns and Matching[^1]

[^1]: Source: <https://doc.rust-lang.org/book/ch18-00-patterns.html>

> **Note**: There is no program included in this chapter, as it's notes only.

_Patterns_ are a special syntax in Rust for matching the structure of types:

- Literals
- Destructured arrays, enums, structs, or tuples
- Variables
- Wildcards
- Placeholders

> **TIP**: `x`, `(a, 3)`, and `Some(Color::Red)` are all example patterns.

## All the Places Patterns Can Be Used

The most common is `match`:

```rs
match x {
    None => None,
    Some(i) => Some(i + 1),
}
```

The next is a conditional `if let` expression:

```rs
// Similar to above, but the compiler will not check for exhaustiveness.
if let Some(i) = x {
  // ...
}
```

And `while let` and `for`:

```rs
while let Some(top) = stack.pop() {
  // ...
}
```

```rs
for (index, value) in v.iter().enumerate() {
  // ...
}
```

Function parameters can _also_ be patterns:

```rs
fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}

fn main() {
    let point = (3, 5);
    print_coordinates(&point);
}
```

See also: <https://doc.rust-lang.org/book/ch18-03-pattern-syntax.html>.
