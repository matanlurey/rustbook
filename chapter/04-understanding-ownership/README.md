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
| `0`   | h     |
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

## The `Slice` Type
