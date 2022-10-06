# Enums and Pattern Matching[^1]

[^1]: Source: <https://doc.rust-lang.org/book/ch06-00-enums.html>

To run the program:

```sh
$ cargo run --bin enums-patterns
   Compiling enums-patterns v0.1.0 ...
```

## Lessons learned

_As I have some prior experience with enums and pattern matching from other
languages (Ruby, OCaml, etc), the lessons learned here will be briefer than
other sections._

Defining an `enum` is easy:

```rs
enum IpAddressKind {
    V4,
    V6,
}
```

They are extra powerful when they contain semantic data:

```rs
enum IpAddress {
    V4(u8, u8, u8, u8),
    V6(String),
}
```

To retrieve values, use `match`:

```rs
match ip_address {
  V4(a, b, c, d) => println!("{a}.{b}.{c}.{d}"),
  V6(e) => println!("{e}"),
}
```

Or, use `if let` to handle a single case:

```rs
if let V4(a, b, c, d) = ip_address {
  println!("{a}.{b}.{c}.{d}");
}
```
