# Smart Pointers[^1]

[^1]: Source: <https://doc.rust-lang.org/book/ch15-00-smart-pointers.html>

To run the program:

```sh
$ cargo run --bin smart-pointers
   Compiling smart-pointers v0.1.0 ...
```

## Lessons Learned

A _pointer_ is a variable that contains an address in memory.

In Rust, the most common kind of pointer is a _reference_, indicated by `&`
symbol and they borrow the value they point to. They don't have any special
capabilities other than referring to data, and have no overhead.

_Smart pointers_ act like a pointer, with additional metadata and capabilities:

- `Box<T>` for allocating values on the heap
- `Rc<T>`, a reference counting type that enables multiple ownership
- `Ref<T>` and `RefMut<T>`, accessed through `RefCell<T>`, a type that enforces
  borrowing rules at _runtime_ instead of compile-time.

> **TIP**: I've already used smart pointers, i.e. `String`, `Vec<T>`!.
>
> For example, `String` stores its capacity as metadata and has extra abilities
> to ensure its data will always be valid UTF-8.

Smart pointers are usually implemented using structs, implementing these traits:

- `Deref`: Allows an instance to behave like a reference.
- `Drop`: Customize the code that's run when the pointer goes out of scope.

### Using `Box<T>` to Point to Data on the Heap

Most straightforward smart pointer is a _box_, a stack pointer to heap data.

Boxes don't have a performance overhead (other than using the heap), but they
also don't have very many capabilities. Most of the time a box is used when:

- You have a type whose size can't be known at compile-time and you want to use
  a value of that type in a context that requires an exact size.
- You have a large amount of data you want to transfer ownership but want to
  ensure the data won't be copied when you do so.
- You want to own a value and you only care that it's a type that implements a
  particular trait rather than being of a specific type.

#### Using a `Box<T>` to Store Data on the Heap

```rs

```
