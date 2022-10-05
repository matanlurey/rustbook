# Using Structs to Structure Related Data[^1]

[^1]: Source: <https://doc.rust-lang.org/book/ch05-00-structs.html>

To run the program:

```sh
$ cargo run --bin using-structs
   Compiling using-structs v0.1.0 ...
```

## Lessons learned

### Defining and Instatiating Structs

Structs are very simple, basically a (nominal) set of name fields:

```rs
struct User {
    active: bool,
    email: String,
    sign_in_count: u64,
}
```

Otherwise, they work very similar to say, tuples, with some conveniences:

```rs
// Avoid typing active: active, email: email, sign_in_count: sign_in_count.
let user = User {
    active,
    email,
    sign_in_count,
};
```

```rs
// Avoid referencing every field manually when creating a new struct.
let user = User {
  sign_in_count: user.sign_in_count + 1,
  ..user
}
```

There are also _tuple_ structs:

```rs
struct Color(i32, i32, i32);
```

You can also _derive_ behavior, i.e. to use for debugging:

```rs
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
let rect = Rectangle {
    width: 30,
    height: 50,
};
dbg!(&rect);
```

### Method Syntax

Using `impl`, we can define _associated_ functions, or member-methods:

```rs
impl Rectangle {
  fn area(&self) -> u32 {
      self.width * self.height
  }

  fn square(size: u32) -> Self {
      Self {
          width: size,
          height: size,
      }
  }
}
println!("Rectangle's area is {}", rect.area());
let rect = Rectangle::square(10);
println!("Rectangle {:?}'s area is {}", rect, rect.area());
```
