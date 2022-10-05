# Common Programming Concepts[^1]

[^1]: Source: <https://doc.rust-lang.org/book/ch03-00-common-programming-concepts.html>

To run the program:

```sh
$ cargo run --bin common-concepts
   Compiling common-concepts v0.1.0 ...
```

## Lessons learned

In this chapter, we reviewed:

1. [Variables and Mutability](#variables-and-mutability)
2. [Data Types](#data-types)
3. [Functions](#functions)
4. [Comments](#comments)
5. [Control Flow](#control-flow)

Additionally, _keywords_ reserved in Rust:

| name       | purpose                                                        |
| ---------- | -------------------------------------------------------------- |
| `as`       | perform primitive casting, disabugiate traits, rename in `use` |
| `async`    | returns a `Future` instead of blocking the current thread      |
| `await`    | suspend execution until the result of a `Future` is ready      |
| `break`    | exit a loop immediately                                        |
| `const`    | define constant items or constant raw pointers                 |
| `continue` | continue to the next loop iteration                            |
| `crate`    | in a module path, refers to the crate root                     |
| `dyn`      | dynamic dispatch to a trait object                             |
| `else`     | fallback for `if` and `if let` control flow constructs         |
| `enum`     | define an enumeration                                          |
| `extern`   | link an external function or variable                          |
| `false`    | boolean false literal                                          |
| `fn`       | define a function or the function pointer type                 |
| `for`      | loop over iterator, implement a trait, specify a lifetime      |
| `if`       | branch based on the result of a conditional expression         |
| `impl`     | implement inherent or trait functionality                      |
| `in`       | part of `for` loop syntax                                      |
| `let`      | bind a variable                                                |
| `loop`     | loop unconditionally                                           |
| `match`    | match a value to patterns                                      |
| `mod`      | define a module                                                |
| `move`     | make a closure take ownership of all captures                  |
| `mut`      | denote mutability in references, raw pointers, or patterns     |
| `pub`      | denote public visibility in fields, blocks, or modules         |
| `ref`      | bind by reference                                              |
| `return`   | return from a function                                         |
| `Self`     | a type alias for the type we are defining or implementing      |
| `self`     | method subject or current module                               |
| `static`   | global variable or lifetime lasting the program execution      |
| `struct`   | define a structure                                             |
| `super`    | parent module of the current module                            |
| `trait`    | define a trait                                                 |
| `true`     | boolean true literal                                           |
| `type`     | define a type alias or associated type                         |
| `union`    | define a union                                                 |
| `unsafe`   | denote unsafe code, functions, traits, or implementations      |
| `use`      | bring symbols into scope                                       |
| `where`    | denote causes that constrain a type                            |
| `while`    | loop unconditionally based on the result of an expression      |

### Variables and Mutability

- Constants are compile-time evaluated[^2]
- Unlike `let` assignments, `const` cannot use type inference:

  ```rs
  // OK
  let x = 60 * 60 * 3;

  // ERROR
  const x = 60 * 60 * 3;

  // OK
  const x: u32 = 60 * 60 * 3;
  ```

- Shadowing is powerful, by either creating explicit new scopes:

  ```rs
  let x = 5;

  {
    let x = x * 2;
  }
  ```

- Or even reusing a particulary good name:

  ```rs
  let spaces = "   ";
  let spaces = spaces.len();
  ```

[^2]: https://doc.rust-lang.org/reference/const_eval.html

### Data Types

A _scalar_ represents a single value, of which Rust has four primary types:

- integers
- floating-point numbers
- booleans
- characters

#### Integer

An _integer_ is a number without a fractional component:

| Length     | [Signed ][^3] | Unsigned |
| ---------- | ------------- | -------- |
| 8-bit      | `i8`          | `u8`     |
| 16-bit     | `i16`         | `u16`    |
| 32-bit     | `i32`         | `u32`    |
| 64-bit     | `i64`         | `u64`    |
| 128-bit    | `i128`        | `u128`   |
| [arch][^4] | `isize`       | `usize`  |

[^3]:
    Whether it is possible to be negative, `-(2n - 1) to 2n - 1 - 1`. See
    [two's complement](https://en.wikipedia.org/wiki/Two%27s_complement).

[^4]:
    Depend on the architecture of the computer your program is running on;
    i.e. 64-bit if on 64-bit architecture, and 32-bit if you are on 32-bit
    architecture. The primary situation you'd use one of these types is when
    indexing some sort of collection.

Integer values can be written in any of the following forms:

| Number literals  | Example       |
| ---------------- | ------------- |
| Decimal          | `98_222`      |
| Hex              | `0xff`        |
| Octal            | `0o77`        |
| Binary           | `ob1111_0000` |
| Byte (`u8` only) | `b'A'`        |

Rust's defaults generally start with `i32`.

> **Integer Overflow**
>
> - In `--debug` mode, panics occur when integer overflow would occur.
> - In `--release` mode, _two's complement wrapping_ occurs.
>
> To explicitly handle overflow, there are several primitive numeric methods:
>
> - Wrap all modes with `wrapping_*`, such as `wrapping_add`
> - Return the `None` value if there is overflow with the `checked_*` methods
> - Return the value and a boolean indicating overflow with `overflowing_*`
> - Saturating at the value's minimum/maximum with `saturating_*`

#### Floating-Point Types

Rust has `f32` and `f64` respectively, with the default being `f64`.

All floating-point types are signed.

#### Boolean Type

I.e. either `true` or `false`.

#### Character Type

Rust's `char` (`u4`) type is the language's most primitive alphabetic type:

```rs
// Note single quotes (char literals) versus double quotes (string literals).
let c = 'z';
let c = '😻';
```

#### Tuples

Grouping together a number of values into a single, fixed-length compound type:

```rs
let tup = (500, 6.4, 1);
```

We mostly use _destructuring_ to read from a tuple:

```rs
let (x, y, z) = tup;

// Also works
let x = tup.0;
let y = tup.1;
let z = tup.2;
```

#### Arrays

Collection of multiple values of the same type, also with a fixed-length:

```rs
let a = [1, 2, 3];
```

We can also use _destructuring_:

```rs
let [x, y, z] = a;

// Also works
let x = x[0];
let y = y[1];
let z = y[2];
```

Unlike, say, `C`, reading out of bounds causes a panic:

```rs
let a = [1, 2, 3];
let a = a[3];
```

### Functions

Return expressions (i.e. without a `;`) are implicit:

```rs
fn five() -> i32 {
  5
}

fn plus_one(x: i32) -> i32 {
  x + 1
}
```

Nothing else too spicy, though being able to write expression bodies is nice:

```rs
let y = {
  let x = 3;
  x + 1
}
```

### Comments

Most comments will begin with two slashes, i.e. `// I'm feeling lucky`.

### Control Flow

#### `if`

Straight-forward, anything than a simple `if` or `else if` would use `match`.

However, `if` is also an expression:

```rs

```

#### `loop`

Mostly either `loop { ... }`, `while <cond> { ... }`, or `for x in a { ... }`.
