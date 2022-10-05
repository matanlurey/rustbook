# Understanding Ownership[^1]

[^1]: Source: <https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html>

To run the program:

```sh
$ cargo run --bin understanding-ownership
   Compiling understanding-ownership v0.1.0 ...
```

## Lessons learned

_Ownership_ is Rust's most unique feature, enabling memory safety without GC.

## What is Ownership?

All programs have to manage the way they use a computer's memory while running.

Different approaches:

- GC (garbage collection) that looks for no-longer used memory e.g. Dart, Java.
- Explicitly allocate and free the memory e.g. C, C++.
- Automatic reference counting e.g. Obj-C, Swift.
- System of ownership with compiler checks e.g. Rust.

### The Stack and the Heap

Stack and Heap = memory available during runtime, structured differently:

- Stack stores values in LIFO, pushing and popping _**known** fixed-size_ data.
- Heap allocates memory and returns a _pointer_ to the address of that location.
  - The _pointer_ can be stored on the stack.
- When your code calls a function, the values (and pointers) are pushed/popped.

Ownership addresses:

- Keeping track of what parts of code are using what data on the heap.
- Minimizing the amount of duplicate data on the heap.
- Cleaning up unused data on the heap.

### Ownership Rules

Basic rules:

- Each value in Rust has an _owner_.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped.

For example:

```rs
// s is not yet eclared.
{
   // s is valid within this scope.
   let s = "hello";
}
// scope is now over, s is no longer valid.
```

### The `String` Type

The types described in
["Data Types"](/chapter/03-common-concepts/README.md#data-types) are all of a
known size (i.e. `i32`), and can be stored on the stack and popped off the stack
when their scope is over, and quickly/trivially copied to use the same value in
a different scope. However, what about data stored on the heap, i.e. `String`?

We've seen string _literals_, i.e. `&str`:

```rs
// Immutable hardcoded string literals (pushed on the stack).
let s: &str = "hello";
```

However, to take input, or compute strings, we need a second type, `String`:∂

```rs
// Immutable string object (allocated on the heap).
let s = String::from("hello");

// Mutable string object (also allocated on the heap).
let mut s = String::from("hello");
s.push_str(", world!");
println!("{}", s);
```

For the latter (mutable, growable piece of text):

- The memory must be requested from the memory allocator at runtime.
- We need a way to return this memory to the allocator when `String` is done.

The first is done by us, via `String::from`.

The second part is different; memory is automatically returned once the variable
that owns it goes out of scope. Multiple variables can interact with the same
data in different ways:

```rs
// Assigns 5 to x, and copies the value to y. Both are now 5.
let x = 5;
let y = x;

// Does not make a copy, they both are _pointers_ are copied, but not the value.
let x = String::from("Hello");
let y = x;
```

For example, `x` and `y` are now both pointers that look like this:

| name     | value             |
| -------- | ----------------- |
| ptr      | `@0xFF` (Example) |
| len      | `5`               |
| capacity | `5`               |

And the `String` located `@0xFF` looks like this:

| index | value |
| ----- | ----- |
| `0`   | H     |
| `1`   | e     |
| `2`   | l     |
| `3`   | l     |
| `4`   | o     |

Rust avoids what is called a _double free_ error:

```rs
let x = String::from("Hello");
let y = x;

// ERROR: Borrow of moved value: `x`
println!("{} World", y);
```

There are a couple of different ways to deal with it:

- **Clone**: Deeply clone the _heap_ data of the `String`. **Often expensive**.

  ```rs
  let x = String::from("Hello");
  let y = x.clone();
  println!("{} World", y);
  ```

- **Copy**: (Stack Only) Shallow (trivial) copy of the _stack_ data:

  ```rs
  // The "Copy" trait allows us to do this.
  // All integer types, boolean types, floating point types, char implement it.
  // Tuples, if they only contain types that implement Copy, i.e. (i32, i32).
  let x = 5;
  let y = x;
  println!("x = {}, y = {}", x , y);
  ```

- **Move**: Passing _or_ returning a variable to/from a function:

  ```rs
  let x = String::from("Hello");

  takes_ownership(x);

  // If we tried to use "x" here, we would get a compile-time error.

  fn takes_ownership(some_string: String) {
     println!("{}", some_string);
  }
  ```

Rust can also use a value without transferring ownership, called a _reference_.

## References and Borrowing

The issue with the above (`takes_ownership`) is that we have to return the
`String` to the calling function so we can still use the `String` after the
call to `take_ownership`. Instead, we can provide a _reference_ to the `String`
value:

- A _reference_ is like a pointer in that's an address we can follow.
- _Unlike_ a pointer, a reference is guaranteed to point to a valid value.

`<Stack>`:

| name | value     |
| ---- | --------- |
| ptr  | `@0x00FF` |

`@0x00FF`:

| name     | value     |
| -------- | --------- |
| ptr      | `@0xFF00` |
| len      | `5`       |
| capacity | `5`       |

`@0xFF00`:

| index | value |
| ----- | ----- |
| `0`   | H     |
| `1`   | e     |
| `2`   | l     |
| `3`   | l     |
| `4`   | o     |

The `&x` syntax lets us create a reference that _refers_ to `x`, but does not
_own_ it.

Because it does not own it, the value it points to will not be dropped
when the reference stops being used:

```rs
// Here x goes into scope (is allocated).
let x = String::from("Hello");
let l = calculate_length(&x);

fn calculate_length(s: &String) -> usize {
   s.len();
   // Here s goes out of scope, but since it's a reference, it's not dropped.
}

// Here x goes out of scope, and it is dropped.
```

We call the action of creating a reference **borrowing**.

As in real life, you can _borrow_ something from someone else, but when you're
done, you have to give it back (you don't own it):

```rs
let x = String::from("Hello");
change(&x);

fn change(s: &String) {
   // ERROR: Cannot borrow `*some_string` as mutable
   s.push_str(" World");
}
```

To fix the code, we need a _mutable reference_:

```rs
let x = String::from("Hello");
change(&x);

fn change(s: &mut String) {
   // ERROR: Cannot borrow `*some_string` as mutable
   s.push_str(" World");
}
```

Mutable references have one big restriction: if you have a mutable reference to
a value, you can have no other references to that value.

This code that attempts to create two mutable references to `x` will fail:

```rs
let x = String::from("Hello");
let a = &mut x;
// ERROR: Cannot borrow `x` as mutable more than once at a time.
let b = &mut x;
println!("{}, {}", a, b);
```

This prevents what is called a _data race_, or:

- Two or more pointers accessing the same data at the same time.
- At least one of the pointers is being used to write to the data.
- There's no mechanism being used to synchronize across to the data.

One option is also creating a new scope:

```rs
let x = String::from("Hello");
{
   let a = &mut x;
}
let b = &mut x;
```

### Dangling References

A _dangling pointer_ references a location in memory given to someone else. In
Rust, the compiler guarantees that references will never be dangling references;
if you have a reference to some data, the compiler will ensure that the data
will not go out of scope before the reference to the data does:

```rs
fn a() {
   let a = b();
}

// ERROR: Missing lifetime specifier.
fn b() -> &String {
   let b = String::from("Hello");
   &b
}
```

To fix this, return the `String` directly:

```rs
fn a() {
   let a = b();
}

fn b() -> String {
   let b = String::from("Hello");
   b
}
```

This works! Ownership is moved out, and nothing is deallocated.

**tl;dr**:

1. References are always valid
2. At any given time, you can have _either_ one mutable reference _or_ any
   number of immutable references.

The next kind of reference is a _slice_.

## The `Slice` Type

A _slice_ _references_ a contiguous sequence of elements in a collection:

```rs
let x = String::from("Hello World");
let h = &x[/*0*/..5];
let w = &x[6../*11*/];
```

`x`:

| name     | value     |
| -------- | --------- |
| ptr      | `@0xFF00` |
| len      | `11`      |
| capacity | `11`      |

`w`:

| name | value     |
| ---- | --------- |
| ptr  | `@0xFF05` |
| len  | `5`       |

`@0xFF00`:

| index | value |
| ----- | ----- |
| `0`   | H     |
| `1`   | e     |
| `2`   | l     |
| `3`   | l     |
| `4`   | o     |
| `5`   |       |
| `6`   | W     |
| `7`   | o     |
| `8`   | r     |
| `9`   | l     |
| `10`  | d     |

Ultimately, _string literals (`&str`) are immutable **references**_!
